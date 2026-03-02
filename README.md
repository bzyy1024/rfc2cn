# RFC2CN - RFC中文翻译网站

一个将RFC（Request for Comments）技术文档翻译成中文的网站，支持中英文对照阅读。

## 功能特点

- 📖 RFC文档中英文对照阅读
- 🔍 全文搜索功能
- 🤖 使用本地Ollama进行AI翻译
- 🔄 自动同步RFC数据（智能跳过已翻译内容）
- 🚀 SEO优化
- 💻 命令行工具管理RFC（无需登录系统）
- 🐳 Docker一键部署

## 技术栈

### 后端
- **Rust** + Axum (Web框架)
- **PostgreSQL** (数据库)
- **SQLx** (数据库ORM)

### 前端
- **Next.js 14** (React框架)
- **Tailwind CSS** (样式)
- **shadcn/ui** (UI组件)

### AI翻译
- **Ollama** (本地AI，推荐使用 qwen3:8b)

## 快速开始

### 环境配置

首先配置环境变量：

```bash
# 复制环境变量示例文件
cp .env.example .env

# 编辑 .env 文件，设置你的数据库密码等敏感信息
# 至少需要配置：
# - DATABASE_URL: 数据库连接字符串（包含密码）
# - POSTGRES_PASSWORD: PostgreSQL 数据库密码（生产环境）
```

### 方式一：Docker部署（推荐）

```bash
# 1. 克隆项目
git clone <repository-url>
cd rfc2cn

# 2. 配置环境变量（见上方"环境配置"）
cp .env.example .env
# 编辑 .env 文件

# 3. 构建并启动所有服务
./deploy.sh build
./deploy.sh up

# 4. 初始化Ollama模型（如需使用AI翻译）
./deploy.sh init-ollama

# 5. 同步RFC数据（可选）
./deploy.sh cli sync --start 1 --end 100

# 访问服务
# - 前端: http://localhost:3000
# - 后端API: http://localhost:8080
# - Ollama: http://localhost:11434



docker-compose up -d --build


```

#### 其他Docker命令

```bash
# 查看服务状态
./deploy.sh status

# 查看日志
./deploy.sh logs              # 所有服务
./deploy.sh logs backend      # 仅后端
./deploy.sh logs frontend     # 仅前端

# 重启服务
./deploy.sh restart

# 停止服务
./deploy.sh down

# 清理所有数据（包括数据库）
./deploy.sh clean

# 使用CLI工具
./deploy.sh cli list                    # 列出所有RFC
./deploy.sh cli add 6749                # 添加RFC
./deploy.sh cli translate 6749          # 翻译RFC
./deploy.sh cli sync --start 1 --end 50 --concurrent 3  # 同步RFC
```

### 方式二：本地开发

#### 1. 环境准备

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js (推荐使用 nvm)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20

# 安装 pnpm
npm install -g pnpm

# 安装 Ollama
curl -fsSL https://ollama.com/install.sh | sh

# 下载翻译模型
ollama pull qwen3:8b
```

#### 2. 数据库设置

```bash
# 启动 PostgreSQL (使用 docker-compose.yml)
docker-compose up -d postgres

# 初始化数据库（使用DBeaver或psql执行）
# 执行 backend/migrations/create_all_tables.sql
```

#### 3. 后端配置

```bash
cd backend

# 配置数据库连接（请替换为实际的密码）
export DATABASE_URL="postgres://rfc2cn:your_password@localhost:5432/rfc2cn"

# 运行服务
cargo run --bin rfc2cn-server

# 或使用发布版本
cargo build --release --bin rfc2cn-server
./target/release/rfc2cn-server
```

#### 4. 前端配置

```bash
cd frontend

# 安装依赖
pnpm install

# 复制环境配置
cp .env.local.example .env.local

# 启动开发服务器
pnpm dev
```

访问 http://localhost:3000 查看网站。

## CLI 工具使用

项目提供了命令行工具 `rfc-cli` 用于管理RFC文档：

### 基本命令

```bash
# 添加新的RFC
rfc-cli add <number> [--translate]

# 添加并翻译（推荐）
rfc-cli add 6749 --translate

# 翻译RFC
rfc-cli translate <number>

# 列出RFC
rfc-cli list [--status draft|published]

