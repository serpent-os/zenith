// SPDX-FileCopyrightText: Copyright © 2020-2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::{fmt, net::SocketAddr};

use status_server::{ZStatus, ZStatusRequest, ZStatusResponse, ZStatusServer};
use tokio::signal;
use tonic::transport::Server;
use tracing::{debug, event, warn, Level};
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::EnvFilter;

pub mod status_server {
    tonic::include_proto!("zenith_status");
    pub use z_status_server::*;
}

#[derive(Debug)]
struct MyStatusService {
    started: std::time::Instant,
}

impl Default for MyStatusService {
    fn default() -> Self {
        Self {
            started: std::time::Instant::now(),
        }
    }
}

#[tonic::async_trait]
impl ZStatus for MyStatusService {
    async fn get_status(
        &self,
        request: tonic::Request<ZStatusRequest>,
    ) -> Result<tonic::Response<ZStatusResponse>, tonic::Status> {
        let response = ZStatusResponse {
            message: "Service is running".to_string(),
            uptime: self.started.elapsed().as_secs(),
            r#version: env!("CARGO_PKG_VERSION").to_string(),
            version_code: 0,
        };

        let whom = request.remote_addr();

        event!(Level::INFO, ?whom, "Received request");
        Ok(tonic::Response::new(response))
    }
}

async fn run_server<S>(address: S) -> Result<(), Box<dyn std::error::Error>>
where
    S: Into<SocketAddr> + fmt::Debug,
{
    let imp = MyStatusService::default();
    let address = address.into();

    event!(Level::INFO, ?address, "Starting server 🚀");
    Server::builder()
        .add_service(ZStatusServer::new(imp))
        .serve_with_shutdown(address, shutdown_handler())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = Format::default()
        .pretty()
        .with_ansi(true)
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_file(false)
        .with_line_number(false)
        .with_target(true)
        .with_thread_ids(false);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let r = tracing_subscriber::fmt()
        .event_format(f)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(r)?;

    debug!("Launching Zenith");

    let addr: SocketAddr = "[::1]:50051".parse()?;
    run_server(addr).await?;

    Ok(())
}

async fn shutdown_handler() {
    signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c event");
    warn!("Received shutdown signal, shutting down server");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_status() {
        let service = MyStatusService::default();
        let request = tonic::Request::new(ZStatusRequest {});

        // sleep for 1.2 seconds
        tokio::time::sleep(std::time::Duration::from_millis(1200)).await;

        let response = service.get_status(request).await.unwrap();
        let response = response.get_ref();

        assert_eq!(response.message, "Service is running");
        assert_eq!(response.version_code, 0);
        assert_eq!(response.version, env!("CARGO_PKG_VERSION"));
        assert!(response.uptime >= 1);
    }
}
