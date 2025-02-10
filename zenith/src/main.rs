// SPDX-FileCopyrightText: Copyright © 2020-2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use status_server::{ZStatus, ZStatusRequest, ZStatusResponse, ZStatusServer};
use tonic::transport::Server;

pub mod status_server {
    tonic::include_proto!("zenith_status");
    pub use z_status_server::*;
}

#[derive(Default)]
struct MyStatusService;

#[tonic::async_trait]
impl ZStatus for MyStatusService {
    async fn get_status(
        &self,
        _request: tonic::Request<ZStatusRequest>,
    ) -> Result<tonic::Response<ZStatusResponse>, tonic::Status> {
        let response = ZStatusResponse {
            message: "Service is running".to_string(),
        };

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let imp = MyStatusService;

    Server::builder()
        .add_service(ZStatusServer::new(imp))
        .serve(addr)
        .await?;

    println!("Hello, world!");
    Ok(())
}
