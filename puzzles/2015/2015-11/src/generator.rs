pub struct PasswordGenerator {
    letters: Vec<u8>,
}

impl PasswordGenerator {
    pub fn new(input: &str) -> Self {
        Self {
            letters: input.as_bytes().to_vec(),
        }
    }
}

impl Iterator for PasswordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // Reverse indexes so we can work back to front
        for index in (0..self.letters.len()).rev() {
            // If the letter is `z`, we set it to a and increment the one before
            if self.letters[index] == b'z' {
                self.letters[index] = b'a';
            } else {
                // Next letter from the back is not a `z` which means we increment it
                self.letters[index] += 1;
                // Now we produce the next password
                break;
            }
        }

        String::from_utf8(self.letters.clone()).ok()
    }
}
