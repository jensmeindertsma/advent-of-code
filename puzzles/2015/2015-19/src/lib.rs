use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let (replacements, molecule) = parse_input(input);

    let mut created_molecules: HashSet<String> = HashSet::new();

    for (old, new) in replacements {
        for (index, part) in molecule.match_indices(&old) {
            let mut new_molecule = String::new();

            new_molecule.push_str(&molecule[0..index]);
            new_molecule.push_str(&new);
            new_molecule.push_str(&molecule[index + part.len()..molecule.len()]);

            created_molecules.insert(new_molecule);
        }
    }

    created_molecules.len()
}

fn parse_input(input: &str) -> (Vec<(String, String)>, String) {
    input
        .split_once("\n\n")
        .map(|(replacements, molecule)| {
            (
                replacements
                    .trim()
                    .lines()
                    .map(|l| parse_replacement(l).unwrap())
                    .collect(),
                molecule.trim().to_owned(),
            )
        })
        .unwrap()
}

fn parse_replacement(string: &str) -> Result<(String, String), ()> {
    string
        .trim()
        .split_once(" => ")
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .ok_or(())
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
                H => HO
                H => OH
                O => HH

                HOH
                "
            ),
            4
        );

        assert_eq!(
            part_1(
                "
                H => HO
                H => OH
                O => HH

                HOHOHO
                "
            ),
            7
        );

        assert_eq!(part_1(INPUT), 576);
    }

    // #[test]
    // fn part_2() {
    //     use crate::part_2;

    //     assert_eq!(
    //         part_2(
    //             "
    //             e => H
    //             e => O
    //             H => HO
    //             H => OH
    //             O => HH

    //             HOH
    //             "
    //         ),
    //         3
    //     );

    //     assert_eq!(
    //         part_2(
    //             "
    //             e => H
    //             e => O
    //             H => HO
    //             H => OH
    //             O => HH

    //             HOHOHO
    //             "
    //         ),
    //         6
    //     );

    //     assert_eq!(part_2(INPUT), ???);
    // }
}
