use nom::{
    Parser,
    bytes::complete::{tag, take_until},
    combinator::map_res,
    multi::separated_list0,
    sequence::terminated,
};

pub struct Sue {
    pub number: u16,
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

type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

pub fn parse_sue(input: &str) -> Result<Sue, ParseError> {
    let mut sue = Sue {
        number: 0,
        akitas: None,
        cars: None,
        cats: None,
        children: None,
        goldfish: None,
        perfumes: None,
        pomeranians: None,
        samoyeds: None,
        trees: None,
        vizslas: None,
    };

    fn property(input: &str) -> nom::IResult<&str, (&str, u8)> {
        (
            take_until(":"),
            tag(": "),
            map_res(take_until(","), |value: &str| value.parse()),
        )
            .map(|(name, _, value)| (name, value))
            .parse(input)
    }

    let (_, (number, properties)) = (
        tag("Sue "),
        terminated(map_res(take_until(":"), |str: &str| str.parse()), tag(": ")),
        separated_list0(tag(", "), property),
    )
        .map(|(_, number, properties)| (number, properties))
        .parse(input)?;

    sue.number = number;

    for (name, value) in properties {
        match name {
            "akitas" => sue.akitas = Some(value),
            "cars" => sue.cars = Some(value),
            "cats" => sue.cats = Some(value),
            "children" => sue.children = Some(value),
            "goldfish" => sue.goldfish = Some(value),
            "perfumes" => sue.perfumes = Some(value),
            "pomeranians" => sue.pomeranians = Some(value),
            "samoyeds" => sue.samoyeds = Some(value),
            "trees" => sue.trees = Some(value),
            "vizslas" => sue.vizslas = Some(value),
            other => panic!("unknown propery `{other}`"),
        }
    }

    Ok(sue)
}
