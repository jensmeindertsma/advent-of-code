pub fn part_2(input: &str) -> u32 {
    let input = input.trim();
    let input_length = input.len();

    let mut buffer = [0u8; 32];
    buffer[..input_length].copy_from_slice(input.as_bytes());

    let mut formatting_buffer = itoa::Buffer::new();
    for number in 0.. {
        let formatted_number_bytes = formatting_buffer.format(number).as_bytes();
        let formatted_number_length = formatted_number_bytes.len();

        buffer[input_length..(input_length + formatted_number_length)]
            .copy_from_slice(formatted_number_bytes);

        let hash = md5::compute(&buffer[..(input_length + formatted_number_length)]).0;

        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return number;
        }
    }

    panic!("No solution!")
}
