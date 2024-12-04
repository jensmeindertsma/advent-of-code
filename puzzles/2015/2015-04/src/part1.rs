use md5::{Digest, Md5};

pub fn part_1(input: &str) -> usize {
    let input = input.trim();

    for n in 0.. {
        let mut hasher = Md5::new();

        hasher.update(format!("{}{n}", input.trim()).as_bytes());

        let hash = hasher.finalize();
        let string = format!("{:x}", hash);

        if string.starts_with("00000") {
            return n;
        }
    }

    unreachable!()
}
