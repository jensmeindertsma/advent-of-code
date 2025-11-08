pub fn part_one(input: &str) -> String {
    let mut code = String::new();

    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    // Middle position (5)
    let mut position: (usize, usize) = (1, 1);

    for line in input.trim().lines() {
        for character in line.chars() {
            let (row, column) = position;

            let new_position = match character {
                'D' => (row + 1, column),
                'U' => (row.saturating_sub(1), column),
                'L' => (row, column.saturating_sub(1)),
                'R' => (row, column + 1),
                other => panic!("unexpected character `{other}`"),
            };

            let (new_row, new_column) = new_position;
            if keypad
                .get(new_row)
                .and_then(|r| r.get(new_column))
                .is_some()
            {
                position = new_position
            }
        }

        let (row, column) = position;
        code.push(keypad[row][column]);
    }

    code
}
