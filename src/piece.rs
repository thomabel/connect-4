#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Draw = -1, 
    Empty, P1, P2
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self {
            Piece::Draw => "Draw".to_string(),
            Piece::Empty => "Empty".to_string(),
            Piece::P1 => "Player 1".to_string(),
            Piece::P2 => "Player 2".to_string(),
        }
    }
}