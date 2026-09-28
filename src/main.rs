mod towers_of_hanoi;
mod recursive;
mod iterative1;
mod iterative2;

use towers_of_hanoi::{TowersOfHanoi};

fn main() {
    let mut towers = TowersOfHanoi::new(3);
    let mut solver = iterative2::Solver::new(towers);
    while solver.next() != None {
        solver.print();
    }
}
