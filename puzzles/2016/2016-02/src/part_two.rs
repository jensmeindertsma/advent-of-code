pub fn part_two(input: &str) -> String {
    let keypad = [
        vec![None, None, Some('1'), None, None],
        vec![None, Some('2'), Some('3'), Some('4'), None],
        vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        vec![None, Some('A'), Some('B'), Some('C'), None],
        vec![None, None, Some('D'), None, None],
    ];

    let mut row: usize = 2;
    let mut column: usize = 0;

    let mut code = String::new();

    for line in input.trim().lines() {
        for character in line.chars() {
            println!(
                "{character} row {row} col {column}, {:?}",
                keypad[row][column]
            );

            match character {
                'U' => {
                    if let Some(key) = keypad.get(row - 1)
                        && key[column].is_some()
                    {
                        row -= 1
                    }
                }
                'D' => {
                    if let Some(key) = keypad.get(row + 1)
                        && key[column].is_some()
                    {
                        row += 1
                    }
                }
                'L' => {
                    if let Some(key) = keypad.get(row).and_then(|row| row.get(column - 1))
                        && key.is_some()
                    {
                        column -= 1
                    }
                }
                'R' => {
                    if let Some(key) = keypad.get(row).and_then(|row| row.get(column + 1))
                        && key.is_some()
                    {
                        column += 1
                    }
                }
                _ => panic!("Unexpected character"),
            }
        }

        println!(
            "line end {:?}",
            keypad.get(row).and_then(|row| row.get(column))
        );

        code.push(keypad[row][column].expect("every line should end at a button"));
    }

    code
}
