use std::fmt::Write;

const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

pub fn part_1(input: &str) -> String {
    let input = input.trim();
    let mut buffer = String::with_capacity(input.len() * 2);

    (0..usize::MAX)
        .filter_map(|n| {
            buffer.clear();
            buffer.push_str(input);
            write!(buffer, "{n}").unwrap();

            let digest = md5::compute(&buffer);

            if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0) == 0 {
                let sixth_char_index = (digest[2] & 0x0F) as usize;
                let sixth_char = HEX_CHARS[sixth_char_index] as char;

                Some(sixth_char)
            } else {
                None
            }
        })
        .take(8)
        .collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1("abc"), "18f47a30");

        assert_eq!(part_1(INPUT), "1a3099aa");
    }
}
