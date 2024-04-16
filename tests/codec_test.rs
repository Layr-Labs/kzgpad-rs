use kzgpad_rs::{convert_by_padding_empty_byte, remove_empty_byte_from_padded_bytes};
use rand::Rng;

#[test]
fn test_codec() {
    let num_iterations = 100;

    for _ in 0..num_iterations {
        // Generate random data
        let data_size = rand::thread_rng().gen_range(1..1000);
        let mut randomly_generated_data = vec![0u8; data_size];
        rand::thread_rng().fill(&mut randomly_generated_data[..]);

        // Convert the data by padding empty bytes
        let padded_data = convert_by_padding_empty_byte(&randomly_generated_data);

        // Remove the empty bytes from the padded data
        let unpadded_data = remove_empty_byte_from_padded_bytes(&padded_data);

        // Check if the original data is equal to the unpadded data
        assert_eq!(randomly_generated_data, unpadded_data);
    }

    println!("Test passed for {} iterations!", num_iterations);
}
