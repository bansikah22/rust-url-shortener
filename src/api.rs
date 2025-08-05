use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::lib::{expand, shorten};

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    short: String,
}

pub async fn run() {
    let app = Router::new()
        .route("/shorten", post(shorten_handler))
        .route("/u/:code", get(expand_handler));

    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server crashed");
}

async fn shorten_handler(Json(payload): Json<ShortenRequest>) -> impl IntoResponse {
    let short = shorten(&payload.url);
    Json(ShortenResponse { short })
}

async fn expand_handler(Path(code): Path<String>) -> Result<Redirect, (StatusCode, &'static str)> {
    match expand(&code) {
        Some(url) => Ok(Redirect::temporary(&url)),
        None => Err((StatusCode::NOT_FOUND, "Short code not found")),
    }
}
