<context_metadata>
The following is an optimized codebase extraction.
Files are enclosed in `<file>` tags and compressed according to their context priority.
</context_metadata>

<repository_inventory>
- .\ultra_benchmark.py
- .\benchmark_results.json
- .\cli\src\main.rs
- .\benchmark.py
- .\benchmark_ui.html
- .\web\package-lock.json
- .\api\src\main.rs
- .\core\src\assemble.rs
- .\core\src\compress.rs
- .\core\src\discover.rs
- .\core\src\ingest.rs
- .\core\src\lib.rs
- .\web\src\app\globals.css
- .\Cargo.toml
- .\web\README.md
- .\api\Cargo.toml
- .\cli\Cargo.toml
- .\core\Cargo.toml
- .\extensions\github-action\action.yml
- .\web\.eslintrc.json
- .\web\package.json
- .\web\tsconfig.json
- .\format_test.py
</repository_inventory>

<file path=".\ultra_benchmark.py" cps="2.40" level="3">
```py
[Summarized File (Source)] 23580 bytes removed to save tokens.
```
</file>

<file path=".\benchmark_results.json" cps="2.10" level="3">
```json
{
  "results": [
    {
      "name": "fakewriter",
      "category": "Micro",
      "scan_ms": 874.0,
      "total_files": 19,
      "code_files": 18,
      "raw_bytes": 50998,
      "raw_tokens": 12437,
      "presets": [
        {
          "preset": "Light",
          "raw_tokens": 12437,
          "compressed_tokens": 8705,
          "ratio": 1.4,
          "proc_ms": 447
        },
        {
          "preset": "Medium",
          "raw_tokens": 12437,
          "compressed_tokens": 4974,
          "ratio": 2.5,
          "proc_ms": 436
        },
        {
          "preset": "Aggressive",
          "raw_tokens": 12437,
          "compressed_tokens": 2487,
          "ratio": 5.0,
          "proc_ms": 458
        },
        {
          "preset": "Ultra",
          "raw_tokens": 12437,
          "compressed_tokens": 621,
          "ratio": 20.0,
          "proc_ms": 433
        }
      ],
      "top_ext": [
        [
          ".py",
          11
        ],
        [
          ".log",
          2
        ],
        [
          "",
          1
        ],
        [
          ".resolved",
          1
        ],
        [
          ".md",
          1
        ]
      ]
    },
    {
      "name": "repomd",
      "category": "Small",
      "scan_ms": 27.6,
      "total_files": 38,
      "code_files": 33,
      "raw_bytes": 535495,
      "raw_tokens": 133861,
      "presets": [
        {
          "preset": "Light",
          "raw_tokens": 133861,
          "compressed_tokens": 93702,
          "ratio": 1.4,
          "proc_ms": 4080
        },
        {
          "preset": "Medium",
          "raw_tokens": 133861,
          "compressed_tokens": 53544,
          "ratio": 2.5,
          "proc_ms": 4083
        },
        {
          "preset": "Aggressive",
          "raw_tokens": 133861,
          "compressed_tokens": 26772,
          "ratio": 5.0,
          "proc_ms": 4100
        },
        {
          "preset": "Ultra",
          "raw_tokens": 133861,
          "compressed_tokens": 6693,
          "ratio": 20.0,
          "proc_ms": 4100
        }
      ],
      "top_ext": [
        [
          ".rs",
          7
        ],
        [
          "",
          4
        ],
        [
          ".toml",
          4
        ],
        [
          ".json",
          4
        ],
        [
          ".py",
          2
        ]
      ]
    },
    {
      "name": "balistik",
      "category": "Small",
      "scan_ms": 550.1,
      "total_files": 25,
      "code_files": 22,
      "raw_bytes": 200730,
      "raw_tokens": 49749,
      "presets": [
        {
          "preset": "Light",
          "raw_tokens": 49749,
          "compressed_tokens": 34824,
          "ratio": 1.4,
          "proc_ms": 1585
        },
        {
          "preset": "Medium",
          "raw_tokens": 49749,
          "compressed_tokens": 19899,
          "ratio": 2.5,
          "proc_ms": 1590
        },
        {
          "preset": "Aggressive",
          "raw_tokens": 49749,
          "compressed_tokens": 9949,
          "ratio": 5.0,
          "proc_ms": 1567
        },
        {
          "preset": "Ultra",
          "raw_tokens": 49749,
          "compressed_tokens": 2487,
          "ratio": 20.0,
          "proc_ms": 1589
        }
      ],
      "top_ext": [
        [
          ".js",
          11
        ],
        [
          ".md",
          4
        ],
        [
          ".html",
          2
        ],
        [
          ".json",
          2
        ],
        [
          "",
          1
        ]
      ]
    },
    {
      "name": "scraperllm",
      "category": "Medium",
      "scan_ms": 184.7,
      "total_files": 12,
      "code_files": 12,
      "raw_bytes": 105032,
      "raw_tokens": 25954,
      "presets": [
        {
          "preset": "Light",
          "raw_tokens": 25954,
          "compressed_tokens": 18167,
          "ratio": 1.4,
          "proc_ms": 857
        },
        {
          "preset": "Medium",
          "raw_tokens": 25954,
          "compressed_tokens": 10381,
          "ratio": 2.5,
          "proc_ms": 862
        },
        {
          "preset": "Aggressive",
          "raw_tokens": 25954,
          "compressed_tokens": 5190,
          "ratio": 5.0,
          "proc_ms": 843
        },
        {
          "preset": "Ultra",
          "raw_tokens": 25954,
          "compressed_tokens": 1297,
          "ratio": 20.0,
          "proc_ms": 871
        }
      ],
      "top_ext": [
        [
          ".ts",
          5
        ],
        [
          ".json",
          4
        ],
        [
          "",
          1
        ],
        [
          ".html",
          1
        ],
        [
          ".md",
          1
        ]
      ]
    },
    {
      "name": "halalweb",
      "category": "Medium",
      "scan_ms": 165.7,
      "total_files": 16,
      "code_files": 14,
      "raw_bytes": 122745,
      "raw_tokens": 30681,
      "presets": [
        {
          "preset": "Light",
          "raw_tokens": 30681,
          "compressed_tokens": 21476,
          "ratio": 1.4,
          "proc_ms": 992
        },
        {
          "preset": "Medium",
          "raw_tokens": 30681,
          "compressed_tokens": 12272,
          "ratio": 2.5,
          "proc_ms": 994
        },
        {
          "preset": "Aggressive",
          "raw_tokens": 30681,
          "compressed_tokens": 6136,
          "ratio": 5.0,
          "proc_ms": 1008
        },
        {
          "preset": "Ultra",
          "raw_tokens": 30681,
          "compressed_tokens": 1534,
          "ratio": 20.0,
          "proc_ms": 1008
        }
      ],
      "top_ext": [
        [
          ".json",
          5
        ],
        [
          ".css",
          2
        ],
        [
          ".tsx",
          2
        ],
        [
          "",
          1
        ],
        [
          ".js",
          1
        ]
      ]
    },
    {
      "name": "Artificial General Detector",
      "category": "Large",
      "scan_ms": 606.5,
      "total_files": 98,
      "code_files": 72,
      "raw_bytes": 2096618,
      "raw_tokens": 523869,
      "presets": [
        {
          "preset": "Light",
          "raw_tokens": 523869,
          "compressed_tokens": 366708,
          "ratio": 1.4,
          "proc_ms": 15796
        },
        {
          "preset": "Medium",
          "raw_tokens": 523869,
          "compressed_tokens": 209547,
          "ratio": 2.5,
          "proc_ms": 15802
        },
        {
          "preset": "Aggressive",
          "raw_tokens": 523869,
          "compressed_tokens": 104773,
          "ratio": 5.0,
          "proc_ms": 15783
        },
        {
          "preset": "Ultra",
          "raw_tokens": 523869,
          "compressed_tokens": 26193,
          "ratio": 20.0,
          "proc_ms": 15805
        }
      ],
      "top_ext": [
        [
          ".py",
          18
        ],
        [
          ".txt",
          13
        ],
        [
          "",
          8
        ],
        [
          ".docx",
          8
        ],
        [
          ".md",
          7
        ]
      ]
    },
    {
      "name": "ourcreativity",
      "category": "XL",
      "scan_ms": 2891.0,
      "total_files": 196,
      "code_files": 168,
      "raw_bytes": 2505522,
      "raw_tokens": 612919,
      "presets": [
        {
          "preset": "Light",
          "raw_tokens": 612919,
          "compressed_tokens": 429043,
          "ratio": 1.4,
          "proc_ms": 18452
        },
        {
          "preset": "Medium",
          "raw_tokens": 612919,
          "compressed_tokens": 245167,
          "ratio": 2.5,
          "proc_ms": 18460
        },
        {
          "preset": "Aggressive",
          "raw_tokens": 612919,
          "compressed_tokens": 122583,
          "ratio": 5.0,
          "proc_ms": 18456
        },
        {
          "preset": "Ultra",
          "raw_tokens": 612919,
          "compressed_tokens": 30645,
          "ratio": 20.0,
          "proc_ms": 18465
        }
      ],
      "top_ext": [
        [
          ".tsx",
          76
        ],
        [
          ".md",
          37
        ],
        [
          ".ts",
          23
        ],
        [
          ".sql",
          13
        ],
        [
          ".json",
          7
        ]
      ]
    },
    {
      "name": "nevil",
      "category": "XL",
      "scan_ms": 181.4,
      "total_files": 19,
      "code_files": 14,
      "raw_bytes": 77462,
      "raw_tokens": 19246,
      "presets": [
        {
          "preset": "Light",
          "raw_tokens": 19246,
          "compressed_tokens": 13472,
          "ratio": 1.4,
          "proc_ms": 652
        },
        {
          "preset": "Medium",
          "raw_tokens": 19246,
          "compressed_tokens": 7698,
          "ratio": 2.5,
          "proc_ms": 651
        },
        {
          "preset": "Aggressive",
          "raw_tokens": 19246,
          "compressed_tokens": 3849,
          "ratio": 5.0,
          "proc_ms": 646
        },
        {
          "preset": "Ultra",
          "raw_tokens": 19246,
          "compressed_tokens": 962,
          "ratio": 20.0,
          "proc_ms": 655
        }
      ],
      "top_ext": [
        [
          ".py",
          3
        ],
        [
          ".json",
          3
        ],
        [
          ".css",
          2
        ],
        [
          ".ts",
          2
        ],
        [
          ".txt",
          1
        ]
      ]
    }
  ],
  "competitive": {
    "repomix": {
      "ratio": 2.3,
      "source": "repomix.com (Tree-sitter ~70% reduction)"
    },
    "gitingest": {
      "ratio": 1.8,
      "source": "gitingest.io (light structured digest)"
    },
    "raw_cat": {
      "ratio": 1.0,
      "source": "cat *.* (verbatim baseline)"
    }
  },
  "generated": "2026-03-20T06:57:16.055268"
}
```
</file>

