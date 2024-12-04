use md5::{Digest, Md5};

pub fn part_2(input: &str) -> usize {
    let input = input.trim();

    (0..)
        .map(|n| {
            let mut hasher = Md5::new();

            hasher.update(format!("{}{n}", input.trim()).as_bytes());

            let output = hasher.finalize();
            let hash = format!("{:x}", output);

            (n, hash)
        })
        .find(|(_, hash)| hash.starts_with("000000"))
        .map(|(n, _)| n)
        .unwrap()
}
