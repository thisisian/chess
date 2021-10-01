#[derive(Debug, Default)]
pub enum Side {
    #[default]
    W,
    B,
}

pub enum Piece {
    P,
    N,
    B,
    R,
    Q,
    K,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum SideSet {
    #[default]
    None,
    White,
    Black,
    Both,
}
