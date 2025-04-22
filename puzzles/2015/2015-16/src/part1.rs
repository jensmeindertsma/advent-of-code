use nom::{
    Parser,
    bytes::complete::{tag, take_until},
    combinator::map_res,
    sequence::terminated,
};

pub fn part_1(input: &str) -> u16 {
    let target = Sue {
        number: 0,
        akitas: Some(0),
        cars: Some(2),
        cats: Some(7),
        children: Some(3),
        goldfish: Some(5),
        perfumes: Some(1),
        pomeranians: Some(3),
        samoyeds: Some(2),
        trees: Some(3),
        vizslas: Some(0),
    };

    input
        .trim()
        .lines()
        .map(|line| parse_sue(line.trim()).unwrap())
        .find_map(|sue| {
            if sue.akitas.is_some() && sue.akitas != target.akitas {
                return None;
            }

            if sue.cars.is_some() && sue.cars != target.cars {
                return None;
            }

            if sue.cats.is_some() && sue.cats != target.cats {
                return None;
            }

            if sue.children.is_some() && sue.children != target.children {
                return None;
            }

            if sue.goldfish.is_some() && sue.goldfish != target.goldfish {
                return None;
            }

            if sue.perfumes.is_some() && sue.perfumes != target.perfumes {
                return None;
            }

            if sue.pomeranians.is_some() && sue.pomeranians != target.pomeranians {
                return None;
            }

            if sue.samoyeds.is_some() && sue.samoyeds != target.samoyeds {
                return None;
            }

            if sue.trees.is_some() && sue.trees != target.trees {
                return None;
            }

            if sue.vizslas.is_some() && sue.vizslas != target.vizslas {
                return None;
            }

            return Some(sue.number);
        })
        .unwrap()
}

struct Sue {
    number: u16,
    akitas: Option<u8>,
    cars: Option<u8>,
    cats: Option<u8>,
    children: Option<u8>,
    goldfish: Option<u8>,
    perfumes: Option<u8>,
    pomeranians: Option<u8>,
    samoyeds: Option<u8>,
    trees: Option<u8>,
    vizslas: Option<u8>,
}

type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

fn parse_sue(input: &str) -> Result<Sue, ParseError> {
    (
        tag("Sue "),
        terminated(map_res(take_until(","), |str: &str| str.parse()), tag(",")),
    )
        .map(|(_, number)| Sue { number })
        .parse(input)
        .map(|(_, sue)| sue)
}
