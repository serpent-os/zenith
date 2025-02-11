// SPDX-FileCopyrightText: Copyright © 2020-2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::time::Duration;

use axum::{routing::get, Router};
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
use tracing::{event, Level};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::Format, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

/// Configures the tracing infrastructure with appropriate formatting and filtering
///
/// Sets up tracing with ANSI colors, uptime timer, and target information.
/// Uses environment variables for filtering or defaults to trace level.
fn configure_tracing() -> color_eyre::Result<()> {
    let f = Format::default()
        .with_ansi(true)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_file(false)
        .with_line_number(false)
        .with_target(true)
        .with_thread_ids(false);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace"));
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().event_format(f))
        .with(ErrorLayer::default())
        .init();

    Ok(())
}

/// Handler for the index route that returns a simple greeting
#[tracing::instrument]
async fn route_index() -> &'static str {
    event!(Level::INFO, "rendering");
    "Hello, World!"
}

/// Entry point for the web server application
///
/// Sets up error handling, tracing, routes and starts the server with graceful shutdown
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    configure_tracing()?;

    let app = Router::new()
        .route("/", get(route_index))
        .layer(TimeoutLayer::new(Duration::from_secs(12)));

    let addr = "[::1]:8081";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    event!(Level::INFO, addr, "Server now listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_handler())
        .await?;
    Ok(())
}

/// Handles graceful shutdown when ctrl-c is received
async fn shutdown_handler() {
    signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c event");
    tracing::warn!("Received shutdown signal, shutting down server");
}
