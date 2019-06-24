use dkt::*;
use std::io::{stdin, stdout, Write};

fn get_position() -> (usize, usize) {
    print!("Enter row and column (comma-separated): ");
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    let e: Vec<&str> = s.trim().split(',').collect();
    let row_index: usize = e[0].trim().parse().unwrap();
    let col_index: usize = e[1].trim().parse().unwrap();
    (row_index, col_index)
}

fn main() {
    let mut t = TicTacToe::new();
    let mut iter = TTTElementsIterator::new(TTTElements::X);

    while !t.is_game_over() {
        let element = iter.next().unwrap();
        loop {
            let (row_index, col_index) = get_position();
            match t.set(row_index, col_index, element) {
                Ok(_) => {
                    if let Some(e) = t.is_game_won() {
                        println!("{:?} won the game!", e);
                    }
                    break;
                }
                Err(err) => println!("{}", err),
            }
        }
        println!("{:?}", t);
    }
}
