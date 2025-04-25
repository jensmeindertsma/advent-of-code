use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while1},
    sequence::terminated,
};

#[derive(Debug)]
pub struct Reindeer<'a> {
    pub name: &'a str,
    pub speed: u8,
    pub flight_time: u8,
    pub rest_time: u8,
    pub state: State,
}

type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

impl<'a> Reindeer<'a> {
    pub fn parse(input: &'a str) -> Result<Reindeer<'a>, ParseError<'a>> {
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
}

#[derive(Debug)]
pub enum State {
    Flying { remaining_time: u8 },
    Resting { remaining_time: u8 },
}
