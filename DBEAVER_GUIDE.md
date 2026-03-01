# 🎯 DBeaver 操作步骤

## 第一步：连接数据库

1. **打开 DBeaver**

2. **创建新连接**：
   - 点击左上角的 "➕ 新建数据库连接"
   - 选择 **PostgreSQL**
   - 点击 "下一步"

3. **填写连接信息**：
   ```
   主机(Host):      localhost
   端口(Port):      5432
   数据库(Database): rfc2cn
   用户名(Username): rfc2cn
   密码(Password):  <你的数据库密码>
   ```

4. **测试连接**：
   - 点击 "测试连接"
   - 如果提示下载驱动，点击 "下载"
   - 看到 "连接成功" ✅

5. **点击 "完成"**

## 第二步：执行建表脚本

1. **打开 SQL 编辑器**：
   - 在左侧连接树中，右键点击 `rfc2cn` 数据库
   - 选择 "SQL 编辑器" → "新建 SQL 脚本"

2. **复制完整 SQL**：
   - 打开 `/home/bzyy/Desktop/tmp/rfc2cn/backend/create_all_tables.sql`
   - 全选（Ctrl+A）并复制所有内容

3. **粘贴并执行**：
   - 在 DBeaver 的 SQL 编辑器中粘贴
   - 点击左上角的 "▶️ 执行 SQL 脚本" 按钮（或按 Ctrl+X）
   - 等待执行完成

4. **验证成功**：
   - 在消息窗口应该看到：`Database initialized successfully!`
   - 在左侧连接树中，刷新 `rfc2cn` 数据库
   - 展开 "表" 节点，应该看到：
     ```
     ✅ glossary
     ✅ rfc_tags
     ✅ rfcs
     ✅ tags
     ✅ translation_tasks
     ✅ translations
     ```

## 第三步：启动后端服务器

返回终端执行：

```bash
cd /home/bzyy/Desktop/tmp/rfc2cn/backend
cargo run --bin rfc2cn-server
```

应该看到：
```
✅ 配置加载完成
✅ 数据库连接成功
✅ 数据库迁移完成
🚀 服务器启动在 http://0.0.0.0:8080
```

## 第四步：测试 CLI 工具

添加第一个 RFC：

```bash
# 检查 Ollama 是否运行
cargo run --bin rfc-cli -- check-ollama

# 添加 RFC 6749（OAuth 2.0）并立即翻译
cargo run --bin rfc-cli -- add 6749 --tags oauth --translate

# 查看添加的 RFC
cargo run --bin rfc-cli -- list
```

## 第五步：启动前端（可选）

```bash
cd /home/bzyy/Desktop/tmp/rfc2cn/frontend

# 安装依赖
npm install

# 创建环境变量
echo "NEXT_PUBLIC_API_URL=http://localhost:8080" > .env.local

# 启动开发服务器
npm run dev
```

前端访问：http://localhost:3000

---

## 📌 快速截图指南

如果遇到问题，请截图：
1. DBeaver 的连接配置界面
2. SQL 执行结果窗口
3. 终端的错误信息

## 🔍 常见问题

**Q: 提示 "连接被拒绝"**
A: 确保 PostgreSQL 服务正在运行，检查端口 5432 是否开放

**Q: 提示 "权限不足"**
A: 确保使用的是 `rfc2cn` 用户，不是 `postgres` 用户

**Q: 执行 SQL 报错**
A: 先运行一次脚本中的 DROP 语句清理，再重新执行整个脚本
