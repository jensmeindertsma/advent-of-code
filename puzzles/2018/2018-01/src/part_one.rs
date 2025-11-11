pub fn part_one(input: &str) -> isize {
    input
        .trim()
        // we have to deal with `+5, -6, +2` as well as all
        // those separate on a newline.
        .split(|c: char| c.is_whitespace() || c == ',')
        // in `+1, -1``, there is ['+1', ',' ' ', '-1'].
        // `::split()` splits when it sees the comma after
        // the `+1`. it skips the comma, then looks for the next
        // separator to split over. it finds the ` ` space between the comma
        // and the `-1` and this is also a separator. so the space between
        // the comma and the space becomes a item yielded by `::split`.
        // this is a empty string ''.
        .filter(|s| !s.is_empty())
        .map(|line| line.trim().parse::<isize>().unwrap())
        .sum()
}
