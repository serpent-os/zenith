// SPDX-FileCopyrightText: Copyright © 2020-2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

fn main() {
    tonic_build::compile_protos("proto/status.proto").expect("Failed to compile status.proto");
}
