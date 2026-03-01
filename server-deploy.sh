#!/bin/bash
# =====================================================
# 服务器端部署脚本
# 在目标服务器上执行，无需任何开发环境，只需 Docker
# 用法: bash server-deploy.sh [镜像tar包路径]
# =====================================================

set -e

# -------- 配置区域（按需修改）--------
NETWORK_NAME="rfc2cn-net"

# PostgreSQL 配置
PG_CONTAINER="rfc2cn-postgres"
PG_IMAGE="postgres:16-alpine"
PG_DB="${POSTGRES_DB:-rfc2cn}"
PG_USER="${POSTGRES_USER:-rfc2cn}"
PG_PASSWORD="${POSTGRES_PASSWORD}"   # ← 请在环境变量中设置
PG_DATA_VOLUME="rfc2cn_pgdata"
PG_PORT="5432"                    # 设为空字符串则不暴露到宿主机

# Backend 配置
BACKEND_CONTAINER="rfc2cn-backend"
BACKEND_IMAGE="rfc2cn-backend:latest"
BACKEND_PORT="8080"

# AI 配置 —— 按实际情况选择其中一种
AI_PROVIDER="openai"              # "ollama" 或 "openai"
OPENAI_API_KEY=""                 # 使用 OpenAI 时填写，或留空后在下方 export
OPENAI_API_BASE="https://api.openai.com/v1"
OPENAI_MODEL="gpt-4o-mini"
OLLAMA_URL="http://rfc2cn-ollama:11434"  # 使用 Ollama 时的地址
OLLAMA_MODEL="qwen2.5:14b"
# --------------------------------------

echo "========================================"
echo "  RFC2CN Backend 服务器部署"
echo "========================================"

# 如果通过环境变量传入了 API Key，优先使用
if [[ -n "${OPENAI_API_KEY_ENV}" ]]; then
    OPENAI_API_KEY="${OPENAI_API_KEY_ENV}"
fi

# 检查 Docker
if ! command -v docker &>/dev/null; then
    echo "[ERROR] 未安装 Docker，请先安装"
    exit 1
fi

# ---- Step 1: 加载镜像 ----
TAR_FILE="${1:-rfc2cn-backend-latest.tar.gz}"
if [[ -f "${TAR_FILE}" ]]; then
    echo "[1/5] 加载镜像: ${TAR_FILE}"
    docker load < "${TAR_FILE}"
else
    # 如果没有 tar 文件，检查镜像是否已存在
    if docker image inspect "${BACKEND_IMAGE}" &>/dev/null; then
        echo "[1/5] 镜像 ${BACKEND_IMAGE} 已存在，跳过加载"
    else
        echo "[ERROR] 未找到 ${TAR_FILE}，且本地无 ${BACKEND_IMAGE} 镜像"
        echo "请先运行 build-image.sh 并将生成的 tar.gz 文件传输到此目录"
        exit 1
    fi
fi

# ---- Step 2: 创建网络 ----
echo "[2/5] 创建 Docker 网络: ${NETWORK_NAME}"
docker network inspect "${NETWORK_NAME}" &>/dev/null \
    || docker network create "${NETWORK_NAME}"

# ---- Step 3: 启动 PostgreSQL ----
echo "[3/5] 启动 PostgreSQL ..."
if docker inspect "${PG_CONTAINER}" &>/dev/null; then
    echo "  容器 ${PG_CONTAINER} 已存在，跳过创建"
    docker start "${PG_CONTAINER}" 2>/dev/null || true
else
    PG_PORTS=""
    [[ -n "${PG_PORT}" ]] && PG_PORTS="-p ${PG_PORT}:5432"

    docker run -d \
        --name "${PG_CONTAINER}" \
        --network "${NETWORK_NAME}" \
        ${PG_PORTS} \
        -e POSTGRES_DB="${PG_DB}" \
        -e POSTGRES_USER="${PG_USER}" \
        -e POSTGRES_PASSWORD="${PG_PASSWORD}" \
        -v "${PG_DATA_VOLUME}:/var/lib/postgresql/data" \
        --restart unless-stopped \
        "${PG_IMAGE}"
fi

# 等待 PostgreSQL 就绪
echo "  等待数据库就绪..."
for i in $(seq 1 30); do
    if docker exec "${PG_CONTAINER}" pg_isready -U "${PG_USER}" -d "${PG_DB}" &>/dev/null; then
        echo "  数据库已就绪"
        break
    fi
    sleep 2
    if [[ $i -eq 30 ]]; then
        echo "[ERROR] 数据库启动超时"
        exit 1
    fi
done

# ---- Step 4: 初始化数据库表（首次运行） ----
echo "[4/5] 检查并初始化数据库表..."
TABLE_EXISTS=$(docker exec "${PG_CONTAINER}" psql -U "${PG_USER}" -d "${PG_DB}" -tAc \
    "SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name='rfcs')")
if [[ "${TABLE_EXISTS}" == "f" ]]; then
    echo "  首次运行，执行建表脚本..."
    # 从 backend 容器镜像内的迁移文件初始化（需要容器中有该文件）
    # 从镜像中提取建表 SQL，通过 psql 执行
    SQL=$(docker run --rm --entrypoint cat "${BACKEND_IMAGE}" /app/migrations/create_all_tables.sql 2>/dev/null)
    if [[ -n "${SQL}" ]]; then
        echo "${SQL}" | docker exec -i "${PG_CONTAINER}" psql -U "${PG_USER}" -d "${PG_DB}"
        echo "  建表完成"
    else
        echo "  [WARN] 无法获取建表脚本，请手动执行 create_all_tables.sql"
    fi
else
    echo "  数据库表已存在，跳过初始化"
fi

# ---- Step 5: 启动 Backend ----
echo "[5/5] 启动 Backend ..."
if docker inspect "${BACKEND_CONTAINER}" &>/dev/null; then
    echo "  停止旧容器..."
    docker stop "${BACKEND_CONTAINER}" && docker rm "${BACKEND_CONTAINER}"
fi

docker run -d \
    --name "${BACKEND_CONTAINER}" \
    --network "${NETWORK_NAME}" \
    -p "${BACKEND_PORT}:8080" \
    -e DATABASE_URL="postgres://${PG_USER}:${PG_PASSWORD}@${PG_CONTAINER}:5432/${PG_DB}" \
    -e SERVER_HOST="0.0.0.0" \
    -e SERVER_PORT="8080" \
    -e RUST_LOG="info" \
    -e AI_PROVIDER="${AI_PROVIDER}" \
    -e OPENAI_API_KEY="${OPENAI_API_KEY}" \
    -e OPENAI_API_BASE="${OPENAI_API_BASE}" \
    -e OPENAI_MODEL="${OPENAI_MODEL}" \
    -e OLLAMA_URL="${OLLAMA_URL}" \
    -e OLLAMA_MODEL="${OLLAMA_MODEL}" \
    --restart unless-stopped \
    "${BACKEND_IMAGE}"

echo ""
echo "========================================"
echo "  部署完成！"
echo "========================================"
echo ""
echo "  API 服务: http://0.0.0.0:${BACKEND_PORT}"
echo "  健康检查: curl http://localhost:${BACKEND_PORT}/api/health"
echo ""
echo "  查看日志: docker logs -f ${BACKEND_CONTAINER}"
echo "  运行翻译: bash translate.sh <rfc号码>"
echo ""
