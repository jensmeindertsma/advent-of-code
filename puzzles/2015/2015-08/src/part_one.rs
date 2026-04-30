pub fn part_one(input: &str) -> usize {
    let mut total = 0;

    for line in input.trim().lines() {
        let bytes = line.as_bytes();
        let mut i = 0;
        let mut memory = 0;

        while i < bytes.len() {
            match bytes[i] {
                b'"' => {
                    // ignore start and end double quote
                    i += 1;
                }

                b'\\' => match bytes[i + 1] {
                    b'\\' | b'"' => {
                        // backslash escape and double quote escape
                        // both take 1 byte in memory
                        memory += 1;

                        // we parsed 2 bytes so move ahead in memory
                        i += 2;
                    }

                    b'x' => {
                        // hexadecimal escape is 1 byte in memory
                        memory += 1;

                        // hexadecimal escape is 4 bytes in literal
                        i += 4;
                    }

                    _ => unreachable!("invalid escape"),
                },

                _ => {
                    memory += 1;
                    i += 1;
                }
            }
        }

        total += line.len() - memory;
    }

    total
}
