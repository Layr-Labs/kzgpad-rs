const BYTES_PER_SYMBOL: usize = 32;

// ConvertByPaddingEmptyByte takes bytes and inserts an empty byte at the front of every 31 bytes.
// The empty byte is padded at the low address, because we use big endian to interpret a field element.
// This ensures every 32 bytes are within the valid range of a field element for the bn254 curve.
// If the input data is not a multiple of 31, the remainder is added to the output by
// inserting a 0 and the remainder. The output does not necessarily need to be a multiple of 32.
pub fn convert_by_padding_empty_byte(data: &[u8]) -> Vec<u8> {
    let data_size = data.len();
    let parse_size = BYTES_PER_SYMBOL - 1;
    let put_size = BYTES_PER_SYMBOL;
    let data_len = (data_size + parse_size - 1) / parse_size;
    let mut valid_data = vec![0u8; data_len * put_size];
    let mut valid_end = valid_data.len();

    for i in 0..data_len {
        let start = i * parse_size;
        let mut end = start + parse_size;

        if end > data.len() {
            end = data.len();
            valid_end = end - start + 1 + i * put_size;
        }

        // With big endian, set the first byte to always be 0 to ensure data is within the valid range
        valid_data[i * BYTES_PER_SYMBOL] = 0x00;
        valid_data[i * BYTES_PER_SYMBOL + 1..i * BYTES_PER_SYMBOL + 1 + end - start]
            .copy_from_slice(&data[start..end]);
    }

    valid_data[..valid_end].to_vec()
}

// RemoveEmptyByteFromPaddedBytes takes bytes and removes the first byte from every 32 bytes.
// This reverses the change made by the function ConvertByPaddingEmptyByte.
// The function does not assume the input is a multiple of BYTES_PER_SYMBOL (32 bytes).
// For the remainder of the input, the first byte is taken out, and the rest is appended to
// the output.
pub fn remove_empty_byte_from_padded_bytes(data: &[u8]) -> Vec<u8> {
    let data_size = data.len();
    let parse_size = BYTES_PER_SYMBOL;
    let data_len = (data_size + parse_size - 1) / parse_size;
    let put_size = BYTES_PER_SYMBOL - 1;
    let mut valid_data = vec![0u8; data_len * put_size];
    let mut valid_len = valid_data.len();

    for i in 0..data_len {
        // Add 1 to leave the first empty byte untouched
        let start = i * parse_size + 1;
        let mut end = start + put_size;

        if end > data.len() {
            end = data.len();
            valid_len = end - start + i * put_size;
        }

        valid_data[i * put_size..i * put_size + end - start].copy_from_slice(&data[start..end]);
    }

    valid_data[..valid_len].to_vec()
}
