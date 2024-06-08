use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| line.is_nice(Rules::Part1))
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| line.is_nice(Rules::Part2))
        .count()
}

trait Likeable {
    fn is_nice(&self, rules: Rules) -> bool;
}

impl Likeable for &str {
    fn is_nice(&self, rules: Rules) -> bool {
        match rules {
            Rules::Part1 => {
                let contains_three_vowels = self
                    .chars()
                    .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
                    .count()
                    >= 3;

                let contains_consecutive_character =
                    self.chars().tuple_windows().any(|(a, b)| a == b);

                let contains_illegal_strings =
                    ["ab", "cd", "pq", "xy"].iter().any(|s| self.contains(s));

                contains_three_vowels && contains_consecutive_character && !contains_illegal_strings
            }
            Rules::Part2 => {
                let contains_two_pairs_non_overlapping = self
                    .chars()
                    .tuple_windows()
                    .enumerate()
                    .any(|(index, (a, b))| {
                        let pair = format!("{a}{b}");

                        let index = index + 2;

                        let remaining = if index <= self.len() {
                            &self[index..]
                        } else {
                            ""
                        };

                        remaining.contains(&pair)
                    });

                let contains_spaced_pair = self
                    .chars()
                    .tuple_windows()
                    .any(|(a, b, c)| a != b && a == c);

                contains_two_pairs_non_overlapping && contains_spaced_pair
            }
        }
    }
}

enum Rules {
    Part1,
    Part2,
}

#[cfg(test)]
mod tests {
    use crate::Likeable;
    use crate::Rules;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert!("ugknbfddgicrmopn".is_nice(Rules::Part1));
        assert!("aaa".is_nice(Rules::Part1));
        assert!(!"jchzalrnumimnmhp".is_nice(Rules::Part1));
        assert!(!"haegwjzuvuyypxyu".is_nice(Rules::Part1));
        assert!(!"dvszwmarrgswjxmb".is_nice(Rules::Part1));

        assert_eq!(part_1(INPUT), 258);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert!("qjhvhtzxzqqjkmpb".is_nice(Rules::Part2));
        assert!("xxyxx".is_nice(Rules::Part2));
        assert!(!"uurcxstgmygtbstg".is_nice(Rules::Part2));
        assert!(!"ieodomkazucvgmuy".is_nice(Rules::Part2));

        assert_eq!(part_2(INPUT), 53);
    }
}
