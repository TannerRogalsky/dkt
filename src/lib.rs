use std::fmt::{Error, Formatter};

/// u1 (bit) => 0-1 => bool
/// u8 (byte) => 0-255 => char
/// u16 => 0-65k => short
/// u32 => 0-4b => int
/// u64 => 0-inf => long
///
/// i8 => -127->128 => signed char

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum TTTElements {
    X,
    O,
}

pub struct TTTElementsIterator {
    current: TTTElements,
}

impl TTTElementsIterator {
    pub fn new(start: TTTElements) -> Self {
        TTTElementsIterator { current: start }
    }
}

impl Iterator for TTTElementsIterator {
    type Item = TTTElements;

    fn next(&mut self) -> Option<Self::Item> {
        let previous = self.current;
        self.current = match self.current {
            TTTElements::X => TTTElements::O,
            TTTElements::O => TTTElements::X,
        };
        Some(previous)
    }
}

/// 0,0 | 0,1 | 0,2
/// ---------------
/// 1,0 | 1,1 | 1,2
/// ---------------
/// 2,0 | 2,1 | 2,2

const WIDTH: usize = 3;
const HEIGHT: usize = 3;

#[derive(Clone, Copy)]
pub struct TicTacToe {
    elements: [Option<TTTElements>; 9],
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            elements: [None; WIDTH * HEIGHT],
        }
    }

    pub fn set(
        &mut self,
        row_index: usize,
        col_index: usize,
        element: TTTElements,
    ) -> Result<(), String> {
        let index = TicTacToe::twodtooned(row_index, col_index);
        match self.get(row_index, col_index) {
            None => {
                self.elements[index] = Some(element);
                Ok(())
            }
            Some(_) => Err("Already set".to_string()),
        }
    }

    pub fn get(&self, row_index: usize, col_index: usize) -> Option<TTTElements> {
        self.elements[TicTacToe::twodtooned(row_index, col_index)]
    }

    pub fn is_game_over(&self) -> bool {
        !self.elements.iter().any(|&e| e == None) || self.is_game_won().is_some()
    }

    pub fn is_game_won(&self) -> Option<TTTElements> {
        // check rows
        for x in 0..WIDTH {
            if (self.get(x, 0).is_some())
                && (self.get(x, 0) == self.get(x, 1))
                && (self.get(x, 1) == self.get(x, 2))
            {
                return self.get(x, 0);
            }
        }

        // check cols
        for y in 0..HEIGHT {
            if (self.get(0, y).is_some())
                && (self.get(0, y) == self.get(1, y))
                && (self.get(1, y) == self.get(2, y))
            {
                return self.get(0, y);
            }
        }

        // check diagonal
        if (self.get(0, 0).is_some())
            && (self.get(0, 0) == self.get(1, 1))
            && (self.get(1, 1) == self.get(2, 2))
        {
            return self.get(1, 1);
        }
        if (self.get(0, 2).is_some())
            && (self.get(0, 2) == self.get(1, 1))
            && (self.get(1, 1) == self.get(2, 0))
        {
            return self.get(1, 1);
        }

        None
    }

    fn twodtooned(row_index: usize, col_index: usize) -> usize {
        row_index * WIDTH + col_index
    }
}

impl std::fmt::Debug for TicTacToe {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "DKT TicTacToe!!");
        for x in 0..3 {
            for y in 0..3 {
                let index = TicTacToe::twodtooned(x, y);
                let element = self.elements[index];
                match element {
                    None => write!(f, "   "),
                    Some(e) => write!(f, " {:?} ", e),
                };
                if y <= 1 {
                    write!(f, "|");
                }
            }
            if x <= 1 {
                writeln!(f, "\n---+---+---");
            }
        }
        write!(f, "")
    }
}
