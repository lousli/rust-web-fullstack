#!/bin/bash
# 医生投放评价系统 - 生产环境部署脚本

set -e

echo "🚀 开始部署医生投放评价系统..."

# 配置变量
APP_NAME="doctor-analysis"
VERSION=$(grep version Cargo.toml | head -1 | cut -d'"' -f2)
BUILD_TIME=$(date '+%Y%m%d_%H%M%S')
BACKUP_DIR="./backups"

# 颜色输出函数
print_info() {
    echo -e "\033[36m[INFO]\033[0m $1"
}

print_success() {
    echo -e "\033[32m[SUCCESS]\033[0m $1"
}

print_warning() {
    echo -e "\033[33m[WARNING]\033[0m $1"
}

print_error() {
    echo -e "\033[31m[ERROR]\033[0m $1"
}

# 检查依赖
check_dependencies() {
    print_info "检查部署依赖..."
    
    if ! command -v docker &> /dev/null; then
        print_error "Docker 未安装或不在 PATH 中"
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose 未安装或不在 PATH 中"
        exit 1
    fi
    
    print_success "依赖检查通过"
}

# 备份数据
backup_data() {
    print_info "备份现有数据..."
    
    mkdir -p $BACKUP_DIR
    
    if [ -f "./data/doctor_analysis.db" ]; then
        cp "./data/doctor_analysis.db" "$BACKUP_DIR/doctor_analysis_$BUILD_TIME.db"
        print_success "数据库备份完成: $BACKUP_DIR/doctor_analysis_$BUILD_TIME.db"
    else
        print_warning "未找到现有数据库文件"
    fi
    
    # 保留最近 10 个备份
    ls -t $BACKUP_DIR/doctor_analysis_*.db 2>/dev/null | tail -n +11 | xargs -r rm
}

# 构建应用
build_app() {
    print_info "构建应用镜像..."
    
    # 创建必要的目录
    mkdir -p data logs
    
    # 构建 Docker 镜像
    docker build -t $APP_NAME:$VERSION -t $APP_NAME:latest .
    
    print_success "应用构建完成"
}

# 部署应用
deploy_app() {
    print_info "部署应用..."
    
    # 停止旧版本
    if docker-compose ps | grep -q $APP_NAME; then
        print_info "停止旧版本服务..."
        docker-compose down --timeout 30
    fi
    
    # 启动新版本
    docker-compose up -d
    
    # 等待服务启动
    print_info "等待服务启动..."
    sleep 10
    
    # 健康检查
    for i in {1..30}; do
        if curl -f http://localhost:8081/api/health >/dev/null 2>&1; then
            print_success "服务启动成功！"
            break
        fi
        
        if [ $i -eq 30 ]; then
            print_error "服务启动超时"
            docker-compose logs
            exit 1
        fi
        
        sleep 2
        echo -n "."
    done
}

# 验证部署
verify_deployment() {
    print_info "验证部署状态..."
    
    # 检查服务状态
    if ! docker-compose ps | grep -q "Up"; then
        print_error "服务未正常运行"
        docker-compose logs
        exit 1
    fi
    
    # 检查健康状态
    HEALTH_STATUS=$(curl -s http://localhost:8081/api/monitoring/health | jq -r '.status' 2>/dev/null || echo "unknown")
    
    if [ "$HEALTH_STATUS" = "healthy" ]; then
        print_success "健康检查通过"
    else
        print_warning "健康检查失败，状态: $HEALTH_STATUS"
    fi
    
    # 显示服务信息
    echo ""
    echo "📊 部署信息:"
    echo "   版本: $VERSION"
    echo "   构建时间: $BUILD_TIME"
    echo "   本地访问: http://localhost:8081"
    echo "   健康检查: http://localhost:8081/api/health"
    echo "   详细监控: http://localhost:8081/api/monitoring/health/detailed"
    echo ""
    
    # 显示容器状态
    print_info "容器状态:"
    docker-compose ps
}

# 清理旧镜像
cleanup() {
    print_info "清理旧镜像..."
    
    # 删除悬空镜像
    docker image prune -f >/dev/null 2>&1 || true
    
    # 保留最近 3 个版本的镜像
    OLD_IMAGES=$(docker images $APP_NAME --format "table {{.Tag}}" | grep -v "latest\|TAG\|$VERSION" | tail -n +4)
    if [ ! -z "$OLD_IMAGES" ]; then
        echo "$OLD_IMAGES" | xargs -I {} docker rmi $APP_NAME:{} 2>/dev/null || true
    fi
    
    print_success "清理完成"
}

# 主函数
main() {
    echo "医生投放评价系统部署脚本 v$VERSION"
    echo "=========================================="
    
    check_dependencies
    backup_data
    build_app
    deploy_app
    verify_deployment
    cleanup
    
    print_success "🎉 部署完成！"
    echo ""
    echo "📝 接下来的步骤:"
    echo "   1. 访问 http://localhost:8081 测试应用"
    echo "   2. 检查日志: docker-compose logs -f"
    echo "   3. 监控状态: docker-compose ps"
    echo ""
}

# 处理命令行参数
case "${1:-deploy}" in
    "deploy")
        main
        ;;
    "stop")
        print_info "停止服务..."
        docker-compose down
        print_success "服务已停止"
        ;;
    "restart")
        print_info "重启服务..."
        docker-compose restart
        print_success "服务已重启"
        ;;
    "logs")
        docker-compose logs -f
        ;;
    "status")
        docker-compose ps
        ;;
    "health")
        curl -s http://localhost:8081/api/monitoring/health/detailed | jq '.' 2>/dev/null || curl -s http://localhost:8081/api/health
        ;;
    *)
        echo "用法: $0 [deploy|stop|restart|logs|status|health]"
        echo ""
        echo "命令说明:"
        echo "  deploy  - 部署应用 (默认)"
        echo "  stop    - 停止服务"
        echo "  restart - 重启服务"
        echo "  logs    - 查看日志"
        echo "  status  - 查看状态"
        echo "  health  - 健康检查"
        exit 1
        ;;
esac
