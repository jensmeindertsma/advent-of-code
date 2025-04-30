use crate::sue::Sue;

pub struct Evaluator {
    pub akitas: fn(u8, u8) -> bool,
    pub cars: fn(u8, u8) -> bool,
    pub cats: fn(u8, u8) -> bool,
    pub children: fn(u8, u8) -> bool,
    pub goldfish: fn(u8, u8) -> bool,
    pub perfumes: fn(u8, u8) -> bool,
    pub pomeranians: fn(u8, u8) -> bool,
    pub samoyeds: fn(u8, u8) -> bool,
    pub trees: fn(u8, u8) -> bool,
    pub vizslas: fn(u8, u8) -> bool,
}

impl Evaluator {
    pub fn evaluate(&self, sue: &Sue, target: &Target) -> bool {
        if let Some(value) = sue.akitas
            && !(self.akitas)(value, target.akitas)
        {
            return false;
        };

        if let Some(value) = sue.cars
            && !(self.cars)(value, target.cars)
        {
            return false;
        };

        if let Some(value) = sue.cats
            && !(self.cats)(value, target.cats)
        {
            return false;
        };

        if let Some(value) = sue.children
            && !(self.children)(value, target.children)
        {
            return false;
        };

        if let Some(value) = sue.goldfish
            && !(self.goldfish)(value, target.goldfish)
        {
            return false;
        };

        if let Some(value) = sue.perfumes
            && !(self.perfumes)(value, target.perfumes)
        {
            return false;
        };

        if let Some(value) = sue.pomeranians
            && !(self.pomeranians)(value, target.pomeranians)
        {
            return false;
        };

        if let Some(value) = sue.samoyeds
            && !(self.samoyeds)(value, target.samoyeds)
        {
            return false;
        };

        if let Some(value) = sue.trees
            && !(self.trees)(value, target.trees)
        {
            return false;
        };

        if let Some(value) = sue.vizslas
            && !(self.vizslas)(value, target.vizslas)
        {
            return false;
        };

        true
    }
}

pub struct Target {
    pub akitas: u8,
    pub cars: u8,
    pub cats: u8,
    pub children: u8,
    pub goldfish: u8,
    pub perfumes: u8,
    pub pomeranians: u8,
    pub samoyeds: u8,
    pub trees: u8,
    pub vizslas: u8,
}
