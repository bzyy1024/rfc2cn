#!/bin/bash
# =====================================================
# 本地构建 Docker 镜像并打包为 tar.gz 文件
# 用于传输到无开发环境的服务器
# =====================================================

set -e

IMAGE_NAME="rfc2cn-backend"
IMAGE_TAG="${1:-latest}"
FULL_IMAGE="${IMAGE_NAME}:${IMAGE_TAG}"
OUTPUT_FILE="${IMAGE_NAME}-${IMAGE_TAG}.tar.gz"

echo "========================================"
echo "  RFC2CN Backend Docker 镜像构建工具"
echo "========================================"
echo ""
echo "镜像名称 : ${FULL_IMAGE}"
echo "输出文件 : ${OUTPUT_FILE}"
echo ""

# 检查 Docker 是否可用
if ! command -v docker &>/dev/null; then
    echo "[ERROR] 未找到 docker 命令，请先安装 Docker"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BACKEND_DIR="${SCRIPT_DIR}/backend"

echo "[1/3] 构建镜像 ${FULL_IMAGE} ..."
docker build \
    --platform linux/amd64 \
    -t "${FULL_IMAGE}" \
    -f "${BACKEND_DIR}/Dockerfile.prod" \
    "${BACKEND_DIR}"

echo ""
echo "[2/3] 导出镜像为 ${OUTPUT_FILE} ..."
docker save "${FULL_IMAGE}" | gzip > "${SCRIPT_DIR}/${OUTPUT_FILE}"

SIZE=$(du -sh "${SCRIPT_DIR}/${OUTPUT_FILE}" | awk '{print $1}')
echo ""
echo "[3/3] 完成！"
echo ""
echo "========================================"
echo "  输出文件 : ${OUTPUT_FILE}  (${SIZE})"
echo "========================================"
echo ""
echo "下一步：将文件传输到服务器并运行 server-deploy.sh"
echo ""
echo "  # 传输文件（替换 user@server 为实际地址）"
echo "  scp ${OUTPUT_FILE} server-deploy.sh translate.sh user@server:~/"
echo ""
echo "  # 在服务器上执行"
echo "  ssh user@server 'bash ~/server-deploy.sh'"