<file path=".\cli\src\main.rs" cps="2.10" level="3">
```rs
[Summarized File (Source)] 22295 bytes removed to save tokens.
```
</file>

<file path=".\benchmark.py" cps="2.00" level="3">
```py
[Summarized File (Source)] 4409 bytes removed to save tokens.
```
</file>

<file path=".\benchmark_ui.html" cps="2.00" level="3">
```html
[Summarized File (Source)] 2848 bytes removed to save tokens.
```
</file>


> [Limit Reached: 50000 tokens. Remaining files omitted.]
<file path=".\api\src\main.rs" cps="1.70" level="3">
```rs
[Summarized File (Source)] 2478 bytes removed to save tokens.
```
</file>

<file path=".\core\src\assemble.rs" cps="1.70" level="3">
```rs
[Summarized File (Source)] 3594 bytes removed to save tokens.
```
</file>

<file path=".\core\src\compress.rs" cps="1.70" level="3">
```rs
[Summarized File (Source)] 3320 bytes removed to save tokens.
```
</file>

<file path=".\core\src\discover.rs" cps="1.70" level="3">
```rs
[Summarized File (Source)] 1994 bytes removed to save tokens.
```
</file>

<file path=".\core\src\ingest.rs" cps="1.70" level="3">
```rs
[Summarized File (Source)] 1322 bytes removed to save tokens.
```
</file>

