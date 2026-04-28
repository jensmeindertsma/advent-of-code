use md5::{Digest, Md5};
use std::fmt::Write;

pub fn part_one(input: &str) -> usize {
    let prefix = input.trim();

    let mut buffer = String::with_capacity(prefix.len() + 20);
    buffer.push_str(prefix);

    for number in 1.. {
        // Keep prefix only
        buffer.truncate(prefix.len());

        write!(buffer, "{}", number).unwrap();

        let result = Md5::digest(buffer.as_bytes());

        // Check if first 20 bits (4 hex digits) are zero
        if result[0] == 0 && result[1] == 0 && (result[2] & 0xF0) == 0 {
            return number;
        }
    }

    unreachable!()
}
