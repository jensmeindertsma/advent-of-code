mod common;
mod part1;
mod part2;

use ornament::{Puzzle, Solution};
use part1::part_1;
use part2::part_2;

fn main() {
    Puzzle {
        name: "Not Quite Lisp",
        year: 2015,
        day: 1,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the instructions take Santa to floor {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("Santa first entered the basement at position {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use super::part_1;

        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1("()()"), 0);
        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(()(()("), 3);
        assert_eq!(part_1("))((((("), 3);
        assert_eq!(part_1("())"), -1);
        assert_eq!(part_1("))("), -1);
        assert_eq!(part_1(")))"), -3);
        assert_eq!(part_1(")())())"), -3);

        assert_eq!(part_1(INPUT), 138);
    }

    #[test]
    fn part_2() {
        use super::part_2;

        assert_eq!(part_2(")"), 1);
        assert_eq!(part_2("()())"), 5);
        assert_eq!(part_2(INPUT), 1771);
    }
}
