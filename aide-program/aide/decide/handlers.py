"""HTTP 请求处理：静态资源与 API。"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Callable
from urllib.parse import urlparse

from aide.decide.errors import DecideError
from aide.decide.storage import DecideStorage
from aide.decide.types import DecideInput, DecideOutput

Response = tuple[int, dict[str, str], bytes]


class DecideHandlers:
    """处理 HTTP 请求，返回 (状态码, 头, 响应体)。"""

    def __init__(self, storage: DecideStorage, web_dir: Path, stop_callback: Callable[[str], None]):
        self.storage = storage
        self.web_dir = web_dir
        self.stop_callback = stop_callback

    def handle(self, method: str, path: str, body: bytes) -> Response:
        parsed = urlparse(path)
        route = parsed.path

        if method == "OPTIONS":
            return 200, self._cors_headers({"Content-Type": "text/plain"}), b""

        if method == "GET":
            if route in {"/", "/index.html"}:
                return self.handle_index()
            if route == "/style.css":
                return self.handle_static("style.css", "text/css; charset=utf-8")
            if route == "/app.js":
                return self.handle_static("app.js", "application/javascript; charset=utf-8")
            if route == "/api/items":
                return self.handle_get_items()
            return self._not_found()

        if method == "POST" and route == "/api/submit":
            return self.handle_submit(body)

        if route in {"/api/items", "/api/submit"}:
            return self._method_not_allowed()
        return self._not_found()

    def handle_index(self) -> Response:
        return self._read_file("index.html", "text/html; charset=utf-8")

    def handle_static(self, filename: str, content_type: str) -> Response:
        safe_name = Path(filename).name
        if safe_name != filename:
            return self._not_found()
        return self._read_file(safe_name, content_type)

    def handle_get_items(self) -> Response:
        try:
            pending = self.storage.load_pending()
        except DecideError as exc:
            return self._server_error("无法读取待定项数据", str(exc))

        if pending is None:
            return self._server_error("无法读取待定项数据", "文件不存在或格式错误")

        body = json.dumps(pending.to_dict(include_meta=False), ensure_ascii=False).encode("utf-8")
        headers = self._cors_headers({"Content-Type": "application/json; charset=utf-8"})
        return 200, headers, body

    def handle_submit(self, body: bytes) -> Response:
        try:
            pending = self.storage.load_pending()
        except DecideError as exc:
            return self._server_error("保存失败", str(exc))

        if pending is None:
            return self._server_error("保存失败", "未找到待定项数据")

        try:
            payload = json.loads(body.decode("utf-8"))
            output = DecideOutput.from_dict(payload)
            self._validate_decisions(pending, output)
        except DecideError as exc:
            return self._bad_request("决策数据无效", str(exc))
        except Exception as exc:
            return self._bad_request("决策数据无效", str(exc))

        try:
            self.storage.save_result(output)
        except DecideError as exc:
            return self._server_error("保存失败", str(exc))

        # 保存成功后触发关闭
        self.stop_callback("completed")
        headers = self._cors_headers({"Content-Type": "application/json; charset=utf-8"})
        body = json.dumps({"success": True, "message": "决策已保存"}, ensure_ascii=False).encode("utf-8")
        return 200, headers, body

    def _validate_decisions(self, pending: DecideInput, output: DecideOutput) -> None:
        items_by_id = {item.id: item for item in pending.items}
        seen: set[int] = set()
        for decision in output.decisions:
            if decision.id in seen:
                raise DecideError(f"待定项 {decision.id} 的决策重复")
            seen.add(decision.id)
            item = items_by_id.get(decision.id)
            if item is None:
                raise DecideError(f"存在未知的待定项 {decision.id}")
            option_values = {opt.value for opt in item.options}
            if decision.chosen not in option_values:
                raise DecideError(f"待定项 {decision.id} 的决策值无效: {decision.chosen}")

        missing = [str(item_id) for item_id in items_by_id.keys() if item_id not in seen]
        if missing:
            raise DecideError(f"缺少待定项 {', '.join(missing)} 的决策")

    def _read_file(self, filename: str, content_type: str) -> Response:
        path = self.web_dir / filename
        if not path.exists():
            return self._server_error("读取静态资源失败", f"{filename} 不存在")
        try:
            data = path.read_bytes()
        except Exception as exc:
            return self._server_error("读取静态资源失败", str(exc))
        headers = self._cors_headers({"Content-Type": content_type})
        return 200, headers, data

    def _not_found(self) -> Response:
        headers = self._cors_headers({"Content-Type": "text/plain; charset=utf-8"})
        return 404, headers, b"Not Found"

    def _method_not_allowed(self) -> Response:
        headers = self._cors_headers({"Content-Type": "text/plain; charset=utf-8"})
        return 405, headers, b"Method Not Allowed"

    def _bad_request(self, message: str, detail: str) -> Response:
        payload = json.dumps({"error": message, "detail": detail}, ensure_ascii=False).encode("utf-8")
        headers = self._cors_headers({"Content-Type": "application/json; charset=utf-8"})
        return 400, headers, payload

    def _server_error(self, message: str, detail: str) -> Response:
        payload = json.dumps({"error": message, "detail": detail}, ensure_ascii=False).encode("utf-8")
        headers = self._cors_headers({"Content-Type": "application/json; charset=utf-8"})
        return 500, headers, payload

    def _cors_headers(self, base: dict[str, str] | None = None) -> dict[str, str]:
        headers = base.copy() if base else {}
        headers["Access-Control-Allow-Origin"] = "*"
        headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
        headers["Access-Control-Allow-Headers"] = "Content-Type"
        return headers

