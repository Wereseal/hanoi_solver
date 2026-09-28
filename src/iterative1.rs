use crate::towers_of_hanoi::{TowersOfHanoi, Transfer, Tower};

pub struct Solver {
    position: usize,
    transfer_order: Vec<u32>,
    towers: TowersOfHanoi,
}
impl Solver {
    fn populate_transfer_order(count: u32, transfer_order: &mut Vec<u32>) {
        if count > 0 {
            Self::populate_transfer_order(count-1, transfer_order);
            transfer_order.push(count);
            Self::populate_transfer_order(count-1, transfer_order);
        }
    }
    pub fn new(towers: TowersOfHanoi) -> Self {
        let mut transfer_order: Vec<u32> = Vec::new();
        Self::populate_transfer_order(towers.ring_count(), &mut transfer_order);
        Solver {
            position: 0,
            transfer_order,
            towers,
        }
    }
    pub fn next(&mut self) -> Option<Transfer> {
        let disk = *self.transfer_order.get(self.position)?;
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
    
