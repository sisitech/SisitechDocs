# Git and GitHub Notes

## Introduction

### What is Git?
- Git is a **distributed version control system** used to track changes in source code.
- It allows **multiple developers** to collaborate on a project efficiently.
- Supports **branching, merging**, and **version history**.

### What is GitHub?
- GitHub is a **web-based platform** that hosts Git repositories.
- It provides tools for collaboration, such as:
  - Pull Requests
  - Issues
  - Project boards
  - GitHub Actions (CI/CD)

---

## Git Workflow Basics

### Initial Setup
```bash
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
```

### Starting a Repository
```bash
git init                   # Initialize a new Git repo
git clone <repo-url>       # Clone an existing repo
```

### Staging and Committing Changes
```bash
git status                 # View current changes
git add <file>             # Stage a file
git add .                  # Stage all changes
git commit -m "message"    # Commit staged changes
```

### Checking History
```bash
git log                    # View commit history
git log --oneline          # Compact view
git diff                   # Show changes
```

---

## Branching and Merging

### Working with Branches
```bash
git branch                 # List branches
git branch <name>          # Create a branch
git checkout <name>        # Switch to a branch
git checkout -b <name>     # Create and switch
```

### Merging Branches
```bash
git merge <branch>         # Merge into current branch
git branch -d <branch>     # Delete branch
```

---

## Remote Repositories

### Adding and Pushing to Remote
```bash
git remote add origin <url>   # Link to remote repo
git push -u origin main       # Push changes
```

### Pulling and Fetching
```bash
git pull                      # Fetch and merge from remote
git fetch                     # Fetch from remote
```

---

## GitHub Essentials

### Cloning a Repo
```bash
git clone https://github.com/user/repo.git
```

### Creating a Pull Request (PR)
1. Fork the repository
2. Clone your fork
3. Create a new branch
4. Commit and push changes
5. Open a PR on GitHub

### GitHub Features
- **Issues**: Report bugs, suggest features
- **Actions**: Automate workflows
- **Wiki**: Share documentation
- **Projects**: Organize tasks with kanban boards

---

## .gitignore

Used to exclude files/folders from version control:
```plaintext
node_modules/
.env
*.log
.DS_Store
```
# Git and GitHub Notes

## Introduction

### What is Git?
- Git is a **distributed version control system** used to track changes in source code.
- It allows **multiple developers** to collaborate on a project efficiently.
- Supports **branching, merging**, and **version history**.

### What is GitHub?
- GitHub is a **web-based platform** that hosts Git repositories.
- It provides tools for collaboration, such as:
  - Pull Requests
  - Issues
  - Project boards
  - GitHub Actions (CI/CD)

---

## Git Workflow Basics

### Initial Setup
```bash
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
```

### Starting a Repository
```bash
git init                   # Initialize a new Git repo
git clone <repo-url>       # Clone an existing repo
```

### Staging and Committing Changes
```bash
git status                 # View current changes
git add <file>             # Stage a file
git add .                  # Stage all changes
git commit -m "message"    # Commit staged changes
```

### Checking History
```bash
git log                    # View commit history
git log --oneline          # Compact view
git diff                   # Show changes
```

---

## Branching and Merging

### Working with Branches
```bash
git branch                 # List branches
git branch <name>          # Create a branch
git checkout <name>        # Switch to a branch
git checkout -b <name>     # Create and switch
```

### Merging Branches
```bash
git merge <branch>         # Merge into current branch
git branch -d <branch>     # Delete branch
```

---

## Remote Repositories

### Adding and Pushing to Remote
```bash
git remote add origin <url>   # Link to remote repo
git push -u origin main       # Push changes
```

### Pulling and Fetching
```bash
git pull                      # Fetch and merge from remote
git fetch                     # Fetch from remote
```

---

## GitHub Essentials

### Cloning a Repo
```bash
git clone https://github.com/user/repo.git
```

### Creating a Pull Request (PR)
1. Fork the repository
2. Clone your fork
3. Create a new branch
4. Commit and push changes
5. Open a PR on GitHub

### GitHub Features
- **Issues**: Report bugs, suggest features
- **Actions**: Automate workflows
- **Wiki**: Share documentation
- **Projects**: Organize tasks with kanban boards

---

## .gitignore

Used to exclude files/folders from version control:
```plaintext
