use crate::towers_of_hanoi::{TowersOfHanoi, Transfer, Tower};

pub struct Solver {
    position: usize,
    towers: TowersOfHanoi,
}
impl Solver {
    fn get_lowest_none_one(&self) -> Option<u32> {
        let mut values: Vec<u32> = self.towers.get_tops().iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<Vec<u32>>();
        values.sort();
        values.get(1).copied()
    }
    pub fn new(towers: TowersOfHanoi) -> Self {
        Solver {
            position: 0,
            towers,
        }
    }
    pub fn next(&mut self) -> Option<Transfer> {
        let disk: u32 = match self.position % 2 {
            0 => 1,
            1 => self.get_lowest_none_one()?,
            _ => panic!("modulo failed?"),
        };
        let tops = self.towers.get_tops();
        let origin: Tower;
        // Ugggghhhh so ugly
        if let Some(x) = tops[0] && x == disk {
            origin = Tower::A;
        } else if let Some(x) = tops[1] && x == disk {
            origin = Tower::B;
        } else if let Some(x) = tops[2] && x == disk {
            origin = Tower::C;
        } else {
            panic!("Failed to find disk");
        }
        let destination = match tops[origin.next().index()] {
            None => origin.next(),
            Some(x) if x > disk => origin.next(),
            _ => origin.next().next(),
        };
        let transfer = Transfer{origin, destination};
        self.position += 1;
        self.towers.transfer(transfer);
        Some(transfer)
    }
    pub fn print(&self) {
        println!("{}", self.towers);
    }
}
    
