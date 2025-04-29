use crate::common::Present;

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(Present::from_description)
        .map(|present| {
            let areas = present.sides().map(|side| side.area());

            let surface_area: usize = areas.iter().sum();
            let smallest_area = areas.iter().min().unwrap();

            surface_area + smallest_area
        })
        .sum()
}
