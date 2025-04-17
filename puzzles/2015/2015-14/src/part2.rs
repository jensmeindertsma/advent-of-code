use std::collections::HashMap;

use crate::reindeer::{Reindeer, State};

pub fn part_2(input: &str, seconds: u16) -> u16 {
    let mut reindeer_list: Vec<Reindeer> = input
        .trim()
        .lines()
        .map(|line| Reindeer::parse(line.trim()).unwrap())
        .collect();

    let mut distances = HashMap::new();
    let mut scoreboard = HashMap::new();

    for _ in 0..seconds {
        for reindeer in &mut reindeer_list {
            match &mut reindeer.state {
                State::Flying { remaining_time } => {
                    *distances.entry(reindeer.name).or_insert(0) += reindeer.speed as u16;
                    *remaining_time -= 1;

                    if *remaining_time == 0 {
                        reindeer.state = State::Resting {
                            remaining_time: reindeer.rest_time,
                        }
                    }
                }
                State::Resting { remaining_time } => {
                    *remaining_time -= 1;

                    if *remaining_time == 0 {
                        reindeer.state = State::Flying {
                            remaining_time: reindeer.flight_time,
                        }
                    }
                }
            }
        }

        let leader_distance = *distances.values().max().unwrap();

        distances
            .iter_mut()
            .filter(|(_, distance)| **distance == leader_distance)
            .map(|(name, _)| *name)
            .for_each(|name| *scoreboard.entry(name).or_insert(0) += 1);
    }

    scoreboard.into_values().max().unwrap()
}
