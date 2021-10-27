#[derive(Debug)]
pub struct Pawn {
    color: PawnColor,
    position: u8,
    state: PawnState,
}

#[derive(Debug)]

pub enum PawnColor {
    Green,
    Blue,
    White,
    Black,
    Orange,
    Red,
}

#[derive(Debug)]

pub enum PawnState {
    GoodColorBadPosition,
    GoodColorGoodPosition,
    NothingGood,
}

impl Pawn {
    pub fn new(color: PawnColor, position: u8) -> Pawn {
        Pawn {
            color,
            position,
            state: PawnState::GoodColorGoodPosition,
        }
    }
}
