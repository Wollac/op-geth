// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg(feature = "generate")]

use hello_world_methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::{get_prover_server, sha::Digest, ExecutorEnv, InnerReceipt, ProverOpts, Receipt};

fn main() {
    let iterations = 100;
    let env = ExecutorEnv::builder()
        .write_slice(&[iterations])
        .build()
        .unwrap();
    let opts = ProverOpts::default();
    let prover = get_prover_server(&opts).unwrap();
    let receipt = prover.prove(env, MULTIPLY_ELF).unwrap();
    let composite_receipt = receipt.inner.composite().unwrap();
    let succinct_receipt = prover.compress(composite_receipt).unwrap();
    let receipt = Receipt {
        inner: InnerReceipt::Succinct(succinct_receipt),
        journal: receipt.journal,
    };
    let receipt_bytes = bincode::serialize(&receipt).unwrap();

    std::fs::write("receipt.bin", receipt_bytes).unwrap();
    let image_id = Digest::from(MULTIPLY_ID);

    println!("ImageID: {image_id}");

    let receipt_bytes = std::fs::read("receipt.bin").unwrap();
    let receipt: Receipt = bincode::deserialize(&receipt_bytes).unwrap();
    receipt.verify(MULTIPLY_ID).unwrap();

    println!("Receipt OK");
}
