use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    sequence::tuple,
};

pub fn part_1(input: &str, time: usize) -> usize {
    let mut reindeer: Vec<Reindeer> = input
        .trim()
        .lines()
        .map(|line| reindeer(line.trim()).unwrap().1)
        .collect();

    let mut scoreboard = HashMap::new();

    for _ in 0..time {
        for r in &mut reindeer {
            match r.state {
                State::Resting { time_remaining } => {
                    let new_time_remaining = time_remaining - 1;

                    if new_time_remaining == 0 {
                        r.state = State::Flying {
                            time_remaining: r.fly_duration,
                        }
                    } else {
                        r.state = State::Resting {
                            time_remaining: new_time_remaining,
                        }
                    }
                }
                State::Flying { time_remaining } => {
                    *scoreboard.entry(r.name.clone()).or_insert(0) += r.speed;

                    let new_time_remaining = time_remaining - 1;

                    if new_time_remaining == 0 {
                        r.state = State::Resting {
                            time_remaining: r.rest_duration,
                        }
                    } else {
                        r.state = State::Flying {
                            time_remaining: new_time_remaining,
                        }
                    }
                }
            }
        }
    }

    *scoreboard.values().max().unwrap() as usize
}

pub fn part_2(input: &str, time: usize) -> usize {
    let mut reindeer: Vec<Reindeer> = input
        .trim()
        .lines()
        .map(|line| reindeer(line.trim()).unwrap().1)
        .collect();

    let mut distances_traveled = HashMap::new();
    let mut scoreboard = HashMap::new();

    for _ in 0..time {
        for r in &mut reindeer {
            match r.state {
                State::Resting { time_remaining } => {
                    let new_time_remaining = time_remaining - 1;

                    if new_time_remaining == 0 {
                        r.state = State::Flying {
                            time_remaining: r.fly_duration,
                        }
                    } else {
                        r.state = State::Resting {
                            time_remaining: new_time_remaining,
                        }
                    }
                }
                State::Flying { time_remaining } => {
                    *distances_traveled.entry(r.name.clone()).or_insert(0) += r.speed;

                    let new_time_remaining = time_remaining - 1;

                    if new_time_remaining == 0 {
                        r.state = State::Resting {
                            time_remaining: r.rest_duration,
                        }
                    } else {
                        r.state = State::Flying {
                            time_remaining: new_time_remaining,
                        }
                    }
                }
            }
        }

        // Get reindeer name(s) which is currently in the lead, award him a point.
        let leading_distance = { *distances_traveled.values().max().unwrap() };
        for r_name in distances_traveled
            .iter()
            .filter(|(_, distance)| **distance == leading_distance)
            .map(|(n, _)| n.clone())
        {
            *scoreboard.entry(r_name).or_insert(0) += 1;
        }
    }

    *scoreboard.values().max().unwrap() as usize
}

#[derive(Clone)]
struct Reindeer {
    name: String,
    speed: u16,
    fly_duration: u16,
    rest_duration: u16,
    state: State,
}

#[derive(Clone)]
enum State {
    Flying { time_remaining: u16 },
    Resting { time_remaining: u16 },
}

fn reindeer(input: &str) -> nom::IResult<&str, Reindeer> {
    map(
        tuple((
            alpha1,
            tag(" can fly "),
            map_res(digit1, |s: &str| s.parse::<u16>()),
            tag(" km/s for "),
            map_res(digit1, |s: &str| s.parse::<u16>()),
            tag(" seconds, but then must rest for "),
            map_res(digit1, |s: &str| s.parse::<u16>()),
            tag(" seconds."),
        )),
        |(name, _, speed, _, fly_duration, _, rest_duration, _)| Reindeer {
            name: name.to_string(),
            speed,
            fly_duration,
            rest_duration,
            state: State::Flying {
                time_remaining: fly_duration,
            },
        },
    )(input)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    const EXAMPLE_INPUT: &str = "
    Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds. 
    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
";

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1(EXAMPLE_INPUT, 1000,), 1120);

        assert_eq!(part_1(INPUT, 2503), 2696);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(EXAMPLE_INPUT, 1000), 689);

        assert_eq!(part_2(INPUT, 2503), 1084);
    }
}
