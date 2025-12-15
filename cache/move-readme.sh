#!/bin/bash
# 将 README.md 移至 docs/project-details.md
set -e

cd /home/user/temp/ccoptimize

# 执行移动操作
mv README.md docs/project-details.md

echo "✓ README.md 已移至 docs/project-details.md"
