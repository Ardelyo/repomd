<div align="center">

<h1>📦 repomd</h1>

<p><strong>Any repo. One command. Perfect context.</strong></p>

<p>Transform any codebase into a single token-optimized Markdown file — ready to paste into ChatGPT, Claude, or Gemini.</p>

[![Crates.io](https://img.shields.io/crates/v/repomd?style=flat-square&color=fc8d62)](https://crates.io/crates/repomd)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](./LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/ardelyo/repomd/ci.yml?style=flat-square)](https://github.com/ardelyo/repomd/actions)
[![Made in Indonesia](https://img.shields.io/badge/Made%20in-Indonesia-red?style=flat-square)](https://github.com/ardelyo)
[![OurCreativity](https://img.shields.io/badge/Supported%20by-OurCreativity-orange?style=flat-square)](https://ourcreativty.org)

<br/>

[快速开始 · Quick Start](#-quick-start) &nbsp;|&nbsp;
[用法 · Usage](#-usage) &nbsp;|&nbsp;
[工作原理 · How It Works](#-how-it-works) &nbsp;|&nbsp;
[参数 · Flags](#-advanced-flags)

</div>

---

## ✨ Features

- 🔍 **Smart File Scoring** — Every file gets a Context Priority Score (CPS) based on its role in the codebase
- ⚡ **Dynamic Compression** — Tree-sitter style semantic compression, not blind truncation
- 🌐 **Local & Remote** — Works on local directories or GitHub URLs directly
- 📋 **Clipboard Support** — Skip the file, copy straight to your clipboard
- 🎛️ **4 Presets** — Light, Medium, Aggressive, Ultra — pick your compression level
- 🤖 **CI/CD Ready** — JSON output mode for pipeline integration
- 🧠 **Knapsack Packing** — Tightly fits the most important files within your token budget

---

## 🚀 Quick Start

> **Requirement:** Rust must be installed — get it at [rustup.rs](https://rustup.rs)

```bash
# Install globally via Cargo
cargo install --path cli

# Run the interactive wizard
repomd
```

---

## 📖 Usage

### Interactive Wizard

The easiest way to get started. Just run `repomd` with no arguments:

```bash
repomd
```

The wizard will guide you through:
- **Source** — current directory (`.`) or a GitHub URL
- **Preset** — compression level (`light` / `medium` / `aggressive` / `ultra`)
- **Budget** — token limit for your LLM (e.g. `50000`, `128000`, `200000`)
- **Output** — save to `repo.md` or copy to clipboard

---

### Generate from Local Directory

```bash
repomd generate
```

Generates `repo.md` from the current directory using the medium preset with a 50k token budget.

---

### Generate from a GitHub URL

No cloning needed. Paste the URL directly — `repomd` handles the rest and cleans up automatically.

```bash
repomd generate https://github.com/torvalds/linux -p ultra -t 100000
```

---

### Copy Directly to Clipboard

```bash
repomd generate --copy
```

---

### Inspect File Scoring

Preview how `repomd` prioritizes your files before generating:

```bash
repomd inspect
```

Displays an interactive dashboard with each file's Context Priority Score (CPS) and estimated category (Source / Config / Docs).

---

## 🛠️ Advanced Flags

| Flag | Description |
|---|---|
| `-t, --tokens <NUM>` | Set maximum token budget (default: `50,000`) |
| `-p, --preset <STR>` | `light` · `medium` · `aggressive` · `ultra` |
| `--dry-run` | Preview output without writing any files |
| `--copy` | Copy result directly to clipboard |
| `-v, --verbose` | Show per-file compression stats in summary |
| `-q, --quiet` | Suppress spinners and dashboard output |
| `--json` | Output generation stats as JSON (for CI/CD) |

---

## 🧠 How It Works

```
📁 Repository
      │
      ▼
  ① Discovery ──── Scans all files, respects .gitignore natively
      │
      ▼
  ② Scoring ────── Assigns a Context Priority Score (CPS) per file
      │              Source > Config > Docs
      ▼
  ③ Compression ── Semantic compression based on preset
      │              Light → Medium → Aggressive → Ultra
      ▼
  ④ Assembly ───── Knapsack-packs files into one .md within token budget
      │
      ▼
  📄 repo.md  /  📋 Clipboard
```

### Compression Presets

| Preset | What gets stripped |
|---|---|
| `light` | Whitespace & comments |
| `medium` | Implementation bodies — keeps structs, enums, signatures |
| `aggressive` | Everything except public interfaces |
| `ultra` | Full file replaced with a `"Summarized File"` metadata entry |

---

## 📂 Repository Structure

- `cli/` — Command-line interface logic (Rust)
- `core/` — Core processing engine (Rust)
- `benchmarks/` — Performance testing scripts and results
- `docs/` — Detailed reports and demonstrations
- `examples/` — Sample outputs and test files
- `api/` — API server implementation
- `web/` — Web dashboard (if applicable)

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome.
Feel free to check the [issues page](https://github.com/ardelyo/repomd/issues).

---

## 📄 License

Distributed under the MIT License. See [`LICENSE`](./LICENSE) for details.

---

<div align="center">

Made with ❤️ by **Ardelyo** · Indonesia 🇮🇩

Supported by **OurCreativity Organization**

</div>