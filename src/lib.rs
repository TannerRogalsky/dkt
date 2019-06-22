/// u1 (bit) => 0-1 => bool
/// u8 (byte) => 0-255 => char
/// u16 => 0-65k => short
/// u32 => 0-4b => int
/// u64 => 0-inf => long
///
/// i8 => -127->128 => signed char

#[derive(Debug, Clone, Copy)]
pub enum TTTElements {
    X,
    O,
}

pub struct TTTElementsIterator {
    current: TTTElements,
}

impl TTTElementsIterator {
    pub fn new(start: TTTElements) -> Self {
        TTTElementsIterator {
            current: start,
        }
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

#[derive(Debug, Clone, Copy)]
pub struct TicTacToe {
    elements: [Option<TTTElements>; 9],
}

/// - Handle invalid row & column values
/// - Handle existing value at coordinates
/// - fn is_win?() returns Element that won
/// - unwinnable state is array is full but win is false

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            elements: [None; 9]
        }
    }

    pub fn set(&mut self, row_index: usize, col_index: usize, element: TTTElements) {
        self.elements[TicTacToe::twodtooned(row_index, col_index)] = Some(element)
    }

    pub fn get(&self, row_index: usize, col_index: usize) -> Option<TTTElements> {
        self.elements[TicTacToe::twodtooned(row_index, col_index)]
    }

    fn twodtooned(row_index: usize, col_index: usize) -> usize {
        row_index * 3 + col_index
    }
}