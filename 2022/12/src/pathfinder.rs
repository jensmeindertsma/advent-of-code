use std::collections::{HashMap, HashSet};

use crate::map::{Map, Position};

pub struct Pathfinder<'a> {
    map: &'a Map,
    history: HashMap<Position, PlaceRecord>,
    next: HashSet<Position>,
}

impl<'a> Pathfinder<'a> {
    pub fn run(self) -> Vec<Position> {
        let start = self.map.start();
        let destination = self.map.destination();

        for next in self.next {
            if next == self.map.destination() {}

            for possible in self.map.surrounding(next).filter() {
                if self.history.keys().any(|key| key == possible) {}
            }
        }

        todo!()
    }
}

impl<'a> From<&'a Map> for Pathfinder<'a> {
    fn from(map: &'a Map) -> Self {
        Self {
            map,
            history: HashMap::new(),
            next: HashSet::new(),
        }
    }
}

struct PlaceRecord {
    visited_from: Option<Position>,
}
