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

            // One hexadecimal byte contains two characters.
            // With indexding we check one byte at a time.
            // With `result[0]` == 0 we are checking both digit 1 and digit 2.
            // then for `result[1]`, we check digit 3 and 4.
            //
            // Then we us bitwise AND to look only at the first "digit" (4 bits) of the
            // 5th byte. The bitwise operator will set the second digit in this byte to
            // zero, so whether or not the `== 0` check passes depends only on the first
            // 4 bits in the byte. If they are all zero (hexadecimal zero) then the check passed.
            result[0] == 0 && result[1] == 0 && (result[2] & 0xF0) == 0
        })
        .unwrap()
}