<file path=".\core\src\lib.rs" cps="1.70" level="3">
```rs
[Summarized File (Source)] 2926 bytes removed to save tokens.
```
</file>

<file path=".\web\src\app\globals.css" cps="1.70" level="3">
```css
[Summarized File (Source)] 8069 bytes removed to save tokens.
```
</file>

<file path=".\Cargo.toml" cps="1.50" level="3">
```toml
[workspace]
members = ["core", "cli", "api"]
resolver = "2"

```
</file>

<file path=".\web\README.md" cps="1.30" level="3">
```md
[Summarized File (Doc)] 1383 bytes removed to save tokens.
```
</file>

<file path=".\api\Cargo.toml" cps="1.20" level="3">
```toml
[package]
name = "repomd-api"
version = "0.1.0"
edition = "2021"

[dependencies]
repomd-core = { path = "../core" }
axum = "0.7"
tokio = { version = "1.35", features = ["full", "rt-multi-thread"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tower-http = { version = "0.5", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
uuid = { version = "1.6", features = ["v4", "fast-rng"] }

```
</file>

<file path=".\cli\Cargo.toml" cps="1.20" level="3">
```toml
[package]
name = "repomd"
version = "0.2.0"
edition = "2021"
description = "Any repo. One command. Perfect context."
authors = ["repomd contributors"]

[[bin]]
name = "repomd"
path = "src/main.rs"

[dependencies]
repomd-core = { path = "../core" }
clap = { version = "4.4", features = ["derive"] }
anyhow = "1.0"
arboard = "3.3"
tokio = { version = "1.35", features = ["full"] }
console = "0.15"
indicatif = "0.17"
dialoguer = "0.11"
comfy-table = "7"
human_bytes = "0.4"
serde_json = "1"

```
</file>

