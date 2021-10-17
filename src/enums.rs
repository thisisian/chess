#[derive(Debug, Default)]
pub enum Side {
    #[default]
    W,
    B,
}

pub enum Piece {
    P = 0,
    N = 1,
    B = 2,
    R = 3,
    Q = 4,
    K = 5,
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    R1 = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
    R6 = 5,
    R7 = 6,
    R8 = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum SideSet {
    #[default]
    None,
    White,
    Black,
    Both,
}

pub enum BoardSide {
    QueenSide,
    KingSide
}