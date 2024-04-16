# kzgpad-rs

`kzgpad-rs` is a Rust library that provides functions for padding and unpadding bytes. It is particularly useful when working with cryptographic primitives or encoding schemes that require specific byte alignments. For more detail on the rationale for this blob encoding, see the [documentation][1].

## Features

- `convert_by_padding_empty_byte`: Inserts an empty byte at the front of every 31 bytes, ensuring that every 32 bytes are within the valid range of a field element for the bn254 curve.
- `remove_empty_byte_from_padded_bytes`: Removes the first byte from every 32 bytes, reversing the changes made by `convert_by_padding_empty_byte`.

## Installation

To use `kzgpad-rs` in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
kzgpad-rs = { git = "https://github.com/Layr-Labs/kzgpad-rs.git" tag = "v0.1.0" }
```

## Usage

Here's an example of how to use the `kzgpad-rs` functions:

```rust
use kzgpad_rs::{convert_by_padding_empty_byte, remove_empty_byte_from_padded_bytes};

fn main() {
    let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];

    // Convert the data by padding empty bytes
    let padded_data = convert_by_padding_empty_byte(&data);

    // disperse padded_data to EigenDA

    // ...

    // retrieve padded_data from EigenDA

    // Remove the empty bytes from the padded data
    let unpadded_data = remove_empty_byte_from_padded_bytes(&padded_data);

    assert_eq!(data, unpadded_data);
}
```

## Function Details

### `convert_by_padding_empty_byte`

```rust
pub fn convert_by_padding_empty_byte(data: &[u8]) -> Vec<u8>
```

This function takes a slice of bytes (`data`) and inserts an empty byte at the front of every 31 bytes. The empty byte is padded at the low address because we use big endian to interpret a field element. This ensures that every 32 bytes are within the valid range of a field element for the bn254 curve.

If the input data is not a multiple of 31, the remainder is added to the output by inserting a 0 and the remainder. The output does not necessarily need to be a multiple of 32.

### `remove_empty_byte_from_padded_bytes`

```rust
pub fn remove_empty_byte_from_padded_bytes(data: &[u8]) -> Vec<u8>
```

This function takes a slice of bytes (`data`) and removes the first byte from every 32 bytes. It reverses the changes made by the `convert_by_padding_empty_byte` function.

The function does not assume that the input is a multiple of 32 bytes. For the remainder of the input, the first byte is taken out, and the rest is appended to the output.

## License

This project is licensed under the [MIT License](LICENSE).

[1]: https://docs.eigenlayer.xyz/eigenda/rollup-guides/blob-encoding
