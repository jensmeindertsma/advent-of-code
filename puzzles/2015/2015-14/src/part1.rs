use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_while1},
    sequence::terminated,
    IResult, Parser,
};

pub fn part_1(input: &str, seconds: u16) -> u16 {
    let mut reindeer_list: Vec<Reindeer> = input
        .trim()
        .lines()
        .map(|line| parse_reindeer(line.trim()).unwrap())
        .collect();

    let mut scoreboard = HashMap::new();

    for _ in 0..seconds {
        for reindeer in &mut reindeer_list {
            reindeer.travel(&mut scoreboard)
        }
    }

    dbg!(&scoreboard);

    scoreboard.into_values().max().unwrap()
}

#[derive(Debug)]
struct Reindeer<'a> {
    name: &'a str,
    speed: u8,
    flight_time: u8,
    rest_time: u8,
    state: State,
}

impl<'a> Reindeer<'a> {
    pub fn travel(&mut self, scoreboard: &mut HashMap<&'a str, u16>) {
        match &mut self.state {
            State::Flying { remaining_time } => {
                *scoreboard.entry(self.name).or_insert(0) += self.speed as u16;
                *remaining_time -= 1;

                if *remaining_time == 0 {
                    self.state = State::Resting {
                        remaining_time: self.rest_time,
                    }
                }
            }
            State::Resting { remaining_time } => {
                *remaining_time -= 1;

                if *remaining_time == 0 {
                    self.state = State::Flying {
                        remaining_time: self.flight_time,
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum State {
    Flying { remaining_time: u8 },
    Resting { remaining_time: u8 },
}

type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

fn parse_reindeer(input: &str) -> Result<Reindeer, ParseError> {
    fn string(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c.is_ascii_alphabetic()).parse(input)
    }

    fn number(input: &str) -> IResult<&str, u8> {
        take_while1(|c: char| c.is_ascii_digit())
            .map_res(|speed: &str| speed.parse())
            .parse(input)
    }

    let (_, reindeer) = (
        terminated(string, tag(" can fly ")),
        terminated(number, tag(" km/s for ")),
        terminated(number, tag(" seconds, but then must rest for ")),
        terminated(number, tag(" seconds.")),
    )
        .map(|(name, speed, flight_time, rest_time)| Reindeer {
            name,
            speed,
            flight_time,
            rest_time,
            state: State::Flying {
                remaining_time: flight_time,
            },
        })
        .parse(input)?;

    Ok(reindeer)
}
