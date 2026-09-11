pub fn part_one(input: &str) -> String {
    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    let mut row: usize = 1;
    let mut column: usize = 1;

    let mut code = String::new();

    for line in input.trim().lines() {
        for character in line.chars() {
            match character {
                'U' => row = row.saturating_sub(1),
                'D' => {
                    if row < keypad[column].len() - 1 {
                        row = row.saturating_add(1)
                    }
                }
                'L' => column = column.saturating_sub(1),
                'R' => {
                    if column < keypad.len() - 1 {
                        column = column.saturating_add(1)
                    }
                }
                _ => panic!("Unexpected character"),
            }
        }

        code.push(keypad[row][column]);
    }

    code
}
