# 🔧 快速修复数据库迁移错误

## 问题
看到错误：`type "rfc_status" already exists`

## ✅ 最简单的解决方案

### 选项1：使用SQL工具（推荐）

1. **使用任何 PostgreSQL 客户端工具连接数据库**：
   - DBeaver（推荐，免费图形界面）
   - pgAdmin
   - TablePlus
   - 或在线SQL客户端

2. **连接信息**（从 .env 文件）：
   ```
   Host: localhost
   Port: 5432
   Database: rfc2cn
   Username: rfc2cn
   Password: <你的数据库密码>
   ```

3. **运行以下SQL**：
   ```sql
   DROP SCHEMA public CASCADE;
   CREATE SCHEMA public;
   GRANT ALL ON SCHEMA public TO rfc2cn;
   GRANT ALL ON SCHEMA public TO public;
   ```

4. **启动服务器**：
   ```bash
   cd backend
   cargo run --bin rfc2cn-server
   ```

### 选项2：安装 psql 并使用命令行

```bash
# 安装 PostgreSQL 客户端
sudo apt install postgresql-client

# 重置数据库（请将 your_password 替换为实际密码）
psql "postgres://rfc2cn:your_password@localhost:5432/rfc2cn" << EOF
DROP SCHEMA public CASCADE;
CREATE SCHEMA public;
GRANT ALL ON SCHEMA public TO rfc2cn;
GRANT ALL ON SCHEMA public TO public;
EOF

# 启动服务器
cd backend
cargo run --bin rfc2cn-server
```

### 选项3：使用 Python 脚本

```bash
# 安装依赖
pip install psycopg2-binary

# 运行重置脚本
cd backend
python3 reset_db.py
```

### 选项4：完全重建数据库

如果有 PostgreSQL 超级用户权限：

```bash
# 以 postgres 用户连接
sudo -u postgres psql << EOF
DROP DATABASE IF EXISTS rfc2cn;
CREATE DATABASE rfc2cn OWNER rfc2cn;
EOF

# 启动服务器
cd backend
cargo run --bin rfc2cn-server
```

## 🎯 快速测试

重置后，启动服务器应该看到：

```
✅ 配置加载完成
✅ 数据库连接成功
✅ 数据库迁移完成
🚀 服务器启动在 http://0.0.0.0:8080
```

## 📝 下载图形化工具（如果需要）

- **DBeaver**（推荐）: https://dbeaver.io/download/
- **pgAdmin**: https://www.pgadmin.org/download/
- **TablePlus**: https://tableplus.com/

使用图形化工具更简单，只需连接数据库后，在SQL编辑器中粘贴并执行SQL即可。
