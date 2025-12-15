#!/bin/bash
cd /home/user/temp/ccoptimize/aide-marketplace/aide-plugin/commands

# 重命名旧命令文件
mv init.md _deprecated_init.md 2>/dev/null || echo "init.md 不存在或已重命名"
mv prep.md _deprecated_prep.md 2>/dev/null || echo "prep.md 不存在或已重命名"
mv exec.md _deprecated_exec.md 2>/dev/null || echo "exec.md 不存在或已重命名"

echo "旧命令文件已重命名"
ls -la
