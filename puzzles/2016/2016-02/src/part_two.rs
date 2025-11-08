pub fn part_two(input: &str) -> String {
    let mut code = String::new();

    let keypad = [
        [None, None, Some('1'), None, None], //                          1
        [None, Some('2'), Some('3'), Some('4'), None], //              2 3 4
        [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')], //  5 6 7 8 9
        [None, Some('A'), Some('B'), Some('C'), None], //              A B C
        [None, None, Some('D'), None, None], //                          D
    ];

    // (2, 0) is the position of "5" where we'll start
    let mut position: (usize, usize) = (2, 0);

    for line in input.trim().lines() {
        for character in line.chars() {
            let (row, column) = position;

            match character {
                'D' => {
                    if row == 4 || keypad[row + 1][column].is_none() {
                        continue;
                    } else {
                        position = (row + 1, column)
                    }
                }
                'U' => {
                    if row == 0 || keypad[row - 1][column].is_none() {
                        continue;
                    } else {
                        position = (row - 1, column)
                    }
                }
                'L' => {
                    if column == 0 || keypad[row][column - 1].is_none() {
                        continue;
                    } else {
                        position = (row, column - 1)
                    }
                }
                'R' => {
                    if column == 4 || keypad[row][column + 1].is_none() {
                        continue;
                    } else {
                        position = (row, column + 1)
                    }
                }
                other => panic!("unexpected character `{other}`"),
            };
        }

        let (row, column) = position;
        code.push(keypad[row][column].unwrap());
    }

    code
}