<file path=".\core\Cargo.toml" cps="1.20" level="3">
```toml
[package]
name = "repomd-core"
version = "0.1.0"
edition = "2021"

[dependencies]
ignore = "0.4"
tiktoken-rs = "0.6"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
walkdir = "2.5"
mime_guess = "2.0"
tree-sitter = "0.20"
tree-sitter-javascript = "0.20"
tree-sitter-python = "0.20"
tree-sitter-go = "0.20"
tree-sitter-rust = "0.20"
regex = "1.10"

```
</file>

<file path=".\extensions\github-action\action.yml" cps="1.20" level="3">
```yml
name: 'Generate repomd Context'
description: 'Automatically generate a repo.md file containing compressed codebase context'
branding:
  icon: 'file-text'
  color: 'blue'

inputs:
  preset:
    description: 'Compression preset (light, medium, aggressive, ultra)'
    required: false
    default: 'medium'
  max_tokens:
    description: 'Target token budget'
    required: false
    default: '50000'
  output:
    description: 'Output file name'
    required: false
    default: 'repo.md'

runs:
  using: 'composite'
  steps:
    - name: Download repomd CLI
      shell: bash
      run: |
        curl -fsSL https://repomd.dev/install | sh
    - name: Generate Context
      shell: bash
      run: |
        repomd --preset ${{ inputs.preset }} --tokens ${{ inputs.max_tokens }} --output ${{ inputs.output }}

```
</file>

<file path=".\web\.eslintrc.json" cps="1.20" level="3">
```json
{
  "extends": "next/core-web-vitals"
}

```
</file>

<file path=".\web\package.json" cps="1.20" level="3">
```json
{
  "name": "web",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint"
  },
  "dependencies": {
    "react": "^18",
    "react-dom": "^18",
    "next": "14.2.3"
  },
  "devDependencies": {
    "typescript": "^5",
    "@types/node": "^20",
    "@types/react": "^18",
    "@types/react-dom": "^18",
    "postcss": "^8",
    "tailwindcss": "^3.4.1",
    "eslint": "^8",
    "eslint-config-next": "14.2.3"
  }
}

```
</file>

<file path=".\web\tsconfig.json" cps="1.20" level="3">
```json
{
  "compilerOptions": {
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [
      {
        "name": "next"
      }
    ],
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}

```
</file>

<file path=".\format_test.py" cps="0.50" level="3">
```py
[Summarized File (Test)] 1687 bytes removed to save tokens.
```
</file>


---
**Total Tokens (excluding prompt): 4697**
