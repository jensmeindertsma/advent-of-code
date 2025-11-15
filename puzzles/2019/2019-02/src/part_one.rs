pub fn part_one(input: &str) -> usize {
    let mut intcode = parse(input);

    intcode[1] = 12;
    intcode[2] = 2;

    compute(&mut intcode)
}

pub fn compute(code: &mut [usize]) -> usize {
    let mut instruction = 0;

    loop {
        let opcode = code[instruction * 4];

        if opcode == 99 {
            return code[0];
        }

        let location_1 = code[instruction * 4 + 1];
        let location_2 = code[instruction * 4 + 2];

        let value_1 = code[location_1];
        let value_2 = code[location_2];

        let output_value = match opcode {
            1 => value_1 + value_2,
            2 => value_1 * value_2,
            _ => panic!("unexpected opcode"),
        };

        let output_location = code[instruction * 4 + 3];
        code[output_location] = output_value;

        instruction += 1;
    }
}

pub fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
