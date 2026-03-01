# RFC2CN 快速启动指南

## 已完成的功能

✅ 移除了用户登录认证模块，内容完全开放
✅ 添加了标签分类系统
✅ 集成了Ollama本地AI翻译（也支持OpenAI）
✅ 创建了CLI工具用于添加和管理RFC
✅ 支持全文搜索和标签筛选
✅ SEO优化的Next.js前端

## 启动步骤

### 1. 启动数据库

```bash
# 使用docker-compose启动PostgreSQL
docker-compose up -d

# 或者使用已有的PostgreSQL，创建数据库
# createdb rfc2cn
```

### 2. 启动Ollama（推荐用于翻译）

```bash
# 如果还没安装Ollama
curl -fsSL https://ollama.com/install.sh | sh

# 启动Ollama服务
ollama serve

# 新开一个终端，下载翻译模型
ollama pull qwen2.5:14b
```

### 3. 配置后端

后端配置文件 `.env` 已创建，默认配置：
- 数据库: `postgres://postgres:postgres@localhost:5432/rfc2cn`
- Ollama: `http://localhost:11434`
- 模型: `qwen2.5:14b`

### 4. 启动后端服务

```bash
cd backend
cargo run --bin rfc2cn-server
```

后端会自动运行数据库迁移并启动在 http://localhost:8080

### 5. 使用CLI添加RFC

```bash
cd backend

# 添加RFC 6749 (OAuth 2.0)并自动翻译
cargo run --bin rfc-cli -- add 6749 --tags oauth -T

# 或者不带自动翻译
cargo run --bin rfc-cli -- add 6749 --tags oauth

# 手动触发翻译
cargo run --bin rfc-cli -- translate 6749

# 查看RFC状态
cargo run --bin rfc-cli -- status 6749

# 列出所有RFC
cargo run --bin rfc-cli -- list

# 查看所有标签
cargo run --bin rfc-cli -- tags

# 检查Ollama服务状态
cargo run --bin rfc-cli -- check-ollama
```

### 6. 启动前端

```bash
cd frontend

# 安装依赖
npm install

# 创建环境配置
echo "NEXT_PUBLIC_API_URL=http://localhost:8080" > .env.local

# 启动开发服务器
npm run dev
```

前端会启动在 http://localhost:3000

## 可用的标签

已预置的标签：
- http, oauth, tls, dns, tcp, udp
- websocket, json, rest, smtp, imap
- ipv4, ipv6, quic, security, cryptography

可以通过CLI添加更多标签到RFC。

## API接口

后端提供以下API：

- `GET /api/rfcs` - 获取RFC列表
- `GET /api/rfcs/search?q=keyword` - 搜索RFC
- `GET /api/rfcs/:number` - 获取RFC详情（含标签）
- `GET /api/rfcs/:number/translations` - 获取翻译内容
- `GET /api/tags` - 获取所有标签
- `GET /api/tags/:slug/rfcs` - 获取标签下的RFC

## 推荐的工作流

1. 启动所有服务（数据库、Ollama、后端）
2. 使用CLI添加需要的RFC
3. 通过前端浏览和阅读翻译好的RFC
4. RFC支持中英对照、仅中文、仅英文三种阅读模式

## 注意事项

- 翻译时间取决于RFC长度和Ollama模型，通常需要几分钟到十几分钟
- 推荐使用 `qwen2.5:14b` 模型，翻译质量较好
- 如果使用 `qwen2.5:7b`，翻译速度更快但质量略低
- 翻译是分段进行的，即使中断也可以继续翻译剩余部分
