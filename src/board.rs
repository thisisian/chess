use crate::consts::{COORDS, TOP_LEFT};
use crate::enums::{Side, SideSet};
use crate::{bitboard::*, enums};

pub trait Board {
    type Move;

    fn test_move(&self, m: Self::Move) -> bool;

    fn make_move(&self, m: Self::Move) -> Self;

    fn pretty_print(&self) -> String;

    // TODO: Empty

    // TODO: Start position

    // TODO: validate function
}

#[derive(Debug, Default, PartialEq)]
struct BbPieceState {
    wp: Bitboard,
    wb: Bitboard,
    wn: Bitboard,
    wr: Bitboard,
    wq: Bitboard,
    wk: Bitboard,
    bp: Bitboard,
    bb: Bitboard,
    bn: Bitboard,
    br: Bitboard,
    bq: Bitboard,
    bk: Bitboard,
}

struct BoardState {
    pieces: BbPieceState,
    en_passant: enums::File,
    reversable_moves: u8,
    b_castling_kingside: bool,
    b_castling_queenside: bool,
    w_castling_kingside: bool,
    w_castling_queenside: bool,
}

pub trait PieceState {
    // Check for logic errors in the piece state
    fn check_valid(&self) -> Result<(), PieceStateValidatorErr>;

    fn pretty_print(&self) -> String;

    fn start() -> Self;
}

#[derive(PartialEq, Debug)]
pub enum PieceStateValidatorErr {
    MissingKing,
    PawnsOnOppositeRank,
    ExtraPawns,
    IllegalPosition,
    PiecesOnSamePosition,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Square {
    val: u8,
}

impl Square {
    pub const fn new(v: u8) -> Self {
        Square { val: v }
    }
}

#[derive(Debug, Default)]
struct BbState {
    piece_state: BbPieceState,
    moved_kings_and_rooks: Vec<Square>,
    castle_status: SideSet,
    active: Side,
    // TODO: Implmeent 50 move draw count
    //fifty_clock: u8,
    // TODO: Implement 3 move repetition
    // How to implement 3 move repetition move?
    // have to carry hash set of previous Boards
    // along with # of times seen
    // Can potentiaally clear the list after pawn
    // move
}

impl PieceState for BbPieceState {
    fn check_valid(&self) -> Result<(), PieceStateValidatorErr> {
        fn check_wrong_number_of_kings(ps: &BbPieceState) -> bool {
            let kings = ps.bk | ps.wk;
            kings.count_bits() != 2
        }

        #[inline]
        fn no_pieces_on_same_square(ps: &BbPieceState) -> bool {
            fn check(bb: &mut Bitboard, rhs: Bitboard) -> bool {
                println!("{}\n", bb.pretty_string());
                if *bb & rhs != Bitboard::default() {
                    return false;
                }
                *bb |= rhs;
                true
            }

            let mut s = ps.wp;
            check(&mut s, ps.wb)
                && check(&mut s, ps.wq)
                && check(&mut s, ps.wk)
                && check(&mut s, ps.bp)
                && check(&mut s, ps.bb)
                && check(&mut s, ps.bn)
                && check(&mut s, ps.br)
                && check(&mut s, ps.bq)
                && check(&mut s, ps.bk)
        }

        if check_wrong_number_of_kings(self) {
            return Err(PieceStateValidatorErr::MissingKing);
        }
        if !no_pieces_on_same_square(self) {
            return Err(PieceStateValidatorErr::PiecesOnSamePosition);
        }

        todo!()
    }

    fn pretty_print(&self) -> String {
        let mut s = String::new();
        for square in TOP_LEFT {
            let coord = COORDS[square.val as usize];
            if (self.wp & coord).val > 0 {
                s.push('P')
            } else if (self.wb & coord).val > 0 {
                s.push('B')
            } else if (self.wn & coord).val > 0 {
                s.push('N')
            } else if (self.wr & coord).val > 0 {
                s.push('R')
            } else if (self.wq & coord).val > 0 {
                s.push('Q')
            } else if (self.wk & coord).val > 0 {
                s.push('K')
            } else if (self.bp & coord).val > 0 {
                s.push('p')
            } else if (self.bb & coord).val > 0 {
                s.push('b')
            } else if (self.bn & coord).val > 0 {
                s.push('n')
            } else if (self.br & coord).val > 0 {
                s.push('r')
            } else if (self.bq & coord).val > 0 {
                s.push('q')
            } else if (self.bk & coord).val > 0 {
                s.push('k')
            } else {
                s.push('.')
            }
            if (square.val + 1) % 8 == 0 {
                s.push('\n')
            }
        }
        s
    }

    fn start() -> Self {
        BbPieceState {
            wp: Bitboard::new(0xff00),
            wb: Bitboard::new(0x24),
            wn: Bitboard::new(0x42),
            wr: Bitboard::new(0x81),
            wq: Bitboard::new(0x8),
            wk: Bitboard::new(0x10),
            bp: Bitboard::new(0xff000000000000),
            bb: Bitboard::new(0x2400000000000000),
            bn: Bitboard::new(0x4200000000000000),
            br: Bitboard::new(0x8100000000000000),
            bq: Bitboard::new(0x0800000000000000),
            bk: Bitboard::new(0x1000000000000000),
        }
    }
}

impl Board for BbState {
    type Move = (Square, Square);

    fn test_move(&self, _: Self::Move) -> bool {
        todo!()
    }

    fn make_move(&self, _: Self::Move) -> Self {
        todo!()
    }

    fn pretty_print(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bbps_sould_fail_on_empty() {
        let ps = BbPieceState::default();
        assert!(ps
            .check_valid()
            .contains_err(&PieceStateValidatorErr::MissingKing))
    }
    #[test]
    fn bbps_sould_fail_duplicate() {
        let ps = BbPieceState {
            wp: Bitboard::new(0),
            wb: Bitboard::new(0),
            wn: Bitboard::new(2),
            wr: Bitboard::new(0),
            wq: Bitboard::new(0),
            wk: Bitboard::new(2),
            bp: Bitboard::new(0),
            bb: Bitboard::new(0),
            bn: Bitboard::new(2),
            br: Bitboard::new(0),
            bq: Bitboard::new(0),
            bk: Bitboard::new(32),
        };
        println!("{:?}", ps.check_valid());
        assert!(ps
            .check_valid()
            .contains_err(&PieceStateValidatorErr::PiecesOnSamePosition))
    }

    #[test]
    fn bbps_pretty_print_start() {
        let start = BbPieceState::start();
        let expected = "rnbqkbnr\n\
                             pppppppp\n\
                             ........\n\
                             ........\n\
                             ........\n\
                             ........\n\
                             PPPPPPPP\n\
                             RNBQKBNR\n";
        println!("{}",start.pretty_print());
        assert!(start.pretty_print() == expected)
    }
}
