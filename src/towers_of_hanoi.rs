pub enum Tower {
    A,
    B,
    C,
}
impl Tower {
    fn get_index(&self) -> usize {
        match &self {
            Tower::A => 0,
            Tower::B => 1,
            Tower::C => 2,
        }
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
        assert_eq!(tower_a.get_index(), 0);
        assert_eq!(tower_b.get_index(), 1);
        assert_eq!(tower_c.get_index(), 2);
    }
    #[test]
    fn TowersOfHanoi_constructs_correct() {
        let towers = TowersOfHanoi::new(0);
        assert!(check_ring_count_is_correct(&towers));
        assert!(towers.solved());
    }


}
