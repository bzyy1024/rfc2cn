#!/bin/bash
# 测试自动标签提取功能

cd /home/bzyy/Desktop/tmp/rfc2cn/backend

echo "========================================="
echo "测试 1: 检查 Ollama 服务状态"
echo "========================================="
cargo run --bin rfc-cli -- check-ollama
echo ""

echo "========================================="
echo "测试 2: 添加 RFC（默认启用自动标签提取）"
echo "========================================="
timeout 150 cargo run --bin rfc-cli -- add 8446
echo ""

echo "========================================="
echo "测试完成！"
echo "========================================="
