pub fn generate_sequence(input: String) -> String {
    let mut result = String::new();
    let mut characters = input.chars().peekable();

    // Character by character we look ahead to see how many
    // of the same characters there are consecutively.
    while let Some(character) = characters.next() {
        let mut count = 1;

        // `peek()` does not consume, so if it's not what we are
        // looking for it comes again in the next iteration of the
        // while loop.
        while let Some(&next) = characters.peek() {
            // If the "streak" continues, we increment count and consume
            // the character, so the next `peek()` looks at the next character
            // and not the same one again.
            if next == character {
                count += 1;
                characters.next();
            } else {
                // "streak" ends so we break, produce output, and
                // start counting again.
                break;
            }
        }

        // After the whole "streak" has been counted, push the count
        // of the character followed by the character. For example:
        // - `21` -> `1211`, one two, one one
        // - `666344` -> `361324`
        result.push_str(&count.to_string());
        result.push(character);
    }

    result
}
