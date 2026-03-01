#!/bin/bash
# =====================================================
# 翻译任务执行脚本
# 在服务器上使用 rfc-cli 运行翻译任务（无需开发环境）
# 用法:
#   bash translate.sh <rfc号码>              # 抓取并翻译
#   bash translate.sh translate <rfc号码>    # 仅翻译（已抓取）
#   bash translate.sh list                   # 列出所有RFC
#   bash translate.sh status <rfc号码>       # 查看翻译状态
#   bash translate.sh batch 1 100            # 批量翻译 RFC 1~100
# =====================================================

BACKEND_IMAGE="rfc2cn-backend:latest"
NETWORK_NAME="rfc2cn-net"
PG_CONTAINER="rfc2cn-postgres"
PG_USER="rfc2cn"
PG_PASSWORD="HK2DbXkBeF6ASsCb"
PG_DB="rfc2cn"

# AI 配置（与 server-deploy.sh 保持一致）
AI_PROVIDER="${AI_PROVIDER:-openai}"
OPENAI_API_KEY="${OPENAI_API_KEY:-}"
OPENAI_API_BASE="${OPENAI_API_BASE:-https://api.openai.com/v1}"
OPENAI_MODEL="${OPENAI_MODEL:-gpt-4o-mini}"
OLLAMA_URL="${OLLAMA_URL:-http://rfc2cn-ollama:11434}"
OLLAMA_MODEL="${OLLAMA_MODEL:-qwen2.5:14b}"

# 公共 docker run 参数（不启动 daemon，用完即删）
run_cli() {
    docker run --rm \
        --network "${NETWORK_NAME}" \
        -e DATABASE_URL="postgres://${PG_USER}:${PG_PASSWORD}@${PG_CONTAINER}:5432/${PG_DB}" \
        -e RUST_LOG="info" \
        -e AI_PROVIDER="${AI_PROVIDER}" \
        -e OPENAI_API_KEY="${OPENAI_API_KEY}" \
        -e OPENAI_API_BASE="${OPENAI_API_BASE}" \
        -e OPENAI_MODEL="${OPENAI_MODEL}" \
        -e OLLAMA_URL="${OLLAMA_URL}" \
        -e OLLAMA_MODEL="${OLLAMA_MODEL}" \
        --entrypoint rfc-cli \
        "${BACKEND_IMAGE}" \
        "$@"
}

CMD="${1:-help}"

case "${CMD}" in
    help|--help|-h)
        echo "用法:"
        echo "  bash translate.sh list                   列出所有RFC"
        echo "  bash translate.sh add <rfc号码>          抓取RFC原文"
        echo "  bash translate.sh translate <rfc号码>    翻译已抓取的RFC"
        echo "  bash translate.sh status <rfc号码>       查看翻译状态"
        echo "  bash translate.sh <rfc号码>              抓取并翻译（一键）"
        echo "  bash translate.sh batch <起始> <结束>    批量翻译（例: batch 1 100）"
        echo ""
        echo "环境变量覆盖示例:"
        echo "  OPENAI_API_KEY=sk-xxx AI_PROVIDER=openai bash translate.sh 791"
        ;;

    list)
        run_cli list
        ;;

    add)
        RFC="${2:?'请提供RFC号码，例: bash translate.sh add 791'}"
        echo "正在抓取 RFC ${RFC} ..."
        run_cli add "${RFC}"
        ;;

    translate)
        RFC="${2:?'请提供RFC号码，例: bash translate.sh translate 791'}"
        echo "正在翻译 RFC ${RFC} ..."
        run_cli translate "${RFC}"
        ;;

    status)
        RFC="${2:?'请提供RFC号码'}"
        run_cli status "${RFC}"
        ;;

    batch)
        START="${2:?'请提供起始RFC号码'}"
        END="${3:?'请提供结束RFC号码'}"
        echo "批量翻译 RFC ${START} ~ ${END}"
        FAILED=()
        for rfc in $(seq "${START}" "${END}"); do
            echo ""
            echo "──────────────────────────────"
            echo "  处理 RFC ${rfc}"
            echo "──────────────────────────────"
            # 先抓取，再翻译
            if run_cli add "${rfc}" 2>&1; then
                if run_cli translate "${rfc}" 2>&1; then
                    echo "  [OK] RFC ${rfc} 完成"
                else
                    echo "  [WARN] RFC ${rfc} 翻译失败"
                    FAILED+=("${rfc}")
                fi
            else
                echo "  [WARN] RFC ${rfc} 抓取失败（可能不存在）"
                FAILED+=("${rfc}")
            fi
        done
        echo ""
        echo "批量任务完成"
        if [[ ${#FAILED[@]} -gt 0 ]]; then
            echo "失败的 RFC: ${FAILED[*]}"
        fi
        ;;

    *)
        # 直接传入RFC号码时：抓取 + 翻译
        RFC="${CMD}"
        if [[ "${RFC}" =~ ^[0-9]+$ ]]; then
            echo "正在抓取 RFC ${RFC} ..."
            run_cli add "${RFC}"
            echo ""
            echo "正在翻译 RFC ${RFC} ..."
            run_cli translate "${RFC}"
        else
            echo "[ERROR] 未知命令: ${RFC}"
            echo "运行 'bash translate.sh help' 查看帮助"
            exit 1
        fi
        ;;
esac
