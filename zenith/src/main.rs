// SPDX-FileCopyrightText: Copyright © 2020-2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use status_server::{ZStatus, ZStatusRequest, ZStatusResponse, ZStatusServer};
use tonic::transport::Server;

pub mod status_server {
    tonic::include_proto!("zenith_status");
    pub use z_status_server::*;
}

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
        _request: tonic::Request<ZStatusRequest>,
    ) -> Result<tonic::Response<ZStatusResponse>, tonic::Status> {
        let response = ZStatusResponse {
            message: "Service is running".to_string(),
            uptime: self.started.elapsed().as_secs(),
            r#version: env!("CARGO_PKG_VERSION").to_string(),
            version_code: 0,
        };

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let imp = MyStatusService::default();

    Server::builder()
        .add_service(ZStatusServer::new(imp))
        .serve(addr)
        .await?;

    println!("Hello, world!");
    Ok(())
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
