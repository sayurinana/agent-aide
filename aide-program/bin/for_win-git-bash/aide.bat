@echo off
setlocal

set SCRIPT_DIR=%~dp0
set PROJECT_ROOT=%SCRIPT_DIR%..\..
set VENV_PY=%PROJECT_ROOT%\.venv\Scripts\python.exe
set PYTHONPATH=%PROJECT_ROOT%\aide-program;%PYTHONPATH%

if not exist "%VENV_PY%" (
    echo ✗ 未找到虚拟环境，请先运行：uv venv .venv ^&^& uv pip install -r requirements.txt
    exit /b 1
)

pushd "%PROJECT_ROOT%"
"%VENV_PY%" -m aide %*
popd
