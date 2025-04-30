use itoa::Buffer as FormattingBuffer;

pub struct Hasher {
    prefix_length: usize,
    buffer: [u8; 32],
    formatter: FormattingBuffer,
    number: usize,
}

impl Hasher {
    pub fn new(input: &str) -> Self {
        let input = input.trim();

        let mut buffer = [0u8; 32];
        buffer[..input.len()].copy_from_slice(input.as_bytes());

        Self {
            prefix_length: input.len(),
            buffer,
            formatter: FormattingBuffer::new(),
            number: 0,
        }
    }

    pub fn current_number(&self) -> usize {
        self.number
    }
}

impl Iterator for Hasher {
    type Item = [u8; 16];

    fn next(&mut self) -> Option<Self::Item> {
        self.number += 1;

        let formatted_number_bytes = self.formatter.format(self.number).as_bytes();
        let formatted_number_length = formatted_number_bytes.len();

        self.buffer[self.prefix_length..(self.prefix_length + formatted_number_length)]
            .copy_from_slice(formatted_number_bytes);

        let hash = md5::compute(&self.buffer[..(self.prefix_length + formatted_number_length)]).0;

        Some(hash)
    }
}
