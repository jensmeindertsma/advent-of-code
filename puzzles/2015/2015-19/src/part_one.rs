use crate::parsing;
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let (descriptions, molecule) = input.split_once("\n\n").unwrap();

    let molecule = molecule.trim();

    let mut replacements = HashMap::new();

    for replacement in descriptions
        .lines()
        .map(parsing::replacement)
        .map(|result| result.unwrap())
    {
        let (from, to) = replacement;

        replacements.entry(from).or_insert(Vec::new()).push(to);
    }

    ////////////////////////////////////////

    let (mut elements, start) = molecule.char_indices().skip(1).fold(
        (Vec::<&str>::new(), 0),
        |(mut elements, mut start), (index, character)| {
            // New token starts at uppercase letter
            if character.is_ascii_uppercase() {
                elements.push(&molecule[start..index]);
                start = index;
            }

            (elements, start)
        },
    );

    elements.push(&molecule[start..]);

    /////////////////////////////////////////

    let mut molecules = HashSet::new();

    for (index, element) in elements.iter().enumerate() {
        if let Some(list) = replacements.get(element) {
            for replacement in list {
                let mut new_elements = elements.clone();

                new_elements[index] = *replacement;

                let new_molecule = new_elements.concat();

                molecules.insert(new_molecule);
            }
        }
    }

    molecules.len()
}
