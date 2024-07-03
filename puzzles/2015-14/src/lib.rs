pub fn part_1(input: &str) -> usize {
    let reindeer: Vec<Reindeer> = input
        .trim()
        .lines()
        .map(|line| reindeer(line.trim()).unwrap().1)
        .collect();

    todo!()

    // TODO: parse reindeer
    // start a second loop, where each second we update the state of the reindeer,
    // State::Resting(time_remaining),
    // State::Flying(time_remaining),
    // .fly() will fly for one second and update a scoreboard with distances
}

struct Reindeer {
    speed: usize,
    state: State,
}

enum State {
    Flying { time_remaining: usize },
    Resting { time_remaining: usize },
}

fn reindeer(input: &str) -> nom::IResult<&str, Reindeer> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(
            part_1(
                "
                    Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds. 
                    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
            "
            ),
            1120
        );

        assert_eq!(part_1(INPUT), 2696);
    }
}
