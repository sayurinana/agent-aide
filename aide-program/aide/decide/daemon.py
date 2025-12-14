"""后台服务入口点。

通过 subprocess 启动，独立运行 HTTP 服务。
用法: python -m aide.decide.daemon <project_root>
"""

from __future__ import annotations

import os
import signal
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) < 2:
        sys.stderr.write("用法: python -m aide.decide.daemon <project_root>\n")
        return 1

    root = Path(sys.argv[1])
    if not root.exists():
        sys.stderr.write(f"目录不存在: {root}\n")
        return 1

    # 延迟导入，避免循环依赖
    from aide.decide.server import DecideServer
    from aide.decide.storage import DecideStorage

    storage = DecideStorage(root)
    server = DecideServer(root, storage)

    # 注册信号处理
    def handle_sigterm(signum: int, frame: object) -> None:
        server.stop("terminated")

    signal.signal(signal.SIGTERM, handle_sigterm)

    # 获取当前进程 PID
    pid = os.getpid()

    # 启动服务（会保存 server.json，阻塞等待）
    success = server.start_daemon(pid)
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())
