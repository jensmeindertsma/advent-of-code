pub fn generate_sequence(input: String) -> String {
    let mut sets: Vec<String> = Vec::new();
    let mut characters = input.chars().peekable();

    while let Some(character) = characters.next() {
        let mut set = String::new();
        set.push(character);
        loop {
            if let Some(n) = characters.peek().cloned() {
                if n == character {
                    characters.next();
                    set.push(n);
                    continue;
                }
            }

            break;
        }
        sets.push(set)
    }

    let mut new_sequence = String::with_capacity(input.len() * 2);

    for set in sets {
        new_sequence.push_str(&format!("{}{}", set.len(), set.chars().next().unwrap()))
    }

    new_sequence
}
