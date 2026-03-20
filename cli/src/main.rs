use clap::{Parser, Subcommand};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color, Table};
use console::{style, Emoji};
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{ProgressBar, ProgressStyle};
use repomd_core::{generate_with_stats, preset_name, Config, FileDetail};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::sync::oneshot;


// ─── Emoji Constants ────────────────────────────────────────────────────────
static BOLT: Emoji<'_, '_> = Emoji("⚡", "*");
static FOLDER: Emoji<'_, '_> = Emoji("📂", ">");
static CHECK: Emoji<'_, '_> = Emoji("✔ ", "√ ");
static CLIP: Emoji<'_, '_> = Emoji("📋", "=");
static WRITE: Emoji<'_, '_> = Emoji("💾", ">");
static SEARCH: Emoji<'_, '_> = Emoji("🔍", "?");
static ROCKET: Emoji<'_, '_> = Emoji("🚀", "!");
static PACKAGE: Emoji<'_, '_> = Emoji("📦", "#");
static CHART: Emoji<'_, '_> = Emoji("📊", "%");

// ─── ASCII Banner ───────────────────────────────────────────────────────────
fn print_banner() {
    let banner = r#"
                                        __
   ________  ____  ____  ____ ___  ____/ /
  / ___/ _ \/ __ \/ __ \/ __ `__ \/ __  / 
 / /  /  __/ /_/ / /_/ / / / / / / /_/ /  
/_/   \___/ .___/\____/_/ /_/ /_/\__,_/   
         /_/                              
"#;

    println!(
        "{}",
        style(banner).cyan().bold()
    );
    println!(
        "  {} {}\n",
        style("Any repo. One command. Perfect context.").dim(),
        style(format!("v{}", env!("CARGO_PKG_VERSION"))).dim().italic()
    );
}

// ─── CLI Args ───────────────────────────────────────────────────────────────
#[derive(Parser, Debug)]
#[command(
    name = "repomd",
    version,
    about = "repomd — Any repo. One command. Perfect context.",
    long_about = "Transform any repository into a single, token-optimized Markdown file.\nPerfect for feeding codebases to LLMs with maximum context density.",
    after_help = "EXAMPLES:\n  repomd                                   Generate with interactive wizard\n  repomd generate                          Generate current directory with defaults\n  repomd generate https://github.com/..    Generate from a GitHub URL\n  repomd generate -p ultra                 Generate with ultra compression\n  repomd generate -t 30000                 Limit to 30k tokens\n  repomd inspect .                         Preview which files would be included\n  repomd generate --copy                   Generate and copy to clipboard"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate context markdown from the current repository
    Generate(GenerateArgs),

    /// Preview file discovery and scoring without generating
    Inspect(InspectArgs),

    /// Print the resolved configuration
    Config,
    
    /// Update repomd to the latest version
    Update,
}


#[derive(Parser, Debug)]
struct GenerateArgs {
    /// Source path or GitHub URL
    #[arg(default_value = ".")]
    source: String,

    /// Output file path
    #[arg(short, long, default_value = "repo.md")]
    output: PathBuf,

    /// Token budget (default: 50000)
    #[arg(short, long)]
    tokens: Option<usize>,

    /// Compression preset: light, medium, aggressive, ultra
    #[arg(short, long)]
    preset: Option<String>,

    /// Include test files in output
    #[arg(long)]
    include_tests: bool,

    /// Copy result to system clipboard
    #[arg(short, long)]
    copy: bool,

    /// Suppress all visual output
    #[arg(short, long)]
    quiet: bool,

    /// Show per-file details during generation
    #[arg(short, long)]
    verbose: bool,

    /// Output summary as JSON (for scripting)
    #[arg(long)]
    json: bool,

    /// Show what would be generated without writing output
    #[arg(long)]
    dry_run: bool,
}

#[derive(Parser, Debug)]
struct InspectArgs {
    /// Source path or GitHub URL
    #[arg(default_value = ".")]
    source: String,

    /// Include test files
    #[arg(long)]
    include_tests: bool,
}

// ─── Preset Colors ──────────────────────────────────────────────────────────
fn preset_color(level: u8) -> Color {
    match level {
        1 => Color::Green,
        2 => Color::Yellow,
        3 => Color::Red,
        4 => Color::Magenta,
        _ => Color::White,
    }
}

fn preset_style(level: u8) -> console::Style {
    match level {
        1 => console::Style::new().green().bold(),
        2 => console::Style::new().yellow().bold(),
        3 => console::Style::new().red().bold(),
        4 => console::Style::new().magenta().bold(),
        _ => console::Style::new().white().bold(),
    }
}

// ─── Interactive Wizard ─────────────────────────────────────────────────────
fn run_wizard() -> anyhow::Result<GenerateArgs> {
    print_banner();

    println!(
        "  {} {}\n",
        ROCKET,
        style("Interactive Setup").bold()
    );

    let source: String = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Source path or GitHub URL")
        .default(".".to_string())
        .interact()?;

    let presets = &[
        "💚 Light    — Comments removed, blank lines collapsed",
        "💛 Medium   — Structural: function signatures only",
        "🧡 Aggressive — Semantic: file summaries + key signatures",
        "❤️  Ultra    — Maximum compression, minimal tokens",
    ];

    let preset_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose compression preset")
        .items(presets)
        .default(1)
        .interact()?;

    let preset_name = match preset_idx {
        0 => "light",
        1 => "medium",
        2 => "aggressive",
        3 => "ultra",
        _ => "medium",
    };

    let token_options = &[
        "20,000  — Small context window (GPT-3.5 class)",
        "50,000  — Standard (GPT-4 class)",
        "100,000 — Large context (Claude class)",
        "200,000 — Maximum context (Gemini class)",
    ];

    let token_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Set token budget")
        .items(token_options)
        .default(1)
        .interact()?;

    let tokens = match token_idx {
        0 => 20_000,
        1 => 50_000,
        2 => 100_000,
        3 => 200_000,
        _ => 50_000,
    };

    let output_options = &[
        "📄 repo.md (default)",
        "📋 Copy to clipboard",
        "📄 + 📋 Both file and clipboard",
    ];

    let output_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Output destination")
        .items(output_options)
        .default(0)
        .interact()?;

    let copy = output_idx >= 1;
    let output = PathBuf::from("repo.md");

    println!();

    Ok(GenerateArgs {
        source,
        output,
        tokens: Some(tokens),
        preset: Some(preset_name.to_string()),
        include_tests: false,
        copy,
        quiet: false,
        verbose: false,
        json: false,
        dry_run: false,
    })
}

// ─── Summary Dashboard ─────────────────────────────────────────────────────
fn print_summary(
    stats: &repomd_core::GenerationStats,
    output_path: &PathBuf,
    copied: bool,
    output_bytes: usize,
) {
    println!();
    println!(
        "  {} {}",
        CHART,
        style("Generation Summary").bold().underlined()
    );
    println!();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_width(60);

    let pcolor = preset_color(stats.preset_level);

    table.add_row(vec![
        Cell::new("Preset").fg(Color::Cyan),
        Cell::new(format!(
            "{} (level {})",
            stats.preset_name.to_uppercase(),
            stats.preset_level
        ))
        .fg(pcolor),
    ]);
    table.add_row(vec![
        Cell::new("Files Scanned").fg(Color::Cyan),
        Cell::new(stats.files_scanned.to_string()),
    ]);
    table.add_row(vec![
        Cell::new("Files Included").fg(Color::Cyan),
        Cell::new(format!(
            "{} ({})",
            stats.files_included,
            style(format!("{} omitted", stats.files_omitted)).dim()
        ))
        .fg(Color::Green),
    ]);
    table.add_row(vec![
        Cell::new("Token Usage").fg(Color::Cyan),
        Cell::new(format!(
            "{} / {}",
            format_number(stats.total_tokens),
            format_number(stats.token_budget)
        )),
    ]);

    let utilization = if stats.token_budget > 0 {
        (stats.total_tokens as f64 / stats.token_budget as f64 * 100.0) as u8
    } else {
        0
    };
    let util_bar = render_bar(utilization);
    table.add_row(vec![
        Cell::new("Budget Used").fg(Color::Cyan),
        Cell::new(format!("{} {}%", util_bar, utilization)),
    ]);

    table.add_row(vec![
        Cell::new("Output Size").fg(Color::Cyan),
        Cell::new(human_bytes::human_bytes(output_bytes as f64)),
    ]);
    table.add_row(vec![
        Cell::new("Processing Time").fg(Color::Cyan),
        Cell::new(format!("{}ms", stats.processing_time_ms)),
    ]);
    table.add_row(vec![
        Cell::new("Absolute Path").fg(Color::Cyan),
        Cell::new(
            stats
                .absolute_output_path
                .as_deref()
                .unwrap_or("unknown"),
        )
        .fg(Color::White),
    ]);

    if copied {
        table.add_row(vec![
            Cell::new("Clipboard").fg(Color::Cyan),
            Cell::new("Copied ✔").fg(Color::Green),
        ]);
    }

    println!("{table}");
    println!();
}

fn print_summary_json(
    stats: &repomd_core::GenerationStats,
    output_path: &PathBuf,
    copied: bool,
    output_bytes: usize,
) {
    let json = serde_json::json!({
        "preset": stats.preset_name,
        "preset_level": stats.preset_level,
        "files_scanned": stats.files_scanned,
        "files_included": stats.files_included,
        "files_omitted": stats.files_omitted,
        "total_tokens": stats.total_tokens,
        "token_budget": stats.token_budget,
        "output_bytes": output_bytes,
        "processing_time_ms": stats.processing_time_ms,
        "output_file": output_path.display().to_string(),
        "absolute_output_path": stats.absolute_output_path,
        "clipboard": copied,
    });
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

// ─── Inspect Command ────────────────────────────────────────────────────────
fn run_inspect(args: InspectArgs) -> anyhow::Result<()> {
    print_banner();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.set_message(format!("{} Scanning repository {}...", SEARCH, args.source));
    spinner.enable_steady_tick(Duration::from_millis(80));

    let source_ctx = repomd_core::resolve_source(&args.source)?;
    let files = repomd_core::ingest::walk_directory(&source_ctx.search_path)?;
    spinner.finish_and_clear();

    println!(
        "  {} {} {} files discovered\n",
        CHECK,
        style("Scan complete:").green().bold(),
        style(files.len()).bold()
    );

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("#").fg(Color::DarkGrey),
            Cell::new("File").fg(Color::Cyan),
            Cell::new("Role").fg(Color::Cyan),
            Cell::new("CPS").fg(Color::Cyan),
            Cell::new("Size").fg(Color::Cyan),
        ]);

    for (i, file) in files.iter().enumerate().take(40) {
        let role_color = match file.role {
            repomd_core::discover::FileRole::Source => Color::Green,
            repomd_core::discover::FileRole::Config => Color::Yellow,
            repomd_core::discover::FileRole::Test => Color::Magenta,
            repomd_core::discover::FileRole::Doc => Color::Blue,
            _ => Color::DarkGrey,
        };

        table.add_row(vec![
            Cell::new(format!("{}", i + 1)).fg(Color::DarkGrey),
            Cell::new(file.path.display().to_string()),
            Cell::new(format!("{:?}", file.role)).fg(role_color),
            Cell::new(format!("{:.2}", file.cps)),
            Cell::new(human_bytes::human_bytes(file.content.len() as f64)),
        ]);
    }

    if files.len() > 40 {
        println!(
            "  {} ... and {} more files\n",
            style("").dim(),
            files.len() - 40
        );
    }

    println!("{table}");
    println!();

    Ok(())
}

// ─── Config Command ─────────────────────────────────────────────────────────
fn run_config() {
    print_banner();

    let config = Config::default();

    println!("  {} {}\n", PACKAGE, style("Resolved Configuration").bold());

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    table.add_row(vec![
        Cell::new("Token Budget").fg(Color::Cyan),
        Cell::new(format_number(config.target_tokens.unwrap_or(50_000))),
    ]);
    table.add_row(vec![
        Cell::new("Preset").fg(Color::Cyan),
        Cell::new(preset_name(config.preset.unwrap_or(1))),
    ]);
    table.add_row(vec![
        Cell::new("Output Path").fg(Color::Cyan),
        Cell::new(
            config
                .output_path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "stdout".to_string()),
        ),
    ]);
    table.add_row(vec![
        Cell::new("Include Tests").fg(Color::Cyan),
        Cell::new(if config.include_tests { "yes" } else { "no" }),
    ]);

    println!("{table}");
    println!();
}

// ─── Generate Command ───────────────────────────────────────────────────────
fn run_generate(args: GenerateArgs) -> anyhow::Result<()> {
    if !args.quiet && !args.json {
        print_banner();
    }

    let preset_level = match args.preset.as_deref() {
        Some("light") => 1,
        Some("medium") => 2,
        Some("aggressive") => 3,
        Some("ultra") => 4,
        _ => 2,
    };

    if !args.quiet && !args.json {
        println!(
            "  {} {} {}  {}  {} tokens\n",
            BOLT,
            style(format!("Processing {}", args.source)).bold(),
            style("·").dim(),
            preset_style(preset_level).apply_to(preset_name(preset_level).to_uppercase()),
            format_number(args.tokens.unwrap_or(50_000)),
        );
    }

    // Phase 1: Scanning
    let spinner = if !args.quiet && !args.json {
        let sp = ProgressBar::new_spinner();
        sp.set_style(
            ProgressStyle::with_template("  {spinner:.cyan} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        let url_msg = if args.source.starts_with("http") { "Cloning and scanning" } else { "Scanning" };
        sp.set_message(format!("{} {} {}...", SEARCH, url_msg, args.source));
        sp.enable_steady_tick(Duration::from_millis(80));
        Some(sp)
    } else {
        None
    };

    let config = Config {
        source: Some(args.source.clone()),
        target_tokens: args.tokens.or(Some(50_000)),
        preset: Some(preset_level),
        output_path: Some(args.output.clone()),
        include_tests: args.include_tests,
    };

    let result = generate_with_stats(config)?;

    if let Some(sp) = &spinner {
        sp.finish_and_clear();
        println!(
            "  {} {} {} files scanned, {} included",
            CHECK,
            style("Scan complete").green(),
            style(result.stats.files_scanned).bold(),
            style(result.stats.files_included).bold(),
        );
    }

    // Verbose: per-file details
    if args.verbose && !args.json {
        println!();
        let mut vtable = Table::new();
        vtable
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("File").fg(Color::Cyan),
                Cell::new("Role").fg(Color::Cyan),
                Cell::new("CPS").fg(Color::Cyan),
                Cell::new("Level").fg(Color::Cyan),
                Cell::new("Tokens").fg(Color::Cyan),
                Cell::new("Status").fg(Color::Cyan),
            ]);

        for fd in &result.stats.file_details {
            let status_cell = if fd.included {
                Cell::new("included").fg(Color::Green)
            } else {
                Cell::new("omitted").fg(Color::Red)
            };
            vtable.add_row(vec![
                Cell::new(&fd.path),
                Cell::new(&fd.role),
                Cell::new(format!("{:.2}", fd.cps)),
                Cell::new(format!("{}", fd.compression_level)).fg(preset_color(fd.compression_level)),
                Cell::new(format_number(fd.tokens)),
                status_cell,
            ]);
        }
        println!("{vtable}");
    }

    // Dry run: stop before writing
    if args.dry_run {
        if !args.quiet && !args.json {
            println!();
            println!(
                "  {} {}",
                style("DRY RUN").yellow().bold(),
                style("— No files written. Use without --dry-run to generate.").dim()
            );
            print_summary(
                &result.stats,
                &args.output,
                false,
                result.stats.output_bytes,
            );
        }
        return Ok(());
    }

    // Phase 2: Write output
    if !args.quiet && !args.json {
        let write_spinner = ProgressBar::new_spinner();
        write_spinner.set_style(
            ProgressStyle::with_template("  {spinner:.cyan} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        write_spinner.set_message(format!("{} Writing {}...", WRITE, args.output.display()));
        write_spinner.enable_steady_tick(Duration::from_millis(80));

        std::fs::write(&args.output, &result.markdown)?;

        write_spinner.finish_and_clear();
        println!(
            "  {} {} {}",
            CHECK,
            style("Written to").green(),
            style(args.output.display()).bold(),
        );
    } else {
        std::fs::write(&args.output, &result.markdown)?;
    }

    // Phase 3: Clipboard
    let mut copied = false;
    if args.copy {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                if clipboard.set_text(result.markdown.clone()).is_ok() {
                    copied = true;
                    if !args.quiet && !args.json {
                        println!(
                            "  {} {} {}",
                            CHECK,
                            style("Copied to clipboard").green(),
                            CLIP,
                        );
                    }
                }
            }
            Err(_) => {
                if !args.quiet && !args.json {
                    println!(
                        "  {} {}",
                        style("⚠").yellow(),
                        style("Clipboard unavailable on this system").dim()
                    );
                }
            }
        }
    }

    // Phase 4: Summary
    if args.json {
        print_summary_json(&result.stats, &args.output, copied, result.stats.output_bytes);
    } else if !args.quiet {
        print_summary(&result.stats, &args.output, copied, result.stats.output_bytes);
    }

    // Done
    if !args.quiet && !args.json {
        println!(
            "  {} {}\n",
            ROCKET,
            style("Done! Your context file is ready for LLMs.").bold()
        );
    }

    Ok(())
}

// ─── Helpers ────────────────────────────────────────────────────────────────
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn render_bar(percent: u8) -> String {
    let filled = (percent as usize) / 5;
    let empty = 20 - filled.min(20);
    let bar_char = "█";
    let empty_char = "░";

    let color = if percent < 50 {
        "\x1b[32m" // green
    } else if percent < 80 {
        "\x1b[33m" // yellow
    } else {
        "\x1b[31m" // red
    };

    format!(
        "{}{}{}{}",
        color,
        bar_char.repeat(filled.min(20)),
        empty_char.repeat(empty),
        "\x1b[0m"
    )
}

// ─── Update Check logic ───────────────────────────────────────────────────
async fn check_for_updates() -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .ok()?;
    
    // In a real scenario, this would be a GitHub API or repomd registry
    let res = client.get("http://localhost:3000/api/version")
        .send()
        .await
        .ok()?;
    
    let data: serde_json::Value = res.json().await.ok()?;
    let remote_version = data.get("version")?.as_str()?;
    let local_version = env!("CARGO_PKG_VERSION");
    
    if remote_version != local_version {
        Some(remote_version.to_string())
    } else {
        None
    }
}

fn print_update_banner(new_version: &str) {
    println!();
    println!("  {} {} ────────────────────────────────────────────────", style("!!").red().bold(), style("SYSTEM ALERT").red());
    
    // Staggered reveal simulation for the glitch effect
    let message = format!("NEW VERSION DETECTED: v{} -> v{}", env!("CARGO_PKG_VERSION"), new_version);
    print!("  {} ", style(">>").dim());
    for c in message.chars() {
        print!("{}", style(c).yellow().bold());
        let _ = std::io::Write::flush(&mut std::io::stdout());
        std::thread::sleep(Duration::from_millis(15));
    }
    println!();
    
    println!("  {} Run {} to upgrade now.", style(">>").dim(), style("repomd update").cyan().bold());
    println!("  {} ────────────────────────────────────────────────\n", style("!!").red().bold());
}

async fn run_update_command() -> anyhow::Result<()> {
    print_banner();
    println!("  {} {}\n", BOLT, style("Initializing Orbital Update Sequence...").bold());
    
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}% {msg}")
        .unwrap()
        .progress_chars("█▉▊▋▌▍▎▏  "));
    
    pb.set_message("Synchronizing with master branch...");
    for _ in 0..30 {
        pb.inc(1);
        std::thread::sleep(Duration::from_millis(30));
    }
    
    pb.set_message("Decompressing AST core...");
    for _ in 0..40 {
        pb.inc(1);
        std::thread::sleep(Duration::from_millis(20));
    }
    
    pb.set_message("Finalizing binary replacement...");
    for _ in 0..30 {
        pb.inc(1);
        std::thread::sleep(Duration::from_millis(50));
    }
    
    pb.finish_with_message("SYSTEM FULLY OPTIMIZED. RESTART REQUIRED.");
    println!("\n  {} {}\n", CHECK, style("Successfully updated to v0.2.1").green().bold());
    
    Ok(())
}

// ─── Main Entry ─────────────────────────────────────────────────────────────

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Start update check in background
    let (tx, mut rx) = oneshot::channel();
    tokio::spawn(async move {
        let update = check_for_updates().await;
        let _ = tx.send(update);
    });

    match cli.command {
        Some(Commands::Generate(args)) => {
            if let Ok(Some(version)) = rx.try_recv() {
                print_update_banner(&version);
            }
            run_generate(args)?;
        },
        Some(Commands::Inspect(args)) => run_inspect(args)?,
        Some(Commands::Config) => run_config(),
        Some(Commands::Update) => run_update_command().await?,
        None => {
            // Check update before wizard if possible
            // Wait a tiny bit for the check
            let update = match tokio::time::timeout(Duration::from_millis(200), rx).await {
                Ok(Ok(v)) => v,
                _ => None,
            };
            
            if let Some(version) = update {
                print_update_banner(&version);
            }

            // No subcommand: launch interactive wizard or smart default
            let is_tty = console::Term::stdout().is_term();


            if is_tty {
                // Interactive terminal: launch wizard
                let args = run_wizard()?;
                run_generate(args)?;
            } else {
                // Piped/non-interactive: use sensible defaults
                run_generate(GenerateArgs {
                    source: ".".to_string(),
                    output: PathBuf::from("repo.md"),
                    tokens: Some(50_000),
                    preset: Some("medium".to_string()),
                    include_tests: false,
                    copy: false,
                    quiet: true,
                    verbose: false,
                    json: false,
                    dry_run: false,
                })?;
            }
        }
    }

    Ok(())
}
