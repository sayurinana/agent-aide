#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
VENV_PY="${PROJECT_ROOT}/.venv/Scripts/python"

if [ ! -x "$VENV_PY" ]; then
  echo "✗ 未找到虚拟环境，请先运行：uv venv .venv && uv pip install -r requirements.txt" >&2
  exit 1
fi

# 不切换目录，保持用户的工作目录
export PYTHONPATH="${PROJECT_ROOT}${PYTHONPATH:+:$PYTHONPATH}"
exec "$VENV_PY" -m aide "$@"
