use itertools::Itertools;

pub fn part_1(input: &str) -> u32 {
    let mut input = input.trim().to_owned();
    input.push(input.chars().next().unwrap());

    let mut sum: u32 = 0;

    for (a, b) in input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .tuple_windows()
    {
        if a == b {
            sum += a
        }
    }

    sum
}

pub fn part_2(input: &str) -> u32 {
    let input = input.trim();
    let length = input.len();

    let mut sum = 0;

    for (index, x) in input.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
        let y_index = (index + (length / 2)) % length;
        let y = input
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .nth(y_index)
            .expect("no y ");

        if x == y {
            sum += x
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1("1122"), 3);
        assert_eq!(part_1("1111"), 4);
        assert_eq!(part_1("1234"), 0);
        assert_eq!(part_1("91212129"), 9);

        assert_eq!(part_1(INPUT), 1034);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2("1212"), 6);
        assert_eq!(part_2("1221"), 0);
        assert_eq!(part_2("123425"), 4);
        assert_eq!(part_2("123123"), 12);
        assert_eq!(part_2("12131415"), 4);

        assert_eq!(part_2(INPUT), 1356);
    }
}
