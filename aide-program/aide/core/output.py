"""输出格式工具，统一使用前缀符号。"""


def ok(message: str) -> None:
    print(f"✓ {message}")


def warn(message: str) -> None:
    print(f"⚠ {message}")


def err(message: str) -> None:
    print(f"✗ {message}")


def info(message: str) -> None:
    print(f"→ {message}")


def step(message: str, current: int | None = None, total: int | None = None) -> None:
    if current is not None and total is not None:
        print(f"[{current}/{total}] {message}")
    else:
        print(f"[·] {message}")
