extern crate rand;
use rand::prelude::*;

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

fn get_computer_move(ttt: &TicTacToe) -> Result<(usize, usize), String> {
    let mut rng = rand::thread_rng();

    if ttt.is_game_over() {
        Err("Game is Over!".to_string())
    } else {
        loop {
            let row_index = rng.gen_range(0, 3);
            let col_index = rng.gen_range(0, 3);
            if ttt.get(row_index, col_index).is_none() {
                return Ok((row_index, col_index));
            }
        }
    }
}

fn main() {
    let mut t = TicTacToe::new();
    let mut iter = TTTElementsIterator::new(TTTElements::X);

    while !t.is_game_over() {
        let element = iter.next().unwrap();
        if element == TTTElements::X {
            let m = get_computer_move(&t);
            if let Ok((row_index, col_index)) = m {
                t.set(row_index, col_index, TTTElements::X);
                if let Some(e) = t.is_game_won() {
                    println!("{:?} won the game!", e);
                }
            } else {
                panic!(m.err().unwrap());
            }
        } else {
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
        }
        println!("{:?}", t);
    }
}
