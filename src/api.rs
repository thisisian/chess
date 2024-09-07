use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
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

pub type File = u8;

#[derive(Debug)]
pub enum PieceColor {
    WhitePawn = 0,
    WhiteRook = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackRook = 7,
    BlackKnight = 8,
    BlackBishop = 9,
    BlackQueen = 10,
    BlackKing = 11,
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
    KingSide,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum MoveType {
    Quiet,
    Capture,
    CastleQueen,
    CastleKing,
    Promote(PromotionType),
    PromoteCapture(PromotionType),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum PromotionType {
    Queen,
    Rook,
    Bishop,
    Knight,
}

// Direction and a distance
type Compass = (Square, Direction, u8);

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    Equal,
}

//
pub trait GameState {
    fn start() -> Self;

    // Print the game state
    fn pretty_string(&self) -> String;

    // Check for logic errors in the piece state
    fn is_legal(&self) -> bool;

    fn make_move(&self, m: Move) -> Option<Self>
    where
        Self: Sized;

    fn to_fen(&self) -> String;

    fn from_fen(s: String) -> Option<Self>
    where
        Self: Sized;
}

pub trait PieceState {
    fn is_legal(&self) -> bool;

    fn start() -> Self;

    fn empty() -> Self;

    // Print the pieces
    fn pretty_string(&self) -> String;

    fn from_pretty_string(s: &str) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Hash, PartialEq, Eq)]
pub struct Move {
    from: Square,
    to: Square,
}

impl Move {
    pub const fn from_idxs(from: u8, to: u8) -> Self {
        Move {
            from: Square::from_idx(from),
            to: Square::from_idx(to),
        }
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Move [{} -> {}]", self.from.idx, self.to.idx)
    }
}

pub trait Board {
    fn make_move(&self, m: Move) -> Self;

    fn is_legal(&self) -> bool;

    fn test_move(&self, m: Move) -> bool;

    fn pretty_string(&self) -> String;

    fn from_fen(s: String) -> Self;

    fn empty() -> Self;

    // TODO: Start position

    // TODO: validate function
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Square {
    pub idx: u8,
}

impl Square {
    pub const fn from_idx(idx: u8) -> Self {
        Square { idx }
    }

    pub const fn from_rank_file(r: Rank, f: File) -> Self {
        Square {
            idx: (r as u8) * 8 + (f as u8),
        }
    }

    pub const fn rank(&self) -> u8 {
        self.idx / 8
    }

    pub const fn file(&self) -> u8 {
        self.idx % 8
    }
}
