pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let mass: usize = line.parse().unwrap();

            let mut total = calculate(mass);
            let mut current = total;

            loop {
                let extra_fuel = calculate(current);

                if extra_fuel == 0 {
                    break total;
                }

                total += extra_fuel;
                current = extra_fuel
            }
        })
        .sum()
}

fn calculate(amount: usize) -> usize {
    let part = amount as f32 / 3.0;
    (part.floor() as usize).saturating_sub(2)
}
