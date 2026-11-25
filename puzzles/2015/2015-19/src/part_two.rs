use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_two(input: &str) -> usize {
    let (replacements, molecule) = input.trim().split_once("\n\n").unwrap();

    let replacements = replacements
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .fold(HashMap::new(), |mut map, (atom, molecule)| {
            map.entry(molecule).or_insert(Vec::new()).push(atom);
            map
        });

    if replacements
        .iter()
        .any(|(molecule, _)| molecule.contains("Rn"))
    {
        let mut characters = molecule.char_indices().peekable();

        let mut elements = Vec::new();

        while let Some((start, ch)) = characters.next() {
            // Each element starts with an uppercase letter
            if ch.is_ascii_uppercase() {
                let end = if let Some(&(next_index, next_ch)) = characters.peek() {
                    if next_ch.is_ascii_lowercase() {
                        characters.next();
                        next_index + 1
                    } else {
                        start + 1
                    }
                } else {
                    start + 1
                };

                elements.push(&molecule[start..end]);
            }
        }

        let total = elements.len();

        let structural = elements.iter().filter(|&&t| t == "Rn" || t == "Ar").count();

        let separators = elements.iter().filter(|&&t| t == "Y").count();

        total - structural - 2 * separators - 1
    } else {
        let mut queue = VecDeque::new();
        let mut fabricated = HashSet::new();

        queue.push_back((molecule.to_string(), 0));
        fabricated.insert(molecule.to_string());

        // Loop through all of the new unique molecules we just created
        while let Some((current, steps)) = queue.pop_front() {
            // If we just created the `e` element, we are done
            if current == "e" {
                return steps;
            }

            // Apply each possible replacement once to the current molecule, and
            // if the result is a molecule we haven't fabricated yet, we add
            // it to the set of molecules we will apply another replacement to
            // next iteration of the while loop.
            for (from, replacements) in &replacements {
                for replacement in replacements {
                    // We apply the replacement once for each occurence in the molecule
                    for (index, _) in current.match_indices(from) {
                        let mut next = current.clone();

                        next.replace_range(index..index + from.len(), replacement);

                        if fabricated.insert(next.clone()) {
                            queue.push_back((next, steps + 1));
                        }
                    }
                }
            }
        }

        panic!("No solution found");
    }
}
