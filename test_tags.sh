#!/bin/bash

# 测试标签生成功能
echo "=== 测试标签生成功能 ==="
echo ""

# 1. 测试添加RFC并自动生成标签
echo "1. 测试添加 RFC 8446 (TLS 1.3) 并自动生成标签..."
cd backend
cargo run --bin rfc-cli -- add 8446 --auto-tag 2>&1 | tail -30

echo ""
echo "2. 查看生成的标签列表..."
cargo run --bin rfc-cli -- tags 2>&1 | tail -30

echo ""
echo "3. 查看 RFC 8446 的状态和标签..."
cargo run --bin rfc-cli -- status 8446 2>&1 | tail -20

echo ""
echo "=== 测试完成 ==="
