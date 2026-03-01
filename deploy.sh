#!/bin/bash
# RFC2CN Docker Deployment Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
COMPOSE_FILE="docker-compose.prod.yml"
PROJECT_NAME="rfc2cn"

# Functions
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is installed
check_docker() {
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
        print_error "Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi
}

# Build all services
build() {
    print_info "Building Docker images..."
    docker-compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" build
    print_info "Build completed successfully!"
}

# Start all services
up() {
    print_info "Starting services..."
    docker-compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" up -d
    print_info "Services started successfully!"
    print_info "Frontend: http://localhost:3000"
    print_info "Backend API: http://localhost:8080"
    print_info "Ollama: http://localhost:11434"
}

# Stop all services
down() {
    print_info "Stopping services..."
    docker-compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" down
    print_info "Services stopped successfully!"
}

# Restart all services
restart() {
    print_info "Restarting services..."
    down
    up
    print_info "Services restarted successfully!"
}

# View logs
logs() {
    SERVICE=${1:-}
    if [ -z "$SERVICE" ]; then
        docker-compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" logs -f
    else
        docker-compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" logs -f "$SERVICE"
    fi
}

# Check service status
status() {
    docker-compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" ps
}

# Clean up everything (including volumes)
clean() {
    print_warn "This will remove all containers, networks, and volumes."
    read -p "Are you sure? (yes/no): " confirm
    if [ "$confirm" = "yes" ]; then
        print_info "Cleaning up..."
        docker-compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" down -v
        print_info "Cleanup completed!"
    else
        print_info "Cleanup cancelled."
    fi
}

# Initialize Ollama with qwen3:8b model
init_ollama() {
    print_info "Initializing Ollama with qwen3:8b model..."
    docker exec rfc2cn-ollama ollama pull qwen3:8b
    print_info "Ollama initialized successfully!"
}

# Run CLI commands inside backend container
cli() {
    if [ $# -lt 1 ]; then
        print_error "Usage: $0 cli <command> [args...]"
        print_info "Example: $0 cli sync --start 1 --end 100"
        exit 1
    fi
    docker exec -it rfc2cn-backend rfc-cli "$@"
}

# Show help
show_help() {
    cat << EOF
RFC2CN Docker Deployment Script

Usage: $0 [command] [options]

Commands:
    build       Build all Docker images
    up          Start all services
    down        Stop all services
    restart     Restart all services
    logs        View logs (optionally specify service: backend, frontend, postgres, ollama)
    status      Show service status
    clean       Remove all containers, networks, and volumes
    init-ollama Initialize Ollama with qwen3:8b model
    cli         Run CLI commands inside backend container
    help        Show this help message

Examples:
    $0 build                    # Build all images
    $0 up                       # Start all services
    $0 logs backend             # View backend logs
    $0 cli sync --start 1       # Sync RFCs starting from 1
    $0 cli list                 # List all RFCs
    $0 clean                    # Clean up everything

EOF
}

# Main script
main() {
    check_docker
    
    case "${1:-help}" in
        build)
            build
            ;;
        up)
            up
            ;;
        down)
            down
            ;;
        restart)
            restart
            ;;
        logs)
            logs "${2:-}"
            ;;
        status)
            status
            ;;
        clean)
            clean
            ;;
        init-ollama)
            init_ollama
            ;;
        cli)
            shift
            cli "$@"
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "Unknown command: $1"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
