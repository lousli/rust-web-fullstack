# ngrok 启动脚本 (PowerShell 版本)
Write-Host "🌐 启动 ngrok 隧道服务..." -ForegroundColor Green
Write-Host "🔗 将本地 8080 端口暴露到公网" -ForegroundColor Green
Write-Host ""
Write-Host "请确保你的 Rust 后端服务已经在 8080 端口运行" -ForegroundColor Yellow
Write-Host "如果还没有启动，请先运行: cargo run" -ForegroundColor Yellow
Write-Host ""
Read-Host "按 Enter 键继续启动 ngrok"
Write-Host "启动 ngrok..." -ForegroundColor Green
& "E:\work\work1\ngrok.exe" http 8080
