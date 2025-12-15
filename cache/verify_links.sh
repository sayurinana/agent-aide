#!/bin/bash
# 验证文档导览链接有效性

cd /home/user/temp/ccoptimize

echo "=== 验证导览链接 ==="
echo ""

# 定义要检查的文档文件
docs=(
    "docs/aide-overview.md"
    "aide-marketplace/aide-plugin/docs/README.md"
    "aide-marketplace/aide-plugin/docs/commands/setup.md"
    "aide-marketplace/aide-plugin/docs/commands/load.md"
    "aide-marketplace/aide-plugin/docs/commands/docs.md"
    "aide-marketplace/aide-plugin/docs/commands/run.md"
    "aide-program/docs/README.md"
    "aide-program/docs/commands/flow.md"
    "aide-program/docs/formats/config.md"
)

errors=0

for doc in "${docs[@]}"; do
    if [ ! -f "$doc" ]; then
        echo "✗ 文档不存在: $doc"
        ((errors++))
        continue
    fi

    dir=$(dirname "$doc")

    # 提取所有 markdown 链接
    links=$(grep -oE '\[([^]]+)\]\(([^)]+)\)' "$doc" | grep -oE '\(([^)]+)\)' | tr -d '()' | grep -v '^http' | grep -v '^#')

    if [ -z "$links" ]; then
        echo "→ $doc: 无相对链接"
        continue
    fi

    doc_errors=0
    for link in $links; do
        # 解析相对路径
        target="$dir/$link"
        # 移除锚点
        target=$(echo "$target" | sed 's/#.*//')

        if [ ! -e "$target" ]; then
            echo "✗ $doc: 链接无效 -> $link"
            ((doc_errors++))
            ((errors++))
        fi
    done

    if [ $doc_errors -eq 0 ]; then
        echo "✓ $doc: 所有链接有效"
    fi
done

echo ""
echo "=== 验证完成 ==="
if [ $errors -eq 0 ]; then
    echo "✓ 所有链接有效"
else
    echo "✗ 发现 $errors 个无效链接"
fi
