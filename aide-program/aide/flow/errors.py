"""错误类型：用于将内部失败统一映射为 CLI 输出。"""

from __future__ import annotations


class FlowError(RuntimeError):
    pass

