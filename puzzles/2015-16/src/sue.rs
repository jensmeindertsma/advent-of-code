use std::str::FromStr;

#[derive(Debug)]
pub struct Sue {
    pub akitas: Option<u8>,
    pub cars: Option<u8>,
    pub cats: Option<u8>,
    pub children: Option<u8>,
    pub goldfish: Option<u8>,
    pub perfumes: Option<u8>,
    pub pomeranians: Option<u8>,
    pub samoyeds: Option<u8>,
    pub trees: Option<u8>,
    pub vizslas: Option<u8>,
}

impl FromStr for Sue {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        parsing::sue(string)
            .map(|(_, res)| res)
            .map_err(|_| string.to_owned())
    }
}

mod parsing {
    use super::Sue;
    use nom::{
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res, opt, rest},
        error::Error,
        sequence::tuple,
        Err, IResult,
    };

    pub fn sue(input: &str) -> IResult<String, Sue, Error<String>> {
        // First we match the `Sue [n]:` part of the string,
        // we take the rest, which are the properties, we sort
        // them alphabetically, so we can extract them in a known
        // order.

        let rest = tuple((tag("Sue "), digit1, tag(": "), rest))(input)
            .map_err(|s: Err<Error<&str>>| s.map_input(|f| f.to_owned()))?
            .1
            // weird formatting?
             .3;

        let mut properties: Vec<&str> = rest.split(", ").collect();
        properties.sort();

        let properties = properties.join(", ");

        let parse_result = map(
            tuple((
                opt(property("akitas")),
                opt(property("cars")),
                opt(property("cats")),
                opt(property("children")),
                opt(property("goldfish")),
                opt(property("perfumes")),
                opt(property("pomeranians")),
                opt(property("samoyeds")),
                opt(property("trees")),
                opt(property("vizslas")),
            )),
            |(
                akitas,
                cars,
                cats,
                children,
                goldfish,
                perfumes,
                pomeranians,
                samoyeds,
                trees,
                vizslas,
            )| Sue {
                akitas,
                cars,
                cats,
                children,
                goldfish,
                perfumes,
                pomeranians,
                samoyeds,
                trees,
                vizslas,
            },
        )(&properties);

        parse_result
            .map(|(a, b)| (a.to_owned(), b))
            .map_err(|s: Err<Error<&str>>| s.map_input(|f| f.to_owned()))
    }

    fn property<'a>(name: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, u8> {
        move |input: &'a str| {
            map(
                tuple((
                    tag(format!("{name}: ").as_str()),
                    map_res(digit1, |s: &str| s.parse()),
                    opt(tag(", ")),
                )),
                |(_, value, _)| value,
            )(input)
        }
    }
}
