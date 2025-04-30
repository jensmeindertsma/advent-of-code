use crate::sue::parse_sue;

pub fn part_2(input: &str) -> u16 {
    let target_akitas = 0;
    let target_cars = 2;
    let target_cats = 7;
    let target_children = 3;
    let target_goldfish = 5;
    let target_perfumes = 1;
    let target_pomeranians = 3;
    let target_samoyeds = 2;
    let target_trees = 3;
    let target_vizslas = 0;

    input
        .trim()
        .lines()
        .map(|line| parse_sue(line.trim()).unwrap())
        .find_map(|sue| {
            if let Some(akitas) = sue.akitas
                && akitas != target_akitas
            {
                return None;
            }

            if let Some(cars) = sue.cars
                && cars != target_cars
            {
                return None;
            }

            if let Some(cats) = sue.cats
                && cats <= target_cats
            {
                return None;
            }

            if let Some(children) = sue.children
                && children != target_children
            {
                return None;
            }

            if let Some(goldfish) = sue.goldfish
                && goldfish >= target_goldfish
            {
                return None;
            }

            if let Some(perfumes) = sue.perfumes
                && perfumes != target_perfumes
            {
                return None;
            }

            if let Some(pomeranians) = sue.pomeranians
                && pomeranians >= target_pomeranians
            {
                return None;
            }

            if let Some(samoyeds) = sue.samoyeds
                && samoyeds != target_samoyeds
            {
                return None;
            }

            if let Some(trees) = sue.trees
                && trees <= target_trees
            {
                return None;
            }

            if let Some(vizslas) = sue.vizslas
                && vizslas != target_vizslas
            {
                return None;
            }

            Some(sue.number)
        })
        .unwrap()
}
