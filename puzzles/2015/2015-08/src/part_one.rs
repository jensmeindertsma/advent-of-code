pub fn part_one(input: &str) -> usize {
    input.trim().lines().fold(0, |mut total, line| {
        let mut characters = line.chars();
        let mut memory = 0;

        while let Some(character) = characters.next() {
            match character {
                '"' => {
                    // ignore start and end double quote
                    continue;
                }

                '\\' => match characters
                    .next()
                    .expect("backslash should be followed by escape")
                {
                    '\\' | '"' => {
                        memory += 1;
                    }

                    'x' => {
                        memory += 1;

                        characters.next();
                        characters.next();
                    }

                    _ => unreachable!("invalid escape"),
                },

                _ => {
                    memory += 1;
                }
            }
        }

        total += line.len() - memory;
        total
    })
}
