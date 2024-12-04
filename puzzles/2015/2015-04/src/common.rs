use md5::{Digest, Md5};

pub struct Miner {
    buffer: Vec<u8>,
    hasher: Md5,
    separation: usize,
    n: usize,
}

impl Miner {
    pub fn new(input: &str) -> Self {
        // usize::MAX is 10 digits
        let mut buffer = Vec::with_capacity(input.len() + 10);
        buffer.extend_from_slice(input.as_bytes());

        Self {
            buffer,
            hasher: Md5::new(),
            separation: input.len(),
            n: 0,
        }
    }
}

impl Iterator for Miner {
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.truncate(self.separation);
        self.buffer.extend_from_slice(self.n.to_string().as_bytes());
        self.hasher.update(&self.buffer);

        let hash = hex::encode(self.hasher.finalize_reset());

        let result = (self.n, hash);

        self.n += 1;

        Some(result)
    }
}
