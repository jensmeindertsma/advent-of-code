pub fn part_two(input: &str) -> String {
    for a in input.trim().lines() {
        for b in input.trim().lines() {
            let mut shared = String::new();
            let mut differences = 0;

            for (ac, bc) in a.chars().zip(b.chars()) {
                if ac == bc {
                    shared.push(ac);
                } else {
                    differences += 1
                }
            }

            if differences == 1 {
                return shared;
            }
        }
    }

    unreachable!()
}
