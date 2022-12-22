use crate::map::Map;

pub struct Pathfinder<'a> {
    map: &'a Map,
}

impl<'a> Pathfinder<'a> {
    pub fn run(self) -> usize {
        let start = self.map.start();
        let destination = self.map.destination();

        println!("Pathfinding from {start:?} to {destination:?}");
        println!("{:?}", self.map);

        todo!()
    }
}

impl<'a> From<&'a Map> for Pathfinder<'a> {
    fn from(map: &'a Map) -> Self {
        Self { map }
    }
}
