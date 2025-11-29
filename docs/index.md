---
layout: default
---

## 📚 Table of Contents

- [Features](#features)
- [Installation](#installation)
  - [npm](#npm)
  - [Homebrew](#homebrew)
- [Usage](#usage)
  - [Options](#options)
- [Uninstall](#uninstall)

---

## ✨ Features

- 🔥 Instantly purge branches gone from remote
- 🛠️ Safe, dry-run mode
- ⚡ Fast and easy to use
- 🏷️ Integrates with your Git workflow

---

## Installation

Install via any JS package manager (**npm**, **yarn**, **pnpm**) or **Homebrew**.

### npm

```sh
npm install -g git-purge
```

### Homebrew

```sh
brew install mariocampacci/tap/git-purge
```

## Usage

```sh
git purge
``` 

### Options

| Flag            | Description                                 |
|-----------------|---------------------------------------------|
| `-f`, `--force` | Force delete branches                       |
| `-n`, `--dry-run` | Show what would be deleted without deleting |
| `-v`, `--verbose` | Show detailed output                        |
| `-V`, `--version` | Show version                                |
| `-h`            | Show help message                           |

## Uninstall

```sh
npm uninstall -g git-purge  # if installed via npm
# or
brew uninstall git-purge  # if installed via brew
```