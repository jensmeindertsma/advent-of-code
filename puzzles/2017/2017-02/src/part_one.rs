pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (min, max) = line
                .split_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                // now we need to find the biggest and smallest <usize>
                // we start with MAX and MIN, then for each
                // number we check if its bigger or smaller than the previous.
                .fold((usize::MAX, usize::MIN), |(min, max), x| {
                    // any number would be smaller than usize::MAX
                    // any number would be bigger than usize::MIN
                    (min.min(x), max.max(x))
                });

            max - min
        })
        .sum()
}
