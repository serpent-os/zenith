// SPDX-FileCopyrightText: Copyright © 2020-2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::backtrace::Backtrace;
use std::error::Error;
use std::net::SocketAddr;

use thiserror::Error;
use tonic::transport;
use tracing::{debug, info, instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub mod status_server {
    tonic::include_proto!("zenith_status");
    pub use z_status_client::*;
}

#[derive(Debug, Error)]
enum ClientError {
    #[error("Failed to connect to server: {0}")]
    IO(#[from] std::io::Error),

    #[error("failed to connect to server: {0}")]
    Transport(#[from] transport::Error),

    #[error(transparent)]
    Status(#[from] tonic::Status),
}

#[instrument]
async fn run_client<S>(address: S) -> Result<(), ClientError>
where
    S: Into<SocketAddr> + std::fmt::Debug,
{
    let address = address.into();
    info!("connecting");

    let mut client =
        status_server::z_status_client::ZStatusClient::connect(format!("http://{}", address))
            .await?;

    let request = tonic::Request::new(status_server::ZStatusRequest {});

    let response = client.get_status(request).await?;

    let resp = response.into_inner();
    info!(?resp, "Response from server");

    Ok(())
}

fn configure_tracing() -> color_eyre::Result<()> {
    let f = Format::default()
        .with_ansi(true)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_file(false)
        .with_line_number(false)
        .with_target(true)
        .with_thread_ids(false);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().event_format(f))
        .with(ErrorLayer::default())
        .init();

    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::config::HookBuilder::default()
        .capture_span_trace_by_default(true)
        .install()?;

    configure_tracing()?;

    let addr: SocketAddr = "[::1]:50051".parse()?;
    run_client(addr).await?;

    Ok(())
}
