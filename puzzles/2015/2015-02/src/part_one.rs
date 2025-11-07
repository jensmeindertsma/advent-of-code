use crate::parsing::parse;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let cuboid = parse(line);

            let total_surface_area = {
                let l = cuboid.length;
                let w = cuboid.width;
                let h = cuboid.height;

                (2 * l * w) + (2 * w * h) + (2 * h * l)
            };

            let smallest_area = cuboid
                .faces()
                .map(|(length, width)| length * width)
                .into_iter()
                .min()
                .expect("one of the sides should be the smallest");

            total_surface_area + smallest_area
        })
        .sum()
}
