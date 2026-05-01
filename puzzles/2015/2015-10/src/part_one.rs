pub fn part_one(input: &str) -> usize {
    (0..40)
        .fold(input.trim().to_owned(), |previous, _| {
            // Each iteration grows about 30% in length, so we allocate new memory with 35% more space
            // to prevent reallocation during this iteration.
            let new_size = ((previous.len() as f64) * 1.35) as usize;
            let mut next = String::with_capacity(new_size);

            let mut values = previous
                .chars()
                .map(|c| char::to_digit(c, 10).expect("character should be a valid digit"))
                .peekable();

            while let Some(value) = values.next() {
                let mut length = 1;

                while let Some(next) = values.peek()
                    && *next == value
                {
                    values.next();
                    length += 1
                }

                // TODO: we now know how long this sequence of the same value is.
                // so we insert <length><value> into the next string
                next.push(char::from_digit(length, 10).expect("digit should be valid character"));
                next.push(char::from_digit(value, 10).expect("digit should be valid character"));
            }

            next
        })
        .len()
}
