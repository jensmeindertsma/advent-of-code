use crate::hasher::Hasher;

pub fn part_2(input: &str) -> usize {
    let mut hasher = Hasher::new(input);

    while let Some(hash) = hasher.next() {
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return hasher.current_number();
        }
    }

    panic!("No solution!")
}
