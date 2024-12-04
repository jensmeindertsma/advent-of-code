use md5::{Digest, Md5};

pub struct Miner {
    buffer: Vec<u8>,
    hasher: Md5,
    n: usize,
    length: usize,
}

impl Miner {
    pub fn new(input: &str) -> Self {
        // We prepare the buffer ahead of time with the input.
        // This means we only need to replace the latter part holding
        // the number during the hot loop.
        let mut buffer = Vec::with_capacity(32);
        buffer.extend_from_slice(input.as_bytes());

        Self {
            buffer,
            hasher: Md5::new(),
            n: 0,
            length: input.len(),
        }
    }
}

impl Iterator for Miner {
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.truncate(self.length);
        self.buffer.extend_from_slice(self.n.to_string().as_bytes());
        self.hasher.update(&mut self.buffer);
        let hash = self.hasher.finalize_reset();

        let result = (self.n, format!("{:x}", hash));

        self.n += 1;

        Some(result)
    }
}
