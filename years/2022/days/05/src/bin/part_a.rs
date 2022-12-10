use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let crates = calculate_crates(&input);

    println!("Crates: {crates}");
}

fn calculate_crates(input: &str) -> String {
    let (stacks_section, instructions_section) = input.split_once("\n\n").unwrap();

    let mut stack_lines = stacks_section.lines().rev();

    let mut stacks = Vec::from_iter(
        stack_lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|_value| -> Vec<&str> { Vec::new() }),
    );

    stack_lines.for_each(|line| {
        let mut crates: Vec<Option<&str>> = Vec::new();

        line.as_bytes().chunks(4).for_each(|chunk| {
            let segment = std::str::from_utf8(&chunk[1..2]).unwrap();

            if segment.chars().all(|char| char == ' ') {
                crates.push(None)
            } else {
                crates.push(Some(segment));
            }
        });

        crates.iter().enumerate().for_each(|(stack, crate_name)| {
            if let Some(crate_name) = crate_name {
                stacks[stack].push(crate_name)
            };
        })
    });

    instructions_section.lines().for_each(|line| {
        let count: usize = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        let from: usize = line.split_whitespace().nth(3).unwrap().parse().unwrap();
        let to: usize = line.split_whitespace().last().unwrap().parse().unwrap();

        println!("Move {count} from {from} to {to}");
        for _ in 0..count {
            if let Some(elf_crate) = stacks[from - 1].pop() {
                stacks[to - 1].push(elf_crate)
            };
        }
    });

    stacks
        .iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<Vec<&str>>()
        .concat()
}

#[cfg(test)]
mod tests {
    use day_05::TEST_INPUT;

    use super::calculate_crates;

    #[test]
    fn example_list() {
        assert_eq!(calculate_crates(TEST_INPUT), "CMZ")
    }
}
