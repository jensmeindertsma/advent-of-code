use crate::parsing::parse;

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let cuboid = parse(line);

            let smallest_perimeter = cuboid
                .faces()
                .map(|(length, width)| (2 * length) + (2 * width))
                .into_iter()
                .min()
                .expect("there should be a smallest perimeter");

            let volume = cuboid.length * cuboid.width * cuboid.height;

            smallest_perimeter + volume
        })
        .sum()
}
