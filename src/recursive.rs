use crate::towers_of_hanoi::{TowersOfHanoi, Tower, Transfer};

pub fn solve(towers: &TowersOfHanoi) -> Vec<Transfer> {
    //God which do I do??? I can either create a new vector with every recurse or mutate one vector.
    //one is ugly and one is super slow. ;-;
    fn recursive_mut(pos: u32, transfers: &mut Vec<Transfer>, origin: Tower, destination: Tower, assist: Tower) {
        if pos != 0 {
            recursive_mut(pos-1, transfers, origin, assist, destination);
            transfers.push(Transfer{origin, destination});
            recursive_mut(pos-1, transfers, assist, destination, origin);
        }
    }
    let mut transfers: Vec<Transfer> = Vec::new();
    let height = towers.ring_count();
    recursive_mut(height, &mut transfers, Tower::A, Tower::C, Tower::B);
    transfers
}
