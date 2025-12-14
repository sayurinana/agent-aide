"""HTTP 服务器生命周期管理。"""

from __future__ import annotations

import json
import socket
import time
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

from aide.core import output
from aide.core.config import ConfigManager
from aide.decide.errors import DecideError
from aide.decide.handlers import DecideHandlers, Response
from aide.decide.storage import DecideStorage


class DecideHTTPServer(HTTPServer):
    """附带处理器实例的 HTTPServer。"""

    def __init__(self, server_address, RequestHandlerClass, handlers: DecideHandlers):
        self.handlers = handlers
        super().__init__(server_address, RequestHandlerClass)


class DecideServer:
    """启动、监听与关闭 HTTP 服务。"""

    def __init__(self, root, storage: DecideStorage):
        self.root = root
        self.storage = storage
        self.port = 3721
        self.timeout = 0
        self.bind = "127.0.0.1"
        self.url = ""
        # web 资源位于 aide 包目录下，而非项目根目录
        self.web_dir = Path(__file__).parent / "web"
        self.should_close = False
        self.close_reason: str | None = None
        self.httpd: DecideHTTPServer | None = None

    def start(self) -> bool:
        try:
            config = ConfigManager(self.root).load_config()
            start_port = _get_int(config, "decide", "port", default=3721)
            self.timeout = _get_int(config, "decide", "timeout", default=0)
            self.bind = _get_str(config, "decide", "bind", default="127.0.0.1")
            self.url = _get_str(config, "decide", "url", default="")
            end_port = start_port + 9
            available = self._find_available_port(start_port)
            if available is None:
                output.err(f"无法启动服务: 端口 {start_port}-{end_port} 均被占用")
                output.info("建议: 关闭占用端口的程序，或在配置中指定其他端口")
                return False
            self.port = available

            handlers = DecideHandlers(
                storage=self.storage,
                web_dir=self.web_dir,
                stop_callback=self.stop,
            )
            RequestHandler = self._build_request_handler(handlers)
            self.httpd = DecideHTTPServer((self.bind, self.port), RequestHandler, handlers)
            self.httpd.timeout = 1.0

            # 生成访问地址：优先使用自定义 url，否则自动生成
            if self.url:
                access_url = self.url
            else:
                access_url = f"http://localhost:{self.port}"

            output.info("Web 服务已启动")
            output.info(f"请访问: {access_url}")
            output.info("等待用户完成决策...")

            self._serve_forever()

            if self.close_reason == "completed":
                output.ok("决策已完成")
                return True
            if self.close_reason == "timeout":
                output.warn("服务超时，已自动关闭")
                return True
            if self.close_reason == "interrupted":
                output.warn("服务已中断")
                return True
            return True
        except DecideError as exc:
            output.err(str(exc))
            return False

    def stop(self, reason: str) -> None:
        if self.should_close:
            return
        self.should_close = True
        self.close_reason = reason

    def start_daemon(self, pid: int) -> bool:
        """作为后台进程启动服务（由 daemon.py 调用）。

        与 start() 的区别：
        - 不输出到 stdout（后台运行）
        - 保存服务信息到 server.json
        - 退出时清理 server.json
        """
        try:
            config = ConfigManager(self.root).load_config()
            start_port = _get_int(config, "decide", "port", default=3721)
            self.timeout = _get_int(config, "decide", "timeout", default=0)
            self.bind = _get_str(config, "decide", "bind", default="127.0.0.1")
            self.url = _get_str(config, "decide", "url", default="")

            available = self._find_available_port(start_port)
            if available is None:
                return False
            self.port = available

            handlers = DecideHandlers(
                storage=self.storage,
                web_dir=self.web_dir,
                stop_callback=self.stop,
            )
            RequestHandler = self._build_request_handler(handlers)
            self.httpd = DecideHTTPServer((self.bind, self.port), RequestHandler, handlers)
            self.httpd.timeout = 1.0

            # 生成访问地址
            access_url = self.url if self.url else f"http://localhost:{self.port}"

            # 保存服务信息（供 CLI 读取）
            self.storage.save_server_info(pid, self.port, access_url)

            # 阻塞等待用户操作
            self._serve_forever()

            # 清理服务信息
            self.storage.clear_server_info()
            return True
        except Exception:
            self.storage.clear_server_info()
            return False

    def _find_available_port(self, start: int) -> int | None:
        attempts = 10
        for offset in range(attempts):
            port = start + offset
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
                sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
                try:
                    sock.bind((self.bind, port))
                    return port
                except OSError:
                    continue
        return None

    def _serve_forever(self) -> None:
        if self.httpd is None:
            return
        deadline = None if self.timeout <= 0 else time.time() + self.timeout

        try:
            while not self.should_close:
                if deadline is not None and time.time() >= deadline:
                    self.stop("timeout")
                    break
                self.httpd.handle_request()
        except KeyboardInterrupt:
            self.stop("interrupted")
        finally:
            try:
                self.httpd.server_close()
            except Exception:
                pass

    def _build_request_handler(self, handlers: DecideHandlers):
        server = self

        class RequestHandler(BaseHTTPRequestHandler):
            protocol_version = "HTTP/1.1"

            def do_GET(self):
                self._dispatch("GET")

            def do_POST(self):
                self._dispatch("POST")

            def do_OPTIONS(self):
                self._dispatch("OPTIONS")

            def _dispatch(self, method: str) -> None:
                length = self.headers.get("Content-Length")
                body = b""
                if method == "POST":
                    try:
                        content_length = int(length) if length else 0
                    except ValueError:
                        self._send_response(
                            (400, handlers._cors_headers({"Content-Type": "application/json; charset=utf-8"}), '{"error":"决策数据无效","detail":"无效的 Content-Length"}'.encode("utf-8"))
                        )
                        return
                    if content_length > 1024 * 1024:
                        self._send_response(
                            (
                                413,
                                handlers._cors_headers({"Content-Type": "application/json; charset=utf-8"}),
                                '{"error":"请求体过大","detail":"单次提交限制 1MB"}'.encode("utf-8"),
                            )
                        )
                        return
                    body = self.rfile.read(content_length)

                try:
                    response = handlers.handle(method, self.path, body)
                except Exception as exc:  # pragma: no cover - 兜底防御
                    payload = (
                        '{"error":"服务器内部错误","detail":'
                        + json.dumps(str(exc), ensure_ascii=False)
                        + "}"
                    ).encode("utf-8")
                    response = (
                        500,
                        handlers._cors_headers({"Content-Type": "application/json; charset=utf-8"}),
                        payload,
                    )
                self._send_response(response)

                if server.should_close:
                    # 已由 handlers 设置关闭标志，等待当前请求结束
                    pass

            def log_message(self, format: str, *args) -> None:  # noqa: A003
                # 静默日志，避免干扰 CLI 输出
                return

            def _send_response(self, response: Response) -> None:
                status, headers, body = response
                self.send_response(status)
                for key, value in headers.items():
                    self.send_header(key, value)
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                if body:
                    self.wfile.write(body)

        return RequestHandler


def _get_int(config: dict, section: str, key: str, default: int) -> int:
    try:
        section_data = config.get(section, {}) if isinstance(config, dict) else {}
        value = section_data.get(key, default)
        if isinstance(value, bool):
            return default
        if isinstance(value, (int, float)):
            as_int = int(value)
            return as_int if as_int >= 0 else default
    except Exception:
        return default
    return default


def _get_str(config: dict, section: str, key: str, default: str) -> str:
    try:
        section_data = config.get(section, {}) if isinstance(config, dict) else {}
        value = section_data.get(key, default)
        if isinstance(value, str):
            return value
    except Exception:
        return default
    return default
