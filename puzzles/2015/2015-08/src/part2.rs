pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            encode(line).len() - line.len()
        })
        .sum()
}

fn encode(string: &str) -> String {
    let mut string = string.to_string();
    string = string.replace('\\', "\\\\");
    format!("\"{}\"", string.replace('\"', "\\\""))
}
