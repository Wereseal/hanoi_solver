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
    count: u32,
    towers: [Vec<u32>; 3],
}
impl TowersOfHanoi {
    pub fn new(count: u32) -> Self {
        let first: Vec<u32> = (1..=count).rev().collect();
        TowersOfHanoi {
            count,
            towers: [ first, Vec::new(), Vec::new() ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
