use std::collections::HashMap;

pub fn part_one(input: &str, duration: u16) -> u16 {
    let mut racers: Vec<(Reindeer, State)> = input
        .trim()
        .lines()
        .map(parsing::reindeer)
        .map(|result| result.expect("each line should be a valid description"))
        .map(|reindeer| {
            let remaining_duration = reindeer.flight_duration;
            (reindeer, State::Flying { remaining_duration })
        })
        .collect();

    let mut scores: HashMap<&str, u16> = HashMap::new();

    for _ in 0..duration {
        for (reindeer, state) in &mut racers {
            match state {
                State::Resting { remaining_duration } => {
                    *remaining_duration -= 1;

                    if *remaining_duration == 0 {
                        *state = State::Flying {
                            remaining_duration: reindeer.flight_duration,
                        }
                    }
                }
                State::Flying { remaining_duration } => {
                    *scores.entry(reindeer.name).or_insert(0) += reindeer.flight_speed;

                    *remaining_duration -= 1;

                    if *remaining_duration == 0 {
                        *state = State::Resting {
                            remaining_duration: reindeer.rest_duration,
                        }
                    }
                }
            }
        }
    }

    *scores
        .values()
        .max()
        .expect("there should be a maximum distance")
}

enum State {
    Flying { remaining_duration: u16 },
    Resting { remaining_duration: u16 },
}

struct Reindeer<'a> {
    name: &'a str,
    flight_speed: u16,
    flight_duration: u16,
    rest_duration: u16,
}

mod parsing {
    use nom::{
        IResult, Parser,
        bytes::complete::tag,
        character::complete::{alpha1, digit1},
        combinator::{map, map_res},
    };

    use super::Reindeer;

    pub fn reindeer<'a>(input: &'a str) -> Option<Reindeer<'a>> {
        map(
            (
                name,
                tag(" can fly "),
                map_res(digit1, |s: &str| s.parse()),
                tag(" km/s for "),
                map_res(digit1, |s: &str| s.parse()),
                tag(" seconds, but then must rest for "),
                map_res(digit1, |s: &str| s.parse()),
                tag(" seconds."),
            ),
            |(name, _, flight_speed, _, flight_duration, _, rest_duration, _)| Reindeer {
                name,
                flight_speed,
                flight_duration,
                rest_duration,
            },
        )
        .parse(input)
        .map(|(_, reindeer)| reindeer)
        .ok()
    }

    fn name(input: &str) -> IResult<&str, &str> {
        alpha1.parse(input)
    }
}
