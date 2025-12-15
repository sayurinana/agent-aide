#!/bin/bash
cd /home/user/temp/ccoptimize/aide-marketplace/aide-plugin/docs/commands

# 重命名旧设计文档
mv init.md _deprecated_init.md 2>/dev/null || echo "init.md 不存在"
mv prep.md _deprecated_prep.md 2>/dev/null || echo "prep.md 不存在"
mv exec.md _deprecated_exec.md 2>/dev/null || echo "exec.md 不存在"

echo "旧设计文档已重命名"
ls -la
