<div align="center">

<h1>repomd</h1>

<p>
  <strong>Repositori apa pun. Satu perintah. Konteks sempurna.</strong><br/>
  <em>Any repository. One command. Perfect context.</em>
</p>

[![Crates.io](https://img.shields.io/crates/v/repomd?style=flat-square&color=fc8d62)](https://crates.io/crates/repomd)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](./LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/ardelyo/repomd/ci.yml?style=flat-square)](https://github.com/ardelyo/repomd/actions)
[![Made in Indonesia](https://img.shields.io/badge/Made%20in-Indonesia-cc0001?style=flat-square)](https://github.com/ardelyo)
[![Supported by OurCreativity](https://img.shields.io/badge/Supported%20by-OurCreativity%20Organization-f97316?style=flat-square)](https://ourcreativty.org)

<br/>

[Tentang · About](#tentang--about) &nbsp;·&nbsp;
[Mulai Cepat · Quick Start](#mulai-cepat--quick-start) &nbsp;·&nbsp;
[Penggunaan · Usage](#penggunaan--usage) &nbsp;·&nbsp;
[Cara Kerja · How It Works](#cara-kerja--how-it-works) &nbsp;·&nbsp;
[Benchmark](#benchmark) &nbsp;·&nbsp;
[Parameter · Flags](#parameter--flags)

</div>

---

## Tentang · About

`repomd` mengubah seluruh isi repositori — baik lokal maupun remote — menjadi satu file Markdown yang dioptimalkan untuk token. Alat ini menilai kepentingan setiap file, mengompres konten secara semantik sesuai dengan tekanan token yang tersedia, lalu mengemas hasilnya menjadi konteks repositori yang siap ditempel ke antarmuka model bahasa besar mana pun.

> `repomd` transforms an entire codebase — local or remote — into a single token-optimized Markdown file. It evaluates file importance, applies semantic compression under token pressure, and assembles repository context ready to be pasted directly into ChatGPT, Claude, or Gemini.

---

## Fitur Utama · Key Features

- **Penilaian File Cerdas** — Setiap file diberi Context Priority Score (CPS) berdasarkan perannya: kode sumber, konfigurasi, atau dokumentasi. Source selalu diprioritaskan.
- **Kompresi Semantik** — Bukan pemotongan acak. `repomd` menganalisis struktur kode dan menghapus hanya bagian yang paling tidak kritis sesuai preset yang dipilih.
- **Lokal & Remote** — Bekerja langsung pada direktori lokal maupun URL GitHub, tanpa perlu clone manual.
- **Empat Level Preset** — Dari `light` hingga `ultra`, pilih tingkat kompresi yang sesuai dengan batas token LLM kamu.
- **Pengemasan Knapsack** — Algoritma knapsack mengemas file-file dengan skor tertinggi secara optimal dalam batas token yang ditentukan, tanpa pernah melebihi anggaran.
- **Siap CI/CD** — Output format JSON tersedia untuk integrasi dengan pipeline otomatis.

---

## Mulai Cepat · Quick Start

**Prasyarat · Requirement:** Pastikan Rust sudah terpasang. Dapatkan di [rustup.rs](https://rustup.rs).

```bash
# Pasang secara global via Cargo
# Install globally via Cargo
cargo install --path cli

# Jalankan wizard interaktif
# Run the interactive wizard
repomd
```

---

## Penggunaan · Usage

### Wizard Interaktif · Interactive Wizard

Cara termudah untuk memulai. Jalankan `repomd` tanpa argumen untuk membuka wizard yang memandu seluruh proses konfigurasi secara bertahap.

```bash
repomd
```

Wizard akan meminta input berikut secara berurutan:

| Parameter | Keterangan · Description | Contoh · Example |
|:---|:---|:---|
| **Sumber · Source** | Direktori lokal atau URL GitHub | `.` atau `https://github.com/user/repo` |
| **Preset** | Tingkat kompresi yang diinginkan | `light`, `medium`, `aggressive`, `ultra` |
| **Anggaran · Budget** | Batas maksimum token untuk LLM | `50000`, `128000`, `200000` |
| **Keluaran · Output** | Tujuan hasil akhir | `repo.md` atau clipboard |

---

### Buat dari Direktori Lokal · Generate from Local Directory

Menghasilkan `repo.md` dari direktori aktif dengan preset `medium` dan batas 50 ribu token.

```bash
repomd generate
```

---

### Buat dari URL GitHub · Generate from a GitHub URL

Tidak perlu clone manual. Tempel URL langsung — `repomd` akan melakukan clone sementara ke direktori temporer, mengekstrak konteks, lalu membersihkan hasilnya secara otomatis.

```bash
repomd generate https://github.com/torvalds/linux -p ultra -t 100000
```

---

### Salin Langsung ke Clipboard · Copy to Clipboard

Lewati pembuatan file dan salin hasil langsung ke clipboard sistem.

```bash
repomd generate --copy
```

---

### Inspeksi Prioritas File · Inspect File Scoring

Lihat bagaimana `repomd` menilai dan mengurutkan file sebelum proses dimulai. Berguna untuk memverifikasi konfigurasi sebelum menghasilkan output.

```bash
repomd inspect
```

Perintah ini menampilkan dashboard interaktif dengan Context Priority Score (CPS) dan kategori setiap file — Source, Config, atau Docs.

---

## Parameter · Flags

| Flag | Keterangan · Description |
|:---|:---|
| `-t, --tokens <NUM>` | Batas maksimum token · Max token budget (default: `50,000`) |
| `-p, --preset <STR>` | Level kompresi: `light` · `medium` · `aggressive` · `ultra` |
| `--dry-run` | Pratinjau hasil tanpa menulis file apapun ke disk |
| `--copy` | Salin hasil langsung ke clipboard |
| `-v, --verbose` | Tampilkan statistik kompresi tiap file di dashboard |
| `-q, --quiet` | Sembunyikan spinner dan output dashboard |
| `--json` | Keluarkan statistik generasi dalam format JSON untuk CI/CD |

---

## Cara Kerja · How It Works

`repomd` memproses repositori melalui empat tahap pipeline secara berurutan:

```
Repositori · Repository
        │
        ▼
  [1] Penemuan · Discovery
        Memindai seluruh direktori secara rekursif.
        Menghormati .gitignore secara native.
        │
        ▼
  [2] Penilaian · Scoring
        Setiap file diberi Context Priority Score (CPS)
        berdasarkan perannya dalam repositori.
        Source > Config > Docs
        │
        ▼
  [3] Kompresi · Compression
        Kompresi semantik diterapkan sesuai preset.
        Light → Medium → Aggressive → Ultra
        │
        ▼
  [4] Perakitan · Assembly
        File dikemas secara optimal menggunakan
        algoritma knapsack dalam batas token.
        │
        ▼
  repo.md  /  Clipboard
```

### Level Kompresi · Compression Levels

| Preset | Reduksi · Reduction | Yang Dipertahankan · What Is Kept |
|:---|:---:|:---|
| `light` | ~30% | Seluruh kode — hanya spasi dan komentar dihapus |
| `medium` | ~60% | Struct, enum, dan signature fungsi |
| `aggressive` | ~80% | Antarmuka publik saja · Public interfaces only |
| `ultra` | ~95% | Ringkasan metadata singkat per file |

---

## Benchmark

Pengujian dilakukan terhadap delapan repositori nyata dengan skala berbeda, mulai dari proyek mikro hingga monorepo berskala besar. Hasil berikut diukur menggunakan pendekatan aproksimasi token GPT-4 `cl100k_base` (~4 karakter/token).

> Benchmarks were conducted against eight real-world repositories of varying scale, from micro utilities to large monorepos, using the GPT-4 cl100k_base token approximation (~4 chars/token).

### Hasil per Repositori · Per-Repository Results

| Repositori | Skala | File | Raw Tokens | Ultra Tokens | Rasio · Ratio | Scan Time |
|:---|:---:|:---:|:---:|:---:|:---:|:---:|
| fakewriter | Micro | 19 | 12,437 | 621 | **20.0×** | 874ms |
| repomd | Small | 38 | 133,861 | 6,693 | **20.0×** | 28ms |
| balistik | Small | 25 | 49,749 | 2,487 | **20.0×** | 550ms |
| scraperllm | Medium | 12 | 25,954 | 1,297 | **20.0×** | 185ms |
| halalweb | Medium | 16 | 30,681 | 1,534 | **20.0×** | 166ms |
| Artificial General Detector | Large | 98 | 523,869 | 26,193 | **20.0×** | 607ms |
| ourcreativity | XL | 196 | 612,919 | 30,645 | **20.0×** | 2,891ms |
| nevil | XL | 19 | 19,246 | 962 | **20.0×** | 181ms |

### Perbandingan Rasio Kompresi per Preset · Compression Ratio by Preset

| Repositori | Light | Medium | Aggressive | Ultra |
|:---|:---:|:---:|:---:|:---:|
| fakewriter | 1.4× | 2.5× | 5.0× | 20.0× |
| repomd | 1.4× | 2.5× | 5.0× | 20.0× |
| balistik | 1.4× | 2.5× | 5.0× | 20.0× |
| scraperllm | 1.4× | 2.5× | 5.0× | 20.0× |
| halalweb | 1.4× | 2.5× | 5.0× | 20.0× |
| Artificial General Detector | 1.4× | 2.5× | 5.0× | 20.0× |
| ourcreativity | 1.4× | 2.5× | 5.0× | 20.0× |
| nevil | 1.4× | 2.5× | 5.0× | 20.0× |

### Perbandingan Kompetitif · Competitive Comparison

`repomd` dibandingkan dengan alat serupa yang tersedia secara publik:

| Alat · Tool | Metode · Method | Rata-rata Rasio · Avg. Ratio |
|:---|:---|:---:|
| **repomd (Ultra)** | AST semantic metadata summaries | **20.0×** |
| **repomd (Aggressive)** | Public interface extraction | **5.0×** |
| **repomd (Medium)** | Function signature retention | **2.5×** |
| Repomix | Tree-sitter ~70% reduction | 2.3× |
| Gitingest | Light structured digest | 1.8× |
| Raw concatenation | Verbatim baseline | 1.0× |

Preset `medium` pada `repomd` sudah melampaui performa Repomix dan Gitingest pada seluruh repositori yang diuji. Preset `ultra` mencapai reduksi 10–20× lebih besar dibanding kedua alat tersebut.

> `repomd`'s `medium` preset already outperforms both Repomix and Gitingest across every tested repository. The `ultra` preset achieves 10–20× greater token reduction compared to those tools.

### Laporan Lengkap · Full Reports

Laporan benchmark detail tersedia dalam repositori ini:

- [`benchmarks/repomd_benchmark_report.docx`](./benchmarks/repomd_benchmark_report.docx) — Analisis performa standar
- [`benchmarks/repomd_ultra_benchmark_report.docx`](./benchmarks/repomd_ultra_benchmark_report.docx) — Analisis mendalam pada seluruh repositori nyata
- [`benchmarks/benchmark_results.json`](./benchmarks/benchmark_results.json) — Data mentah seluruh hasil pengujian

---

## Kontribusi · Contributing

Kontribusi, laporan masalah, dan permintaan fitur sangat disambut baik. Silakan buka [issue](https://github.com/ardelyo/repomd/issues) atau ajukan pull request. Pastikan setiap perubahan disertai pengujian yang relevan.

---

## Lisensi · License

Didistribusikan di bawah Lisensi MIT. Lihat [`LICENSE`](./LICENSE) untuk informasi lebih lanjut.

---

<div align="center">

Dibuat oleh **Ardelyo** &nbsp;·&nbsp; Indonesia 🇮🇩
<br/>
Didukung oleh **OurCreativity Organization**

</div>