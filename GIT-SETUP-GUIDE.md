# Setting Up Git and GitHub for Port Scanner

## ⚠️ Git Not Installed

Git is not currently installed on your system. Follow the steps below to install Git and push your project to GitHub.

---

## 📥 Step 1: Install Git

### Option 1: Using winget (Windows Package Manager)
```powershell
winget install --id Git.Git -e --source winget
```

### Option 2: Download Git Installer
1. Visit: https://git-scm.com/download/win
2. Download the installer (64-bit recommended)
3. Run the installer with default settings
4. Restart your terminal after installation

### Verify Installation
After installing, close and reopen PowerShell, then run:
```powershell
git --version
```

You should see something like: `git version 2.42.0.windows.1`

---

## 🔧 Step 2: Configure Git

Set up your identity (required for commits):

```powershell
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

Optional but recommended settings:
```powershell
git config --global init.defaultBranch main
git config --global core.autocrlf true
```

---

## 🌐 Step 3: Create GitHub Repository

### A. Via GitHub Website (Easiest)

1. **Go to GitHub**: https://github.com/new
2. **Repository details**:
   - **Name**: `rust-port-scanner` (or your preferred name)
   - **Description**: "A modern, modular port scanner written in Rust"
   - **Visibility**: Public or Private (your choice)
   - **DO NOT** initialize with README, .gitignore, or license (we already have these)
3. **Click**: "Create repository"
4. **Copy the repository URL** shown (e.g., `https://github.com/yourusername/rust-port-scanner.git`)

### B. Via GitHub CLI (Alternative)

Install GitHub CLI:
```powershell
winget install --id GitHub.cli
```

Then create repo:
```powershell
gh repo create rust-port-scanner --public --source=. --remote=origin
```

---

## 📤 Step 4: Initialize and Push to GitHub

### Navigate to your project directory:
```powershell
cd "C:\Rust\Hello World"
```

### Initialize Git repository:
```powershell
git init
```

### Add all files:
```powershell
git add .
```

### Create initial commit:
```powershell
git commit -m "Initial commit: Rust port scanner v2.0.0 with clean architecture"
```

### Add remote repository:
Replace `yourusername` with your actual GitHub username:
```powershell
git remote add origin https://github.com/yourusername/rust-port-scanner.git
```

### Rename branch to main (if needed):
```powershell
git branch -M main
```

### Push to GitHub:
```powershell
git push -u origin main
```

---

## 🔐 Step 5: Authentication

When you push for the first time, you'll need to authenticate.

### Option A: Personal Access Token (Recommended)

1. **Go to**: https://github.com/settings/tokens
2. **Click**: "Generate new token" → "Generate new token (classic)"
3. **Settings**:
   - **Note**: "Port Scanner Development"
   - **Expiration**: 90 days (or your preference)
   - **Scopes**: Select `repo` (full control of private repositories)
4. **Generate** and **copy the token** (save it securely!)
5. **When prompted for password during push**: Paste the token

### Option B: GitHub CLI
```powershell
gh auth login
```
Follow the prompts to authenticate through your browser.

---

## 📋 Step 6: Verify Upload

1. Go to your repository: `https://github.com/yourusername/rust-port-scanner`
2. You should see all your files
3. Check that workflows are visible in the "Actions" tab

---

## 🚀 Step 7: Set Up CI/CD Secrets

For the GitHub Actions workflows to work, add these secrets:

1. **Go to**: Repository → Settings → Secrets and variables → Actions
2. **Click**: "New repository secret"
3. **Add each secret**:

| Secret Name | Purpose | Where to Get |
|-------------|---------|--------------|
| `CODECOV_TOKEN` | Code coverage | https://codecov.io (after signing in with GitHub) |
| `CARGO_TOKEN` | Publish to crates.io | https://crates.io/settings/tokens |
| `DOCKER_USERNAME` | Docker Hub username | Your Docker Hub username |
| `DOCKER_PASSWORD` | Docker Hub token | https://hub.docker.com/settings/security |

---

## 🏷️ Step 8: Create First Release

To trigger the full CI/CD pipeline including releases:

```powershell
# Tag the current commit
git tag -a v2.0.0 -m "Release v2.0.0: Production-ready port scanner"

# Push the tag
git push origin v2.0.0
```

This will automatically:
- Run all tests on multiple platforms
- Build release binaries for 5 platforms
- Create a GitHub Release with downloadable binaries
- Publish to crates.io (if token is set)
- Build and push Docker images (if credentials are set)

---

## 📝 Quick Command Reference

### Daily Git Workflow

```powershell
# Check status
git status

# Add changes
git add .

# Commit
git commit -m "Your commit message"

# Push to GitHub
git push

# Pull latest changes
git pull

# Create new branch
git checkout -b feature/new-feature

# Switch branches
git checkout main

# Merge branch
git merge feature/new-feature
```

### Creating Releases

```powershell
# Update version in Cargo.toml first
# Then:
git add Cargo.toml
git commit -m "Bump version to 2.1.0"
git tag v2.1.0
git push origin main
git push origin v2.1.0
```

---

## 🎯 Complete Setup Script

Here's a complete PowerShell script to do everything at once:

```powershell
# Navigate to project
cd "C:\Rust\Hello World"

# Initialize Git
git init

# Configure Git (update with your details)
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# Add all files
git add .

# Initial commit
git commit -m "Initial commit: Rust port scanner v2.0.0"

# Add remote (replace 'yourusername')
git remote add origin https://github.com/yourusername/rust-port-scanner.git

# Rename to main branch
git branch -M main

# Push to GitHub
git push -u origin main

# Create and push first tag
git tag -a v2.0.0 -m "Release v2.0.0"
git push origin v2.0.0
```

---

## ✅ Success Checklist

- [ ] Git installed and configured
- [ ] GitHub account created
- [ ] Repository created on GitHub
- [ ] Local repository initialized
- [ ] Files committed locally
- [ ] Remote added and pushed
- [ ] Repository visible on GitHub
- [ ] Actions tab showing workflows
- [ ] Secrets configured (optional but recommended)
- [ ] First release tag created (optional)

---

## 🆘 Troubleshooting

### "git: command not found"
- Restart your terminal after installing Git
- Check installation: `git --version`

### Authentication Failed
- Use Personal Access Token, not password
- Make sure token has `repo` scope

### Push Rejected
```powershell
git pull origin main --rebase
git push origin main
```

### Large Files Warning
- Check .gitignore includes `/target/`
- Remove target directory: `git rm -r --cached target`

---

## 📚 Additional Resources

- [Git Documentation](https://git-scm.com/doc)
- [GitHub Guides](https://guides.github.com/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust Book](https://doc.rust-lang.org/book/)

---

## 🎉 Next Steps After Push

1. **Update README badges**: Replace `yourusername` with your actual username
2. **Enable GitHub Pages** (optional): For documentation hosting
3. **Set up branch protection**: Settings → Branches → Add rule for `main`
4. **Add collaborators**: Settings → Collaborators
5. **Create issues and milestones**: For project management
6. **Star your own repo**: Why not? 😄

---

**Need Help?** Check the [Contributing Guide](CONTRIBUTING.md) or open an issue!
