use risc0_zkvm::{sha::Digest, MaybePruned, Output, ReceiptClaim};

#[repr(C)]
pub struct VarLengthArray {
    ptr: *const u8, // Pointer to the array data
    len: usize,     // Length of the array
}

/// Verifies the given succinct proof.
#[no_mangle]
pub extern "C" fn verify(
    pre_state: *const [u8; 32],
    post_state: *const [u8; 32],
    input: *const [u8; 32],
    journal: *const [u8; 32],
    seal: VarLengthArray,
) -> ErrorCode {
    let pre_state: [u8; 32] = unsafe {
        assert!(!pre_state.is_null(), "Pointer must not be null");
        *pre_state
    };
    let post_state: [u8; 32] = unsafe {
        assert!(!post_state.is_null(), "Pointer must not be null");
        *post_state
    };
    let input: [u8; 32] = unsafe {
        assert!(!input.is_null(), "Pointer must not be null");
        *input
    };
    let journal: [u8; 32] = unsafe {
        assert!(!journal.is_null(), "Pointer must not be null");
        *journal
    };
    let seal: &[u8] = unsafe {
        assert!(!seal.ptr.is_null(), "Pointer must not be null");
        std::slice::from_raw_parts(seal.ptr, seal.len)
    };

    if seal.len() % 4 != 0 {
        return ErrorCode::InvalidSeal;
    }
    let seal = seal
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    let receipt = risc0_zkvm::SuccinctReceipt {
        seal,
        claim: ReceiptClaim {
            pre: MaybePruned::Pruned(pre_state.into()),
            post: MaybePruned::Pruned(post_state.into()),
            exit_code: risc0_zkvm::ExitCode::Halted(0),
            input: input.into(),
            output: MaybePruned::Value(Some(Output {
                journal: MaybePruned::Pruned(journal.into()),
                assumptions: MaybePruned::Pruned(Digest::ZERO),
            })),
        },
        control_id: Digest::ZERO,
    };

    match receipt.verify_integrity() {
        Ok(_) => return ErrorCode::Ok,
        Err(_) => return ErrorCode::VerifyError,
    }
}

#[repr(u8)]
pub enum ErrorCode {
    Ok = 0,
    VerifyError = 1,
    InvalidSeal = 2,
}
