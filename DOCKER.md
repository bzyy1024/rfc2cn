# Docker Deployment Guide

## 🚀 Quick Start

### Prerequisites

- Docker (version 20.10+)
- Docker Compose (version 2.0+)
- At least 4GB free disk space
- At least 2GB free RAM

### Step 1: Clone and Build

```bash
# Clone the repository
git clone <repository-url>
cd rfc2cn

# Build all Docker images
./deploy.sh build
```

This will build:
- `rfc2cn-backend`: Rust backend service with API and CLI tools
- `rfc2cn-frontend`: Next.js frontend with standalone mode
- `rfc2cn-postgres`: PostgreSQL 16 database
- `rfc2cn-ollama`: Ollama AI service for translation

### Step 2: Start Services

```bash
# Start all services
./deploy.sh up
```

Services will be available at:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
- Ollama API: http://localhost:11434

### Step 3: Initialize (Optional)

```bash
# Pull Ollama model for translation
./deploy.sh init-ollama

# Sync initial RFC data (e.g., RFC 1-100)
./deploy.sh cli sync --start 1 --end 100
```

## 📋 Service Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Docker Network                        │
│                    (rfc2cn-network)                      │
│                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Frontend   │  │   Backend    │  │  PostgreSQL  │  │
│  │  (Next.js)   │─→│   (Rust)     │─→│   Database   │  │
│  │   Port 3000  │  │   Port 8080  │  │   Port 5432  │  │
│  └──────────────┘  └──────┬───────┘  └──────────────┘  │
│                           │                              │
│                           ↓                              │
│                   ┌──────────────┐                       │
│                   │    Ollama    │                       │
│                   │  (AI Model)  │                       │
│                   │  Port 11434  │                       │
│                   └──────────────┘                       │
└─────────────────────────────────────────────────────────┘
```

## 🛠️ Management Commands

### Service Control

```bash
# View service status
./deploy.sh status

# Stop all services
./deploy.sh down

# Restart all services
./deploy.sh restart
```

### Logs

```bash
# View all logs
./deploy.sh logs

# View specific service logs
./deploy.sh logs backend
./deploy.sh logs frontend
./deploy.sh logs postgres
./deploy.sh logs ollama

# Follow logs in real-time (Ctrl+C to exit)
./deploy.sh logs backend
```

### CLI Tool Access

The backend container includes the `rfc-cli` tool:

```bash
# List all RFCs
./deploy.sh cli list

# Add a new RFC
./deploy.sh cli add 8446 --tags tls,security

# Translate a specific RFC
./deploy.sh cli translate 8446

# Sync RFC range with auto-translation
./deploy.sh cli sync --start 1 --end 100 --concurrent 3

# Check Ollama status
./deploy.sh cli check-ollama

# Manage tags
./deploy.sh cli tags
./deploy.sh cli tag 8446 cryptography
```

### Database Management

```bash
# Access PostgreSQL shell
docker exec -it rfc2cn-postgres psql -U rfc2cn -d rfc2cn

# Backup database
docker exec rfc2cn-postgres pg_dump -U rfc2cn rfc2cn > backup.sql

# Restore database
cat backup.sql | docker exec -i rfc2cn-postgres psql -U rfc2cn rfc2cn
```

## 🧹 Cleanup

```bash
# Stop and remove containers (keeps data)
./deploy.sh down

# Remove everything including volumes (⚠️ DELETES ALL DATA)
./deploy.sh clean
```

## 🔧 Configuration

### Environment Variables

Edit `docker-compose.prod.yml` to customize:

```yaml
# Backend configuration
environment:
  DATABASE_URL: ${DATABASE_URL}  # Set in .env file
  RUST_LOG: info  # Change to 'debug' for more logs
  SERVER_HOST: 0.0.0.0
  SERVER_PORT: 8080
  OLLAMA_URL: http://ollama:11434

# Frontend configuration
environment:
  NEXT_PUBLIC_API_URL: http://localhost:8080  # Change for production
```

### Port Mapping

To change exposed ports, edit `docker-compose.prod.yml`:

```yaml
services:
  frontend:
    ports:
      - "8080:3000"  # Change 8080 to your desired port
  
  backend:
    ports:
      - "3001:8080"  # Change 3001 to your desired port
```

### Volume Persistence

Data is stored in Docker volumes:

```bash
# List volumes
docker volume ls | grep rfc2cn

# Inspect volume
docker volume inspect rfc2cn_postgres_data
docker volume inspect rfc2cn_ollama_data
```

## 🚦 Health Checks

All services include health checks:

```bash
# Check backend health
curl http://localhost:8080/api/health

# Check database
docker exec rfc2cn-postgres pg_isready -U rfc2cn -d rfc2cn

# Check Ollama
curl http://localhost:11434/api/tags
```

## 🐛 Troubleshooting

### Services won't start

```bash
# Check Docker status
docker ps -a

# View service logs
./deploy.sh logs

# Restart services
./deploy.sh restart
```

### Database connection issues

```bash
# Check database is ready
docker exec rfc2cn-postgres pg_isready -U rfc2cn -d rfc2cn

# Verify connection string
docker exec rfc2cn-backend env | grep DATABASE_URL
```

### Ollama not responding

```bash
# Check Ollama service
docker exec rfc2cn-ollama ollama list

# Pull model manually
docker exec rfc2cn-ollama ollama pull qwen3:8b

# View Ollama logs
./deploy.sh logs ollama
```

### Port already in use

```bash
# Check what's using the port
sudo lsof -i :8080
sudo lsof -i :3000

# Either kill the process or change port in docker-compose.prod.yml
```

## 🔐 Production Deployment

### Security Best Practices

1. **Change default credentials** in `docker-compose.prod.yml`:
   ```yaml
   POSTGRES_PASSWORD: your-secure-password
   ```

2. **Use environment files** for sensitive data:
   ```bash
   # Create .env.prod
   echo "POSTGRES_PASSWORD=your-secure-password" > .env.prod
   
   # Reference in docker-compose.prod.yml
   env_file:
     - .env.prod
   ```

3. **Enable HTTPS** with reverse proxy (nginx/traefik):
   ```nginx
   server {
       listen 443 ssl;
       server_name rfc2cn.example.com;
       
       location / {
           proxy_pass http://localhost:3000;
       }
       
       location /api {
           proxy_pass http://localhost:8080;
       }
   }
   ```

4. **Limit resource usage**:
   ```yaml
   services:
     backend:
       deploy:
         resources:
           limits:
             cpus: '2'
             memory: 2G
   ```

### Scaling

To run multiple backend instances:

```yaml
services:
  backend:
    deploy:
      replicas: 3
```

Add a load balancer (nginx) in front of backend services.

## 📊 Monitoring

### View resource usage

```bash
# CPU and memory usage
docker stats rfc2cn-backend rfc2cn-frontend rfc2cn-postgres

# Disk usage
docker system df
```

### Export metrics

Consider adding Prometheus and Grafana for production monitoring.

## 🔄 Updates

```bash
# Pull latest changes
git pull

# Rebuild and restart
./deploy.sh build
./deploy.sh restart
```

## 📝 Maintenance

### Regular Tasks

- **Backup database** daily/weekly
- **Monitor logs** for errors
- **Check disk space** (logs and database can grow)
- **Update Ollama models** periodically
- **Review security updates** for base images

### Cleanup Old Data

```bash
# Remove unused Docker images
docker image prune -a

# Remove unused volumes (⚠️ careful!)
docker volume prune
```
