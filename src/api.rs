#[derive(Debug, Clone, Copy)]
pub enum Side {
    W,
    B,
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
    KingSide,
}

#[derive(Debug)]
enum MoveType {
    Quiet,
    Capture,
    CastleQueen,
    CastleKing,
    Promote(PromotionType),
    PromoteCapture(PromotionType),
}

#[derive(Debug)]
enum PromotionType {
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

pub trait GameState {
    fn start() -> Self;

    fn empty() -> Self;

    // Print the game state
    fn pretty_print(&self) -> String;

    fn is_legal(&self) -> bool;

    fn make_move(&self, m: Move) -> Self;
}

pub trait PieceState {
    // Check for logic errors in the piece state
    fn is_legal(&self) -> bool;

    // Print the pieces
    fn pretty_print(&self) -> String;

    fn make_move(&self, m: Move) -> Self;

    fn start() -> Self;

    fn empty() -> Self;
}

pub struct Move {
    from: Square,
    to: Square,
}

pub trait Board {
    fn make_move(&self, m: Move) -> Self;

    fn is_legal(&self) -> bool;

    fn test_move(&self, m: Move) -> bool;

    fn pretty_print(&self) -> String;

    fn from_fen(s: String) -> Self;

    fn empty() -> Self;

    // TODO: Start position

    // TODO: validate function
}

#[derive(Clone, Copy)]
pub struct Square {
    pub v: u8,
}

impl Square {
    pub const fn new(v: u8) -> Self {
        Square { v }
    }

    pub const fn from_rank_file(r: Rank, f: File) -> Self {
        Square {
            v: (r as u8) * 8 + (f as u8),
        }
    }
}
