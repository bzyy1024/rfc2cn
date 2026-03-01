#!/bin/bash

echo "=== 测试多标签AND搜索功能 ==="
echo ""

# 等待服务器启动
echo "等待服务器启动..."
sleep 2

echo "1. 测试单标签搜索 (http):"
curl -s "http://localhost:3000/api/search?tags=http" | jq '.rfcs[] | {rfc_number, title}'

echo ""
echo "2. 测试多标签AND搜索 (http,web-protocol):"
curl -s "http://localhost:3000/api/search?tags=http,web-protocol" | jq '.rfcs[] | {rfc_number, title}'

echo ""
echo "3. 测试三个标签AND搜索 (http,web-protocol,protocol-standard):"
curl -s "http://localhost:3000/api/search?tags=http,web-protocol,protocol-standard" | jq '.rfcs[] | {rfc_number, title}'

echo ""
echo "=== 测试完成 ==="
