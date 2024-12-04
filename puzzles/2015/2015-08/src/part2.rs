pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            line.encode().len() - line.len()
        })
        .sum()
}

trait Encodeable {
    fn encode(&self) -> String;
}

impl Encodeable for &str {
    fn encode(&self) -> String {
        let mut string = self.to_string();
        string = string.replace('\\', "\\\\");
        format!("\"{}\"", string.replace('\"', "\\\""))
    }
}
