# ngrok 远程访问配置指南

## 什么是 ngrok？
ngrok 是一个反向代理工具，可以将你的本地开发服务器暴露到公网，让远程同事通过一个公网 URL 访问你的项目。

## 使用步骤

### 1. 启动 Rust 后端服务
首先确保你的后端服务正在运行：

```bash
cd rust-web-backend
cargo run
```

服务启动后会显示：
```
🚀 启动医生数据分析系统后端服务器...
📊 本地访问: http://127.0.0.1:8080
```

### 2. 启动 ngrok 隧道

#### 方法一：使用批处理文件 (推荐)
双击运行 `start-ngrok.bat` 文件

#### 方法二：使用 PowerShell 脚本
右键以 PowerShell 运行 `start-ngrok.ps1` 文件

#### 方法三：手动运行
打开新的命令行窗口，运行：
```bash
E:\work\work1\ngrok.exe http 8080
```

### 3. 获取公网访问地址

ngrok 启动后会显示类似以下信息：
```
ngrok                                                          
                                                               
Visit http://localhost:4040 for complete traffic inspection    
                                                               
Session Status                online                           
Account                       your-account (Plan: Free)        
Version                       3.x.x                           
Region                        United States (us)               
Latency                       45ms                             
Web Interface                 http://127.0.0.1:4040           
Forwarding                    https://xxxx-xxx-xxx-xxx.ngrok-free.app -> http://localhost:8080

Connections                   ttl     opn     rt1     rt5     p50     p90     
                              0       0       0.00    0.00    0.00    0.00    
```

**重要**：复制 `Forwarding` 行中的 HTTPS 地址（如：`https://xxxx-xxx-xxx-xxx.ngrok-free.app`），这就是你同事可以访问的公网地址！

### 4. 分享给同事

将获得的 ngrok URL 发送给你的同事，他们就可以通过这个地址访问你的项目了：
- 前端页面：`https://xxxx-xxx-xxx-xxx.ngrok-free.app`
- API 接口：`https://xxxx-xxx-xxx-xxx.ngrok-free.app/api/`

## 注意事项

### 安全提醒
1. **仅用于开发测试**：ngrok 免费版会暴露你的本地服务到公网，请勿在生产环境使用
2. **临时地址**：每次重启 ngrok 会生成新的随机地址
3. **访问限制**：免费版有连接数和带宽限制

### 防火墙配置
如果遇到连接问题，确保：
1. Windows 防火墙允许 ngrok.exe 和你的 Rust 应用通过
2. 杀毒软件没有阻止 ngrok 运行

### 性能优化
1. **保持服务运行**：不要关闭运行 Rust 服务的命令行窗口
2. **网络稳定性**：确保你的网络连接稳定，ngrok 断线会导致外部无法访问

## 高级配置（可选）

### 注册 ngrok 账户
1. 访问 https://ngrok.com/ 注册免费账户
2. 获取 authtoken
3. 运行：`E:\work\work1\ngrok.exe config add-authtoken YOUR_TOKEN`

### 固定子域名（付费功能）
注册账户后可以使用固定的子域名：
```bash
E:\work\work1\ngrok.exe http 8080 --subdomain=myproject
```

## 故障排除

### 常见错误及解决方案

1. **"bind: address already in use"**
   - 检查端口 8080 是否被其他程序占用
   - 确保只有一个 Rust 服务实例在运行

2. **"connection refused"**
   - 确保 Rust 后端服务正在运行
   - 检查服务是否正确监听在 0.0.0.0:8080

3. **ngrok 连接失败**
   - 检查网络连接
   - 尝试重启 ngrok
   - 检查防火墙设置

4. **同事无法访问**
   - 确认分享的是完整的 HTTPS URL
   - 检查 ngrok 是否还在运行
   - 确认后端服务没有停止

## 使用流程总结

1. 📦 启动 Rust 后端：`cargo run`
2. 🌐 启动 ngrok：运行 `start-ngrok.bat`
3. 📋 复制公网地址：从 ngrok 输出中复制 HTTPS URL
4. 📤 分享给同事：将 URL 发送给需要访问的人
5. ✅ 测试访问：让同事确认可以正常访问

**记住**：每次重启 ngrok 都会获得新的随机地址，需要重新分享给同事！