# 更新RFC状态
rfc-cli status <number> <status>  # status: draft|published

# 检查Ollama服务
rfc-cli check-ollama
```

### 自动同步命令

同步命令会自动从RFC官网下载RFC文档，智能判断是否需要添加或翻译：

```bash
# 同步单个RFC
rfc-cli sync --start 6749

# 同步范围（1-100）
rfc-cli sync --start 1 --end 100

# 同步但跳过翻译
rfc-cli sync --start 1 --end 100 --skip-translate

# 并发同步（提高速度）
rfc-cli sync --start 1 --end 100 --concurrent 3

# 完整示例：并发同步RFC 1-500，自动翻译
rfc-cli sync --start 1 --end 500 --concurrent 5


cargo run --bin rfc-cli sync --start 1 --end 100 --concurrent 1

cargo run --bin rfc-cli sync


/usr/bin/nohup ./rfc-cli sync --start 605 --end 10000 --concurrent 2 > log.log 2>&1 &

```

#### 同步逻辑

- ✅ 如果RFC已存在且已翻译 → **跳过**
- ✅ 如果RFC已存在但未翻译 → **自动翻译**
- ✅ 如果RFC不存在 → **添加并翻译**
- ✅ 如果RFC在官网不存在（404） → **跳过并记录**
- 🏷️ **自动标签提取**：使用AI从标题和摘要中提取3-8个相关技术标签
- ✅ 自动统计：添加数、跳过数、翻译数、失败数

#### 标签提取说明

自动标签提取功能会：
- 分析RFC的英文标题和摘要
- 如果有中文翻译，也会参考中文内容
- 使用AI（Ollama）提取3-8个最相关的技术标签
- 标签格式：小写英文，多词用连字符连接（如：`oauth`, `access-token`, `http-security`）
- 仅添加数据库中已存在的标签（16个预定义标签）
- 自动去重，不会重复添加已有的标签

## 项目结构

```

# 添加RFC并自动翻译
./target/release/rfc-cli add 6749 --tags oauth -T

# 翻译指定RFC
./target/release/rfc-cli translate 6749

# 强制重新翻译
./target/release/rfc-cli translate 6749 --force

# 查看RFC列表
./target/release/rfc-cli list

# 查看RFC状态
./target/release/rfc-cli status 6749

# 查看所有标签
./target/release/rfc-cli tags

# 为RFC添加标签
./target/release/rfc-cli tag 6749 http,security

# 检查Ollama服务状态
./target/release/rfc-cli check-ollama
```

## 项目结构

```
rfc2cn/
├── backend/                 # Rust后端
│   ├── src/
│   │   ├── main.rs         # 服务器入口
│   │   ├── bin/
│   │   │   └── cli.rs      # CLI工具
│   │   ├── config.rs       # 配置管理
│   │   ├── db.rs           # 数据库连接
│   │   ├── error.rs        # 错误处理
│   │   ├── handlers/       # HTTP处理器
│   │   ├── models/         # 数据模型
│   │   └── services/       # 业务逻辑
│   ├── migrations/         # 数据库迁移
│   └── Cargo.toml
├── frontend/               # Next.js前端
│   ├── src/
│   │   ├── app/           # 页面路由
│   │   ├── components/    # React组件
│   │   └── lib/           # 工具函数
│   └── package.json
└── docker-compose.yml      # Docker配置
```

## 配置说明

### 后端配置 (.env)

```bash
# 数据库
DATABASE_URL=postgres://user:pass@localhost:5432/rfc2cn

# 服务器
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# AI提供商: "ollama" 或 "openai"
AI_PROVIDER=ollama

# Ollama配置
OLLAMA_URL=http://localhost:11434
OLLAMA_MODEL=qwen2.5:14b
```

## 推荐的Ollama模型

- `qwen2.5:14b` - 推荐，中文翻译质量好
- `qwen2.5:7b` - 较小，速度更快

## 许可证

MIT License

### 前端启动

```bash
cd frontend
npm install
npm run dev
```

## 开发路线

- [x] 项目初始化
- [ ] 后端基础架构
- [ ] 数据库设计
- [ ] RFC抓取模块
- [ ] AI翻译集成
- [ ] 前端展示页面
- [ ] 管理后台

## License

MIT
