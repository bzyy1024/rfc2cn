#!/bin/bash
# 快速启动脚本

echo "🚀 启动 RFC2CN 后端服务器..."

cd "$(dirname "$0")"

# 检查数据库连接
if ! timeout 2 psql "$DATABASE_URL" -c "SELECT 1" > /dev/null 2>&1; then
    echo "❌ 数据库连接失败，请检查 .env 中的 DATABASE_URL"
    exit 1
fi

echo "✅ 数据库连接正常"

# 启动服务器
cargo run --release --bin rfc2cn-server
