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
            let (delta_row, delta_column) = match character {
                'D' => (1, 0),
                'U' => (-1, 0),

                'L' => (0, -1),
                'R' => (0, 1),
                other => panic!("unexpected character `{other}`"),
            };

            let (row, column) = position;

            let new_row = row.saturating_add_signed(delta_row);
            let new_column = column.saturating_add_signed(delta_column);

            let within_bounds = (0..5).contains(&new_row) && (0..5).contains(&new_column);

            if !within_bounds || keypad[new_row][new_column].is_none() {
                continue;
            }

            position = (new_row, new_column)
        }

        let (row, column) = position;

        code.push(keypad[row][column].expect("button should exist"));
        println!("code is now {code}")
    }

    code
}
