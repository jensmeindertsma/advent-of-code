use std::collections::BTreeMap;

pub const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

pub fn parse_input(input: &str) -> Vec<Command> {
    let mut lines = input.lines().peekable();
    let mut commands: Vec<Command> = Vec::new();

    while let Some(line) = lines.next() {
        if !line.starts_with('$') {
            panic!("A line following `ls` wasn't properly parsed.")
        }

        // Skip the `$`.
        let mut slices = line.split_whitespace().skip(1);

        let command = match slices.next().unwrap() {
            "cd" => {
                let direction = match slices.next().unwrap() {
                    "/" => Direction::Root,
                    ".." => Direction::Up,
                    name => Direction::Down { name },
                };

                Command::Move(direction)
            }
            "ls" => {
                let mut entries = Vec::new();

                // Collect up all the following lines until the next command.
                // These lines form the output of the `ls` command.
                while let Some(line) = lines.next_if(|line| !line.starts_with('$')) {
                    let mut slices = line.split_whitespace();

                    let entry = match slices.next().unwrap() {
                        "dir" => Entry::Directory,
                        other => Entry::File {
                            size: other.parse().unwrap(),
                        },
                    };

                    entries.push(entry);
                }

                Command::List(entries)
            }
            other => panic!("Unexpected command: {other}"),
        };

        commands.push(command)
    }

    commands
}

pub fn generate_tree<'a>(commands: &'a [Command]) -> BTreeMap<Vec<&'a str>, usize> {
    let mut current_path: Vec<&str> = Vec::new();
    let mut size_tree: BTreeMap<Vec<&str>, usize> = BTreeMap::new();

    for command in commands {
        match command {
            Command::Move(Direction::Root) => current_path.push("/"),
            Command::Move(Direction::Up) => {
                current_path.pop();
            }
            Command::Move(Direction::Down { name }) => current_path.push(name),
            Command::List(entries) => {
                let sum: usize = entries
                    .iter()
                    .filter_map(|entry| match entry {
                        Entry::File { size, .. } => Some(size),
                        Entry::Directory { .. } => None,
                    })
                    .sum();

                for depth in 0..current_path.len() {
                    // Set the value for this path in the map to the sum of its files.
                    // Nested directories are ignored but they'll get their own entry
                    size_tree
                        .entry(current_path[0..=depth].to_vec())
                        .and_modify(|value| *value += sum)
                        .or_insert(sum);
                }
            }
        }
    }

    size_tree
}

#[derive(Debug)]
pub enum Command<'a> {
    Move(Direction<'a>),
    List(Vec<Entry>),
}

#[derive(Debug)]
pub enum Direction<'a> {
    Root,
    Up,
    Down { name: &'a str },
}

#[derive(Debug)]
pub enum Entry {
    File { size: usize },
    Directory,
}
