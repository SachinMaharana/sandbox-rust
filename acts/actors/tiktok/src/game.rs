use std::ops::Not;

const BOARD_SIZE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    X,
    O,
}

impl Not for Piece {
    type Output = Piece;

    fn not(self) -> Self::Output {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

pub type Tile = Option<Piece>;

pub type Tiles = [[Tile; BOARD_SIZE]; BOARD_SIZE];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub enum Winner {
    X,
    O,
    Tie,
}

pub struct Game {
    tiles: Tiles,
    current_piece: Piece,
    winner: Option<Winner>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            tiles: Default::default(),
            current_piece: Piece::X,
            winner: None,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.winner.is_some()
    }
}
