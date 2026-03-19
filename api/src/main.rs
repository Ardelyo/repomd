use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use repomd_core::{Config, generate};

#[derive(Deserialize)]
struct GenerateRequest {
    source: String,
    preset: Option<String>,
    max_tokens: Option<usize>,
    options: Option<GenerateOptions>,
}

#[derive(Deserialize)]
struct GenerateOptions {
    include_tests: Option<bool>,
    focus: Option<String>,
}

#[derive(Serialize)]
struct GenerateResponse {
    id: String,
    output: String,
    stats: GenerationStats,
}

#[derive(Serialize)]
struct GenerationStats {
    output_tokens: usize,
    processing_time_ms: u64,
}

async fn generate_context(Json(payload): Json<GenerateRequest>) -> Json<GenerateResponse> {
    // In a full implementation, `payload.source` (e.g. github URL) would be cloned here.
    // For local Phase 3 MVP, it just parses the local '.' representation.
    
    let preset_val = match payload.preset.as_deref() {
        Some("light") => 1,
        Some("medium") => 2,
        Some("aggressive") => 3,
        Some("ultra") => 4,
        _ => 2,
    };

    let include_tests = payload.options.and_then(|o| o.include_tests).unwrap_or(false);

    let config = Config {
        target_tokens: payload.max_tokens.or(Some(50_000)),
        preset: Some(preset_val),
        output_path: None,
        include_tests,
    };

    let start = std::time::Instant::now();
    let result = generate(config).unwrap_or_else(|e| format!("Error: {}", e));
    let elapsed = start.elapsed().as_millis() as u64;

    // Approximate output token return 
    let tokens = result.len() / 4; 

    Json(GenerateResponse {
        id: format!("rmd_{}", uuid::Uuid::new_v4().simple()),
        output: result,
        stats: GenerationStats {
            output_tokens: tokens,
            processing_time_ms: elapsed,
        },
    })
}

async fn health_check() -> &'static str {
    "repomd API is operational"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/generate", post(generate_context))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("repomd API listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
