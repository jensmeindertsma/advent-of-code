pub fn part_1(input: &str) -> usize {
    let required_presents: usize = input.trim().parse().unwrap();

    for house_number in 1..(required_presents / 10) {
        let visiting_elves = divisors(house_number).into_iter();

        let presents_delivered: usize = visiting_elves.sum::<usize>() * 10;

        if presents_delivered >= required_presents {
            return house_number;
        }
    }

    unreachable!()
}

pub fn part_2(input: &str) -> usize {
    let required_presents: usize = input.trim().parse().unwrap();

    for house_number in 1..(required_presents / 10) {
        let visiting_elves = divisors(house_number)
            .into_iter()
            .filter(|elf_n| house_number / elf_n <= 50);

        let presents_delivered: usize = visiting_elves.sum::<usize>() * 11;

        if presents_delivered >= required_presents {
            return house_number;
        }
    }

    unreachable!()
}

fn divisors(n: usize) -> Vec<usize> {
    let mut divisors: Vec<usize> = Vec::from_iter(1..((n as f64).sqrt() as usize + 1))
        .into_iter()
        .filter(|i| n % *i == 0)
        .collect();

    let mut divisors_large: Vec<usize> = divisors
        .iter()
        .filter(|d| n != **d * **d)
        .map(|d| n / d)
        .collect();

    divisors.append(&mut divisors_large);
    divisors
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1(INPUT), 776160);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(INPUT), 786240);
    }
}
