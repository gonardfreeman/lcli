<h1 align="center">LCLI</h1>

<p align="center">
  Simple Linear CLI
</p>

<p align="center">
  <a href="https://github.com/gonardfreeman/lcli/releases">
    <img src="https://img.shields.io/github/v/release/gonardfreeman/lcli?label=version" />
  </a>
  <a href="https://github.com/gonardfreeman/lcli/actions">
    <img src="https://github.com/gonardfreeman/lcli/actions/workflows/release.yml/badge.svg" />
  </a>
  <a href="https://github.com/gonardfreeman/lcli/actions">
    <img src="https://github.com/gonardfreeman/lcli/actions/workflows/test.yml/badge.svg" />
  </a>
</p>

<p align="center">
  <a href="https://github.com/gonardfreeman/lcli/releases/latest/download/lcli-linux-x86_64">
    <img src="https://img.shields.io/badge/Linux-download-blue?logo=linux" />
  </a>
  <a href="https://github.com/gonardfreeman/lcli/releases/latest/download/lcli-macos-x86_64">
    <img src="https://img.shields.io/badge/macOS-download-black?logo=apple" />
  </a>
  <a href="https://github.com/gonardfreeman/lcli/releases/latest/download/lcli-windows-x86_64.exe">
    <img src="https://img.shields.io/badge/Windows-download-blue?logo=windows" />
  </a>
</p>

---

**lcli** is a simple Rust-based command-line tool for interacting with Linear issues.

---

## 🧠 Features

* Fetch Linear issues by key
* Post comments to issues
* Optional control over subscription behavior
* Lightweight and fast Rust binary
* Built with **Clap**

---

## 🚀 Installation

### From source

```bash
git clone https://github.com/gonardfreeman/lcli.git
cd lcli
cargo build --release
```

Binary location:

```bash
target/release/lcli
```

---

### Install via script (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/gonardfreeman/lcli/main/install.sh | bash
```

This installs the latest release to `/usr/local/bin` (or prompts for a location).

---

### Install on Windows

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
iex (iwr https://raw.githubusercontent.com/gonardfreeman/lcli/main/install.ps1 -UseBasicParsing)
```

Optional: add to PATH

```powershell
[Environment]::SetEnvironmentVariable("PATH", "$env:USERPROFILE\bin;" + $env:PATH, [EnvironmentVariableTarget]::User)
```

---

## 🔐 Authentication

`lcli` requires a Linear API key.

Set it as an environment variable:

```bash
export LINEAR_API_KEY=your_api_key
```

On Windows:

```powershell
setx LINEAR_API_KEY "your_api_key"
```

---

## 🎯 Usage

### Get an issue

```bash
lcli get LIN-15
```

---

### Post a comment

```bash
lcli post-comment -i LIN-15 -b "This is a comment"
```

---

### Post a comment without subscribing

```bash
lcli post-comment -i LIN-15 -b "This is a comment" -d true
```

---

## ⚙️ Commands

### `get`

Fetch a Linear issue by key.

```bash
lcli get <ISSUE_KEY>
```

---

### `post-comment`

Post a comment to a Linear issue.

```bash
lcli post-comment [OPTIONS]
```

| Flag                     | Description                   |
| ------------------------ | ----------------------------- |
| `-i`, `--issue-key`      | Issue key (e.g., `LIN-15`)    |
| `-b`, `--body`           | Comment body                  |
| `-d`, `--dont-subscribe` | Do not subscribe to the issue |

---

## 📦 Releases

Precompiled binaries are available on GitHub:

[https://github.com/gonardfreeman/lcli/releases](https://github.com/gonardfreeman/lcli/releases)

---

## 🛠️ Development

Run locally:

```bash
cargo run -- get LIN-15
```

```bash
cargo run -- post-comment -i LIN-15 -b "test"
```

Build optimized binary:

```bash
cargo build --release
```

---

## 📁 Project Structure

* `src/main.rs` — CLI entry point
* `src/requests/` — API request logic
* `src/models/` — data models
* `src/config/` — configuration handling
* `install.sh` / `install.ps1` — install scripts

---

## 📝 License

MIT License — free to use, modify, and distribute.

---

## 👍 Contributing

Contributions welcome:

* Open issues
* Submit PRs
* Suggest improvements

---

## 🧩 Notes

* Uses `env_logger` for logging
* Errors are handled via `anyhow`
* CLI parsing powered by `clap`

