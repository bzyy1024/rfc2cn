# 数据库迁移错误修复指南

## 问题描述

如果看到错误：`type "rfc_status" already exists`，这是因为数据库中已经存在旧的schema。

## 解决方案

### 方案1：删除并重建数据库（推荐，最干净）

```bash
cd /home/bzyy/Desktop/tmp/rfc2cn

# 如果使用 docker-compose
docker-compose down -v
docker-compose up -d

# 如果手动管理 PostgreSQL
dropdb rfc2cn
createdb rfc2cn

# 然后启动服务器，会自动运行迁移
cd backend
cargo run --bin rfc2cn-server
```

### 方案2：手动清理数据库

使用PostgreSQL客户端工具（如DBeaver、pgAdmin、psql等）连接到数据库，执行以下SQL：

```sql
-- 清理所有对象
DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

-- 授予权限
GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO public;
```

或者运行提供的脚本：

```bash
# 假设你的数据库是 postgres://postgres:postgres@localhost:5432/rfc2cn
psql "postgres://postgres:postgres@localhost:5432/rfc2cn" < manual_reset.sql
```

### 方案3：使用提供的重置脚本

```bash
cd backend

# 方法1：如果安装了psql命令
cat manual_reset.sql | psql "$DATABASE_URL"

# 方法2：如果使用docker的PostgreSQL
docker exec -i <postgres_container_id> psql -U postgres -d rfc2cn < manual_reset.sql

# 然后启动服务器
cargo run --bin rfc2cn-server
```

## 验证

启动服务器后应该看到：

```
✅ 数据库连接成功
✅ 数据库迁移完成
🚀 服务器启动在 http://0.0.0.0:8080
```

## 预防措施

如果后续需要修改数据库schema：
1. 不要修改已经运行的迁移文件
2. 创建新的迁移文件（新的时间戳）
3. 使用 `IF NOT EXISTS`、`IF EXISTS` 等条件语句
4. 在开发环境测试迁移后再应用到生产环境

## 快速重新开始（开发环境）

如果这是全新的开发环境，最快的方法：

```bash
# 1. 停止所有服务
pkill rfc2cn-server

# 2. 删除并重建数据库
dropdb rfc2cn 2>/dev/null || true
createdb rfc2cn

# 3. 启动服务器（自动运行迁移）
cd backend
cargo run --bin rfc2cn-server
```
