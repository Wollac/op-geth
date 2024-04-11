#[repr(C)]
pub struct VarLengthArray {
    ptr: *const u8, // Pointer to the array data
    len: usize,     // Length of the array
}

/// Verifies a receipt for a given image_id.
/// Returns 0 if the receipt is valid, 1 if the receipt did not deserialize, 2 if the receipt did not verify.
#[no_mangle]
pub extern "C" fn verify(image_id: *const [u8; 32], receipt: VarLengthArray) -> u8 {
    let image_id: [u8; 32] = unsafe {
        assert!(!image_id.is_null(), "Pointer must not be null");
        *image_id
    };
    let receipt_bytes: &[u8] = unsafe {
        assert!(!receipt.ptr.is_null(), "Pointer must not be null");
        std::slice::from_raw_parts(receipt.ptr, receipt.len)
    };

    let receipt: risc0_zkvm::Receipt = match bincode::deserialize(receipt_bytes) {
        Ok(receipt) => receipt,
        Err(_) => return ErrorCode::DeserializeError as u8,
    };
    match receipt.verify(image_id) {
        Ok(_) => return ErrorCode::Ok as u8,
        Err(_) => return ErrorCode::VerifyError as u8,
    }
}

#[repr(u8)]
enum ErrorCode {
    Ok = 0,
    VerifyError = 1,
    DeserializeError = 2,
}
