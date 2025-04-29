pub fn part_2(input: &str) -> u32 {
    let input = input.trim();
    let size = input.len();

    let mut buffer = [0u8; 32];
    buffer[..size].copy_from_slice(input.as_bytes());

    let mut formatting_buffer = itoa::Buffer::new();
    for number in 0.. {
        let formatted_number_bytes = formatting_buffer.format(number).as_bytes();

        buffer[size..(size + formatted_number_bytes.len())].copy_from_slice(formatted_number_bytes);

        let hash = md5::compute(&buffer[..(size + formatted_number_bytes.len())]).0;

        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return number;
        }
    }

    panic!("No solution!")
}
