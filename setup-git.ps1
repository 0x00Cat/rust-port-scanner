# Git & GitHub Setup Script for Port Scanner
# This script will guide you through setting up Git and pushing to GitHub

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Port Scanner - Git Setup Assistant  " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Step 1: Check if Git is installed
Write-Host "[Step 1/6] Checking Git installation..." -ForegroundColor Yellow
$gitInstalled = $false
try {
    $gitVersion = git --version 2>$null
    if ($gitVersion) {
        Write-Host "✓ Git is already installed: $gitVersion" -ForegroundColor Green
        $gitInstalled = $true
    }
} catch {
    Write-Host "✗ Git is not installed" -ForegroundColor Red
}

if (-not $gitInstalled) {
    Write-Host ""
    Write-Host "Installing Git..." -ForegroundColor Yellow
    
    # Try winget first
    try {
        winget install --id Git.Git -e --source winget --accept-source-agreements --accept-package-agreements
        Write-Host "✓ Git installed successfully!" -ForegroundColor Green
        Write-Host ""
        Write-Host "⚠️  IMPORTANT: Please close and reopen PowerShell, then run this script again." -ForegroundColor Red
        Write-Host ""
        Read-Host "Press Enter to exit"
        exit
    } catch {
        Write-Host "✗ Failed to install Git automatically" -ForegroundColor Red
        Write-Host ""
        Write-Host "Please install Git manually:" -ForegroundColor Yellow
        Write-Host "1. Visit: https://git-scm.com/download/win" -ForegroundColor Cyan
        Write-Host "2. Download and run the installer" -ForegroundColor Cyan
        Write-Host "3. Use default settings" -ForegroundColor Cyan
        Write-Host "4. Restart PowerShell" -ForegroundColor Cyan
        Write-Host "5. Run this script again" -ForegroundColor Cyan
        Write-Host ""
        Read-Host "Press Enter to exit"
        exit
    }
}

# Step 2: Configure Git
Write-Host ""
Write-Host "[Step 2/6] Configuring Git..." -ForegroundColor Yellow

$userName = git config --global user.name 2>$null
$userEmail = git config --global user.email 2>$null

if (-not $userName) {
    Write-Host ""
    $userName = Read-Host "Enter your name (for Git commits)"
    git config --global user.name "$userName"
    Write-Host "✓ Name configured: $userName" -ForegroundColor Green
} else {
    Write-Host "✓ Name already configured: $userName" -ForegroundColor Green
}

if (-not $userEmail) {
    Write-Host ""
    $userEmail = Read-Host "Enter your email (for Git commits)"
    git config --global user.email "$userEmail"
    Write-Host "✓ Email configured: $userEmail" -ForegroundColor Green
} else {
    Write-Host "✓ Email already configured: $userEmail" -ForegroundColor Green
}

# Set default branch to main
git config --global init.defaultBranch main 2>$null
Write-Host "✓ Default branch set to 'main'" -ForegroundColor Green

# Step 3: Initialize repository
Write-Host ""
Write-Host "[Step 3/6] Initializing Git repository..." -ForegroundColor Yellow

$projectPath = "C:\Rust\Hello World"
Set-Location $projectPath

if (Test-Path ".git") {
    Write-Host "✓ Git repository already initialized" -ForegroundColor Green
} else {
    git init
    Write-Host "✓ Git repository initialized" -ForegroundColor Green
}

# Step 4: Check files to commit
Write-Host ""
Write-Host "[Step 4/6] Checking files to commit..." -ForegroundColor Yellow

$status = git status --short
if ($status) {
    $fileCount = ($status | Measure-Object).Count
    Write-Host "✓ Found $fileCount files to commit" -ForegroundColor Green
    Write-Host ""
    Write-Host "Files to be committed:" -ForegroundColor Cyan
    git status --short | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
} else {
    Write-Host "✓ Working directory is clean" -ForegroundColor Green
}

# Step 5: Commit files
Write-Host ""
Write-Host "[Step 5/6] Creating initial commit..." -ForegroundColor Yellow

# Check if there are uncommitted changes
$uncommitted = git status --porcelain
if ($uncommitted) {
    git add .
    git commit -m "Initial commit: Rust port scanner v2.0.0 with clean architecture"
    Write-Host "✓ Initial commit created" -ForegroundColor Green
} else {
    Write-Host "✓ No changes to commit (already committed)" -ForegroundColor Green
}

