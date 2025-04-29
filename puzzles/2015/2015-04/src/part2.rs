use itoa::Buffer;
use rayon::prelude::*;

pub fn part_2(input: &str) -> u32 {
    let input = input.trim();
    let input_bytes = input.as_bytes();
    let input_length = input.len();

    (0..u32::MAX)
        .into_par_iter()
        .find_any(|number| {
            let mut buffer = [0u8; 32];
            let mut formatting_buffer = Buffer::new();
            let number_formatted_bytes = formatting_buffer.format(*number).as_bytes();
            let total_length = input_length + number_formatted_bytes.len();

            // buffer[..input_length].copy_from_slice(input_bytes);
            // buffer[input_length..total_length].copy_from_slice(number_formatted_bytes);

            unsafe {
                std::ptr::copy_nonoverlapping(
                    input_bytes.as_ptr(),
                    buffer.as_mut_ptr(),
                    input_length,
                );
                std::ptr::copy_nonoverlapping(
                    number_formatted_bytes.as_ptr(),
                    buffer.as_mut_ptr().add(input_length),
                    number_formatted_bytes.len(),
                );
            }

            let hash = md5::compute(&buffer[..total_length]).0;

            hash[0] == 0 && hash[1] == 0 && hash[2] == 0
        })
        .unwrap()
}
