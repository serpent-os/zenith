// SPDX-FileCopyrightText: Copyright © 2020-2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

fn main() {
    println!("cargo:rerun-if-changed=../zenith/proto/status.proto");
    tonic_build::compile_protos("../zenith/proto/status.proto")
        .expect("Failed to compile status.proto");
}
