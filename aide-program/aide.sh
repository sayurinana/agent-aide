#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
VENV_PY="${PROJECT_ROOT}/.venv/bin/python"

if [ ! -x "$VENV_PY" ]; then
  echo "✗ 未找到虚拟环境，请先运行：uv venv .venv && uv pip install -r requirements.txt" >&2
  exit 1
fi

cd "$PROJECT_ROOT"
export PYTHONPATH="${PROJECT_ROOT}/aide-program${PYTHONPATH:+:$PYTHONPATH}"
exec "$VENV_PY" -m aide "$@"