# Step 6: Set up GitHub remote
Write-Host ""
Write-Host "[Step 6/6] Setting up GitHub remote..." -ForegroundColor Yellow
Write-Host ""
Write-Host "To push to GitHub, you need to:" -ForegroundColor Cyan
Write-Host "1. Create a new repository on GitHub:" -ForegroundColor White
Write-Host "   https://github.com/new" -ForegroundColor Blue
Write-Host ""
Write-Host "2. Repository settings:" -ForegroundColor White
Write-Host "   - Name: rust-port-scanner (or your choice)" -ForegroundColor Gray
Write-Host "   - Description: A modern, modular port scanner written in Rust" -ForegroundColor Gray
Write-Host "   - Public or Private: Your choice" -ForegroundColor Gray
Write-Host "   - DO NOT initialize with README, .gitignore, or license" -ForegroundColor Gray
Write-Host ""

$setupRemote = Read-Host "Have you created the GitHub repository? (y/n)"

if ($setupRemote -eq 'y' -or $setupRemote -eq 'Y') {
    Write-Host ""
    $repoUrl = Read-Host "Enter your GitHub repository URL (e.g., https://github.com/username/rust-port-scanner.git)"
    
    # Check if remote already exists
    $existingRemote = git remote get-url origin 2>$null
    if ($existingRemote) {
        Write-Host "⚠️  Remote 'origin' already exists: $existingRemote" -ForegroundColor Yellow
        $updateRemote = Read-Host "Update to new URL? (y/n)"
        if ($updateRemote -eq 'y' -or $updateRemote -eq 'Y') {
            git remote set-url origin $repoUrl
            Write-Host "✓ Remote updated" -ForegroundColor Green
        }
    } else {
        git remote add origin $repoUrl
        Write-Host "✓ Remote added: $repoUrl" -ForegroundColor Green
    }
    
    # Rename branch to main
    $currentBranch = git branch --show-current
    if ($currentBranch -ne "main") {
        git branch -M main
        Write-Host "✓ Branch renamed to 'main'" -ForegroundColor Green
    }
    
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  Ready to Push!                       " -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Run this command to push to GitHub:" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  git push -u origin main" -ForegroundColor Green
    Write-Host ""
    Write-Host "When prompted for credentials:" -ForegroundColor Yellow
    Write-Host "  - Username: Your GitHub username" -ForegroundColor Gray
    Write-Host "  - Password: Use a Personal Access Token (NOT your password)" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Get token at: https://github.com/settings/tokens" -ForegroundColor Blue
    Write-Host "  - Click 'Generate new token (classic)'" -ForegroundColor Gray
    Write-Host "  - Select 'repo' scope" -ForegroundColor Gray
    Write-Host "  - Copy and paste when prompted for password" -ForegroundColor Gray
    Write-Host ""
    
    $pushNow = Read-Host "Push to GitHub now? (y/n)"
    if ($pushNow -eq 'y' -or $pushNow -eq 'Y') {
        Write-Host ""
        Write-Host "Pushing to GitHub..." -ForegroundColor Yellow
        git push -u origin main
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host ""
            Write-Host "========================================" -ForegroundColor Green
            Write-Host "  SUCCESS! 🎉                          " -ForegroundColor Green
            Write-Host "========================================" -ForegroundColor Green
            Write-Host ""
            Write-Host "Your code is now on GitHub!" -ForegroundColor Green
            Write-Host ""
            Write-Host "Next steps:" -ForegroundColor Yellow
            Write-Host "1. Visit your repository: $repoUrl" -ForegroundColor Cyan
            Write-Host "2. Update README.md badges with your username" -ForegroundColor Cyan
            Write-Host "3. Set up GitHub Actions secrets (see CI-CD-GUIDE.md)" -ForegroundColor Cyan
            Write-Host "4. Create first release tag: git tag v2.0.0 && git push origin v2.0.0" -ForegroundColor Cyan
            Write-Host ""
        } else {
            Write-Host ""
            Write-Host "⚠️  Push failed. Common solutions:" -ForegroundColor Red
            Write-Host "  - Make sure you're using a Personal Access Token, not password" -ForegroundColor Yellow
            Write-Host "  - Check that the repository URL is correct" -ForegroundColor Yellow
            Write-Host "  - Try: git push -u origin main --verbose" -ForegroundColor Yellow
            Write-Host ""
        }
    }
} else {
    Write-Host ""
    Write-Host "No problem! When you're ready:" -ForegroundColor Yellow
    Write-Host "1. Create repository on GitHub" -ForegroundColor Cyan
    Write-Host "2. Run this script again" -ForegroundColor Cyan
    Write-Host ""
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "For more help, see:" -ForegroundColor White
Write-Host "  - GIT-GITHUB-QUICKSTART.md" -ForegroundColor Blue
Write-Host "  - GIT-SETUP-GUIDE.md" -ForegroundColor Blue
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

Read-Host "Press Enter to exit"
