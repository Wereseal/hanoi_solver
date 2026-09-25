use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Tower {
    A,
    B,
    C,
}
impl Tower {
    fn index(&self) -> usize {
        match &self {
            Tower::A => 0,
            Tower::B => 1,
            Tower::C => 2,
        }
    }
}
impl fmt::Display for Tower{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tower::A => write!(f, "A")?,
            Tower::B => write!(f, "B")?,
            Tower::C => write!(f, "C")?,
        }
        Ok(())
    }
}
#[derive(Clone, Copy)]
pub enum TransferError {
    SourceIsDestination,
    EmptySource,
    DestinationTooSmall,
}
// I don't like using step instead of move but move is a rust keyword. :(
#[derive(Clone, Copy)]
pub struct Transfer {
    pub origin: Tower,
    pub destination: Tower,
}
impl fmt::Display for Transfer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.origin, self.destination)?;
        Ok(())
    }
}
pub struct TowersOfHanoi {
    ring_count: u32,
    towers: [Vec<u32>; 3],
}
impl TowersOfHanoi {
    pub fn new(count: u32) -> Self {
        let first: Vec<u32> = (1..=count).rev().collect();
        TowersOfHanoi {
            ring_count: count,
            towers: [ first, Vec::new(), Vec::new() ],
        }
    }
    pub fn ring_count(&self) -> u32 {
        self.ring_count
    }
    pub fn towers(&self) -> &[Vec<u32>; 3] {
        &self.towers
    }
    pub fn solved(&self) -> bool {
        self.towers.iter().map(|tower| if tower.len() == 0 {0} else {1}).sum::<u8>() <= 1
    }
    pub fn transfer(&mut self, transfer: Transfer) -> Result<(), TransferError> {
        if transfer.origin == transfer.destination {
            return Err(TransferError::SourceIsDestination);
        }

        let origin_val: u32 = *self.towers[transfer.origin.index()].last().ok_or(TransferError::EmptySource)?;

        if let Some(destination_val) = self.towers[transfer.destination.index()].last() && *destination_val < origin_val {
            return Err(TransferError::DestinationTooSmall);
        }
        
        self.towers[transfer.origin.index()].pop();
        self.towers[transfer.destination.index()].push(origin_val);

        return Ok(());

    }
}
impl fmt::Display for TowersOfHanoi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nA: ")?;
        for ring in &self.towers[0] {
            write!(f, "{}, ", ring)?;
        }
        write!(f, "\nB: ")?;
        for ring in &self.towers[1] {
            write!(f, "{}, ", ring)?;
        }
        write!(f, "\nC: ")?;
        for ring in &self.towers[2] {
            write!(f, "{}, ", ring)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_ring_count_is_correct(towers: &TowersOfHanoi) -> bool {
        let real_total: u32 = towers.towers.iter().map(|tower| tower.len() as u32).sum();
        let reported_total: u32 = towers.ring_count();
        real_total == reported_total
    }
    #[test]
    fn tower_get_index_gives_correct_index() {
        let tower_a = Tower::A;
        let tower_b = Tower::B;
        let tower_c = Tower::C;
        assert_eq!(tower_a.index(), 0);
        assert_eq!(tower_b.index(), 1);
        assert_eq!(tower_c.index(), 2);
    }
    #[test]
    fn TowersOfHanoi_constructs_correct() {
        let towers = TowersOfHanoi::new(0);
        assert!(check_ring_count_is_correct(&towers));
        assert!(towers.solved());
    }


}
