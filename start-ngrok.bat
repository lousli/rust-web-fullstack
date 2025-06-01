@echo off
chcp 65001 >nul 2>&1
echo 🌐 启动 ngrok 隧道服务...
echo 🔗 将本地 8080 端口暴露到公网
echo.
echo 请确保你的 Rust 后端服务已经在 8080 端口运行
echo 如果还没有启动，请先运行: cargo run
echo.
pause
echo 启动 ngrok...
E:\work\work1\ngrok.exe http 8080
echo    ./ngrok http 8080