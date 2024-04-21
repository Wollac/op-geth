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

use hello_world_methods::MULTIPLY_ELF;
use risc0_zkvm::sha::Digestible;
use risc0_zkvm::{get_prover_server, sha::Digest, ExecutorEnv, ProverOpts, SuccinctReceipt};

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

    std::fs::write("seal.bin", succinct_receipt.get_seal_bytes()).unwrap();
    let claim = succinct_receipt.claim;
    println!("PreState:\t{}", claim.pre.digest());
    println!("PostState:\t{}", claim.post.digest());
    println!("Input:  \t{}", claim.input);
    let journal = &claim.output.as_value().unwrap().as_ref().unwrap().journal;
    println!("Journal:\t{}", journal.digest());

    let seal_bytes = std::fs::read("seal.bin").unwrap();
    let seal = seal_bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    let receipt = SuccinctReceipt {
        seal,
        control_id: Digest::ZERO,
        claim,
    };
    receipt.verify_integrity().unwrap();

    println!("Receipt OK");
}
