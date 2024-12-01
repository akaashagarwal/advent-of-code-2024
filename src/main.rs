mod puzzle_1;
mod puzzle_2;

use puzzle_1::solve_puzzle_1;
use puzzle_2::solve_puzzle_2;

fn main() {
    println!("Answer to puzzle 1: {}", solve_puzzle_1());
    println!("Answer to puzzle 2: {}", solve_puzzle_2());
}
