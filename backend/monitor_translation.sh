#!/bin/bash
# 翻译进度监控脚本

RFC_NUM=${1:-1196}

echo "📊 监控 RFC $RFC_NUM 的翻译进度..."
echo ""

while true; do
    # 获取翻译统计
    TOTAL=$(curl -s "http://localhost:8080/api/rfcs/$RFC_NUM/translations" | jq 'length')
    TRANSLATED=$(curl -s "http://localhost:8080/api/rfcs/$RFC_NUM/translations" | jq '[.[] | select(.translated_text != null)] | length')
    
    if [ "$TOTAL" -gt 0 ]; then
        PERCENT=$((TRANSLATED * 100 / TOTAL))
        echo -ne "\r已翻译: $TRANSLATED / $TOTAL ($PERCENT%)  "
    else
        echo -ne "\r获取数据中...  "
    fi
    
    sleep 2
done
