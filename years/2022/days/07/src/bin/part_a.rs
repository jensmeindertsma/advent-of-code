use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let size = calculate_size(&input);

    println!("Total size of directories: {size}");
}

fn calculate_size(input: &str) -> usize {
    let mut lines = input.lines().peekable();
    let mut operations: Vec<Operation> = Vec::new();

    while let Some(line) = lines.next() {
        if !line.starts_with('$') {
            panic!("A line following `ls` wasn't properly parsed.")
        }

        // Skip the `$`.
        let mut slices = line.split_whitespace().skip(1);

        let operation = match slices.next().unwrap() {
            "cd" => {
                let direction = match slices.next().unwrap() {
                    "/" => Direction::Root,
                    ".." => Direction::Up,
                    name => Direction::Down { name },
                };

                Operation::Move(direction)
            }
            "ls" => {
                let mut entries = Vec::new();

                // Collect up all the following lines until the next command.
                // These lines form the output of the `ls` command.
                while let Some(line) = lines.next_if(|line| !line.starts_with('$')) {
                    let mut slices = line.split_whitespace();

                    let entry = match slices.next().unwrap() {
                        "dir" => Entry::Directory {
                            name: slices.next().unwrap(),
                        },
                        other => Entry::File {
                            size: other.parse().unwrap(),
                            name: slices.next().unwrap(),
                        },
                    };

                    entries.push(entry);
                }

                Operation::List(entries)
            }
            other => panic!("Unexpected command: {other}"),
        };

        operations.push(operation);
    }

    println!("{operations:#?}");

    0
}

#[derive(Debug)]
enum Operation<'a> {
    Move(Direction<'a>),
    List(Vec<Entry<'a>>),
}

#[derive(Debug)]
enum Direction<'a> {
    Root,
    Up,
    Down { name: &'a str },
}

#[derive(Debug)]
enum Entry<'a> {
    File { size: u32, name: &'a str },
    Directory { name: &'a str },
}

#[cfg(test)]
mod tests {
    use day_07::TEST_INPUT;

    use super::calculate_size;

    #[test]
    fn test_input_a() {
        assert_eq!(calculate_size(TEST_INPUT), 95437)
    }
}
