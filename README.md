<div align="center">

<h1>repomd</h1>

<p><strong>Repositori apa pun. Satu perintah. Konteks sempurna.</strong><br/>
<em>Any repository. One command. Perfect context.</em></p>

[![Crates.io](https://img.shields.io/crates/v/repomd?style=flat-square&color=fc8d62)](https://crates.io/crates/repomd)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](./LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/ardelyo/repomd/ci.yml?style=flat-square)](https://github.com/ardelyo/repomd/actions)
[![Made in Indonesia](https://img.shields.io/badge/Made%20in-Indonesia-cc0001?style=flat-square)](https://github.com/ardelyo)
[![Supported by OurCreativity](https://img.shields.io/badge/Supported%20by-OurCreativity%20Organization-f97316?style=flat-square)](https://ourcreativty.org)

<br/>

[Mulai Cepat · Quick Start](#mulai-cepat--quick-start) &nbsp;·&nbsp;
[Penggunaan · Usage](#penggunaan--usage) &nbsp;·&nbsp;
[Cara Kerja · How It Works](#cara-kerja--how-it-works) &nbsp;·&nbsp;
[Parameter · Flags](#parameter--flags)

</div>

---

## Tentang · About

`repomd` mengubah seluruh kode dalam repositori — baik lokal maupun remote — menjadi satu file Markdown yang dioptimalkan untuk token. Alat ini menilai kepentingan setiap file, mengompres konten secara semantik sesuai tekanan token, dan menghasilkan konteks repositori yang siap ditempel langsung ke ChatGPT, Claude, atau Gemini.

> `repomd` transforms an entire codebase — local or remote — into a single token-optimized Markdown file. It evaluates file importance, applies semantic compression based on token pressure, and produces clean repository context ready to be pasted into any large language model interface.

---

## Fitur Utama · Key Features

- **Penilaian File Cerdas** — Setiap file diberi Context Priority Score (CPS) berdasarkan perannya: kode sumber, konfigurasi, atau dokumentasi.
- **Kompresi Semantik** — Bukan pemotongan acak. `repomd` menganalisis struktur kode dan hanya menghapus bagian yang paling tidak kritis.
- **Lokal & Remote** — Bekerja dengan direktori lokal maupun URL GitHub secara langsung, tanpa perlu clone manual.
- **Empat Level Preset** — Dari `light` hingga `ultra`, sesuaikan tingkat kompresi dengan kebutuhan token LLM kamu.
- **Siap CI/CD** — Output format JSON tersedia untuk integrasi dengan pipeline otomatis.

---

## Mulai Cepat · Quick Start

**Prasyarat · Requirement:** Pastikan Rust sudah terpasang. Dapatkan di [rustup.rs](https://rustup.rs).

```bash
# Pasang secara global via Cargo
# Install globally via Cargo
cargo install --path cli
```

Setelah terpasang, jalankan wizard interaktif dengan:

```bash
repomd
```

---

## Penggunaan · Usage

### Wizard Interaktif · Interactive Wizard

Cara termudah untuk memulai. Jalankan `repomd` tanpa argumen untuk membuka wizard yang memandu seluruh proses konfigurasi.

```bash
repomd
```

Wizard akan meminta input berikut secara berurutan:

| Parameter | Keterangan · Description | Contoh · Example |
|---|---|---|
| **Sumber · Source** | Direktori lokal atau URL GitHub | `.` atau `https://github.com/user/repo` |
| **Preset** | Tingkat kompresi | `light`, `medium`, `aggressive`, `ultra` |
| **Anggaran · Budget** | Batas maksimum token | `50000`, `128000`, `200000` |
| **Keluaran · Output** | Tujuan hasil | `repo.md` atau clipboard |

---

### Buat dari Direktori Lokal · Generate from Local Directory

Menghasilkan `repo.md` dari direktori aktif dengan preset `medium` dan batas 50 ribu token.

```bash
repomd generate
```

---

### Buat dari URL GitHub · Generate from a GitHub URL

Tidak perlu clone manual. Tempel URL langsung — `repomd` akan melakukan clone sementara, mengekstrak konteks, lalu membersihkan hasil clone secara otomatis.

```bash
repomd generate https://github.com/torvalds/linux -p ultra -t 100000
```

---

### Salin Langsung ke Clipboard · Copy to Clipboard

Lewati pembuatan file dan salin hasil langsung ke clipboard.

```bash
repomd generate --copy
```

---

### Inspeksi Prioritas File · Inspect File Scoring

Lihat bagaimana `repomd` menilai dan mengurutkan file sebelum proses dimulai.

```bash
repomd inspect
```

Perintah ini menampilkan dashboard interaktif yang menunjukkan Context Priority Score (CPS) dan kategori setiap file — Source, Config, atau Docs.

---

## Parameter · Flags

| Flag | Keterangan · Description |
|---|---|
| `-t, --tokens <NUM>` | Batas maksimum token · Max token budget (default: `50,000`) |
| `-p, --preset <STR>` | Level kompresi: `light` · `medium` · `aggressive` · `ultra` |
| `--dry-run` | Pratinjau hasil tanpa menulis file apapun |
| `--copy` | Salin hasil langsung ke clipboard |
| `-v, --verbose` | Tampilkan statistik kompresi tiap file di dashboard |
| `-q, --quiet` | Sembunyikan spinner dan output dashboard |
| `--json` | Keluarkan statistik generasi dalam format JSON |

---

## Cara Kerja · How It Works

```
Repositori / Repository
        │
        ▼
  [1] Penemuan · Discovery
        Memindai seluruh direktori, menghormati .gitignore secara native.
        │
        ▼
  [2] Penilaian · Scoring
        Setiap file diberi Context Priority Score (CPS).
        Source > Config > Docs
        │
        ▼
  [3] Kompresi · Compression
        Kompresi semantik berdasarkan preset yang dipilih.
        Light → Medium → Aggressive → Ultra
        │
        ▼
  [4] Perakitan · Assembly
        File dikemas secara optimal ke dalam satu .md
        dalam batas token menggunakan algoritma knapsack.
        │
        ▼
  repo.md  /  Clipboard
```

### Level Kompresi · Compression Levels

| Preset | Yang Dihapus · What Gets Stripped |
|---|---|
| `light` | Spasi berlebih dan komentar · Whitespace and comments |
| `medium` | Isi fungsi — menyimpan struct, enum, dan signature |
| `aggressive` | Semua kecuali antarmuka publik · Everything except public interfaces |
| `ultra` | Seluruh file diganti dengan ringkasan metadata singkat |

---

## Benchmark & Laporan · Benchmarks & Reports

Saksikan bagaimana `repomd` mengoptimalkan kode kamu hingga batas maksimal.

### Visualisasi · Visualizations

<div align="center">

| Kinerja Scan · Scan Performance | Penghematan Token · Token Savings |
|:---:|:---:|
| ![Scan Speed](./benchmarks/chart_scan_speed.png) | ![Token Reduction](./benchmarks/chart_token_reduction.png) |
| *Kecepatan pemindaian (ms)* | *Reduksi token per preset* |

| Rasio Kompresi · Compression Ratio | Token Mentah · Raw Tokens |
|:---:|:---:|
| ![Compression Ratio](./benchmarks/chart_compression_ratio.png) | ![Raw Tokens](./benchmarks/chart_raw_tokens.png) |
| *Efisiensi ruang* | *Volume data sebelum kompresi* |

</div>

### Statistik Performa · Performance Statistics

Hasil pengujian pada berbagai skala repositori (diukur pada Intel i9-13900K):

| Repositori · Repository | Ukuran · Size | File | Raw Tokens | Ultra (ratio) | Scan Time |
|:---|:---:|:---:|:---:|:---:|:---:|
| **fakewriter** | Micro | 19 | 12k | 621 (20x) | 874ms |
| **repomd** | Small | 38 | 133k | 6.6k (20x) | 27ms |
| **Artificial General Detector** | Large | 98 | 523k | 26k (20x) | 606ms |
| **ourcreativity** | XL | 196 | 612k | 30k (20x) | 2.8s |

### Perbandingan Kompetitif · Competitive Analysis

`repomd` dirancang untuk mengungguli alat serupa dalam hal kepadatan konteks:

| Alat · Tool | Rasio Kompresi · Ratio | Metode · Method |
|:---|:---:|:---|
| **repomd (Ultra)** | **20.0x** | **Semantic metadata summaries** |
| **repomd (Aggressive)** | **5.0x** | **Public interface extraction** |
| **repomix** | 2.3x | Tree-sitter (~70% reduction) |
| **gitingest** | 1.8x | Light structured digest |
| **raw cat** | 1.0x | Verbatim baseline |

</div>

### Laporan Detail · Detailed Reports

Pelajari hasil pengujian mendalam kami pada berbagai ukuran repositori:

- 📊 [**Laporan Benchmark Standar**](./docs/repomd_benchmark_report.docx) — Analisis performa dasar.
- 🚀 [**Laporan Ultra Benchmark**](./docs/repomd_ultra_benchmark_report.docx) — Batas ekstrem kompresi pada repo masif.
- 📝 [**Demo OurCreativity**](./docs/ourcreativity_demo.md) — Contoh output nyata pada proyek OurCreativity.

---

## Kontribusi · Contributing

Kontribusi, laporan masalah, dan permintaan fitur sangat disambut baik. Silakan buka [issue](https://github.com/ardelyo/repomd/issues) atau ajukan pull request.

---

## Lisensi · License

Didistribusikan di bawah Lisensi MIT. Lihat [`LICENSE`](./LICENSE) untuk informasi lebih lanjut.

---

<div align="center">

Dibuat oleh **Ardelyo** &nbsp;·&nbsp; Indonesia 🇮🇩

Didukung oleh **OurCreativity Organization**

</div>