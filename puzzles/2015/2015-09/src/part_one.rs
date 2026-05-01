use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parsing::description)
        .map(|result| result.expect("each description should be valid"))
        .fold(
            (HashMap::new(), HashSet::new()),
            |(mut paths, mut cities), (from, to, distance)| {
                // TODO:: warn on duplicates? how to handle?

                /*

                2. ⚠️ Conflicting routes (important case)

                these can be ignored because the input would be faulty
                but  we should panic at the point where we'd insert a conflicting route
                just like with a

                London to Dublin = 464
                London to Dublin = 500

                Now you have a real ambiguity:

                same key (London, Dublin)
                different values

                                 */

                paths.insert((from, to), distance);
                paths.insert((to, from), distance);

                cities.insert(from);
                cities.insert(to);

                (paths, cities)
            },
        );

    // TODO: we have a list of all the places with routes.keys()
    // Generate every ordering of those cities

    todo!()
}

mod parsing {
    use nom::{
        Finish, Parser,
        bytes::complete::tag,
        character::complete::{alpha1, digit1},
        combinator::{map, map_res},
    };

    pub fn description(input: &str) -> Option<(&str, &str, u16)> {
        map(
            (destination, tag(" to "), destination, tag(" = "), distance),
            |(from, _, to, _, distance)| (from, to, distance),
        )
        .parse(input)
        .finish()
        .ok()
        .map(|(_, result)| result)
    }

    fn destination(input: &str) -> nom::IResult<&str, &str> {
        alpha1.parse(input)
    }

    fn distance(input: &str) -> nom::IResult<&str, u16> {
        map_res(digit1, str::parse).parse(input)
    }
}
