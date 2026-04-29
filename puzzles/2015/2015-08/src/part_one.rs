pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let literal_size = line.len();

            let mut memory_size = 0;
            let mut characters = line.chars();

            while let Some(character) = characters.next() {
                match character {
                    c if c.is_alphabetic() => memory_size += 1,

                    '"' => continue,

                    '\\' => {
                        let next = characters.next().unwrap();

                        match next {
                            '"' => {
                                // `\"` is one memory space
                                memory_size += 1
                            }

                            'x' => {
                                // `\x??`` hexadecimal escape code for one character
                                characters.next().unwrap();
                                characters.next().unwrap();
                                memory_size += 1
                            }

                            '\\' => {
                                // `\\` double backslash is one backslash
                                memory_size += 1
                            }

                            other => panic!("unhandled character `{other}`"),
                        }
                    }

                    other => panic!("unhandled character `{other}`"),
                }
            }

            literal_size - memory_size
        })
        .sum()
}
