# GitHub Token 设置指南

## 1. 创建 GitHub Personal Access Token

### 步骤：
1. 登录 GitHub
2. 访问：**Settings** → **Developer settings** → **Personal access tokens** → **Tokens (classic)**
3. 点击 **"Generate new token"** → **"Generate new token (classic)"**
4. 填写信息：
   - **Note**: `图表项目开发` (或其他描述性名称)
   - **Expiration**: 选择合适的过期时间
   - **Select scopes**: 勾选以下权限：
     - ✅ `repo` (完整的仓库访问权限)
     - ✅ `workflow` (如果需要 GitHub Actions)
     - ✅ `write:packages` (如果需要发布包)

5. 点击 **"Generate token"**
6. **重要**: 立即复制生成的 token（只显示一次！）

## 2. 在本地配置 Token

### 方法一：使用 Git 凭据管理器（推荐）
```powershell
# Windows 凭据管理器会自动存储 token
git config --global credential.helper manager-core
```

### 方法二：设置环境变量（临时）
```powershell
# 在 PowerShell 中设置临时环境变量
$env:GITHUB_TOKEN = "your_token_here"
```

### 方法三：配置 Git 凭据（持久）
```powershell
# 配置 Git 使用 token 作为密码
git config --global user.name "Your Name"
git config --global user.email "your-email@example.com"
```

## 3. 连接到 GitHub 仓库

### 创建新的 GitHub 仓库后：
```powershell
# 添加远程仓库
git remote add origin https://github.com/username/repository-name.git

# 第一次推送（会提示输入凭据）
git push -u origin master
```

### 输入凭据时：
- **用户名**: 您的 GitHub 用户名
- **密码**: 输入您刚才生成的 Personal Access Token（不是您的 GitHub 密码）

## 4. 验证连接

```powershell
# 检查远程仓库配置
git remote -v

# 测试连接
git fetch origin
```

## 5. 安全建议

- ✅ 定期更新 Token
- ✅ 只授予必要的权限
- ✅ 不要在代码中硬编码 Token
- ✅ 如果 Token 泄露，立即撤销并重新生成

## 故障排除

### 如果推送失败：
```powershell
# 检查 Git 配置
git config --list

# 清除缓存的凭据
git config --global --unset credential.helper
git config --global credential.helper manager-core
```

### 如果仍有问题：
```powershell
# 使用 HTTPS URL with token
git remote set-url origin https://your-token@github.com/username/repo.git
```

---

**注意**: 当您的网络恢复正常后，就可以按照这个指南来设置 GitHub 连接了。
