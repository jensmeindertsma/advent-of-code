use md5::{Digest, Md5};
use std::fmt::Write;

pub fn part_one(input: &str) -> usize {
    let input = input.trim();
    let mut buffer = String::with_capacity(input.len() * 3);

    (1..)
        .find(|number| {
            write!(buffer, "{input}{number}").unwrap();

            let mut hasher = Md5::new();

            hasher.update(buffer.as_bytes());

            let result = hasher.finalize();

            buffer.clear();

            result.starts_with("00000".as_bytes())
        })
        .unwrap()
}
