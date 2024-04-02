#[repr(C)]
pub struct VariableLengthArray {
    ptr: *const u8, // Pointer to the array data
    len: usize,     // Length of the array
}

#[no_mangle]
pub extern "C" fn verify(image_id: *const [u8; 32], receipt: VariableLengthArray) -> u16 {
    let image_id: &[u8; 32] = unsafe {
        assert!(!image_id.is_null(), "Pointer must not be null");
        // Dereference the raw pointer to access the array. This is safe only if the above
        // conditions are met.
        &*image_id
    };

    let receipt: &[u8] = unsafe {
        assert!(!receipt.ptr.is_null(), "Pointer must not be null");
        std::slice::from_raw_parts(receipt.ptr, receipt.len)
    };

    let receipt: risc0_zkvm::Receipt = bincode::deserialize(receipt).unwrap();
    match receipt.verify(*image_id) {
        Ok(_) => return 1,
        Err(_) => return 0,
    }
}