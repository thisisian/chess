use crate::bitboard::*;
use crate::enums::{Side, SideSet};

pub trait Board {
    type Move;

    fn test_move(&self, m: Self::Move) -> bool;

    fn make_move(&self, m: Self::Move) -> Self;

    // TODO: Empty

    // TODO: Start position

    // TODO: validate function
}

pub enum MoveResult {}

pub trait PieceState {
    // Check for logic errors in the piece state
    fn check_valid(&self) -> Result<(), PieceStateValidatorErr>;
}

#[derive(PartialEq)]
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

        fn check_correct_number_of_kings(ps: &BbPieceState) -> Result<(), PieceStateValidatorErr> {
            let kings = ps.bk & ps.wk;
            if kings.count_bits() != 2 {
                return Err(PieceStateValidatorErr::MissingKing);
            } else {
                return Ok(())
            }
        };


        #[inline]
        fn check_pieces_on_same_square(ps: &BbPieceState) -> bool {
            #[inline]
            fn check(bb: &mut Bitboard, rhs: Bitboard) -> bool {
                if *bb & rhs != Bitboard::default() {
                    return false
                }
                *bb |= rhs;
                return true;
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
        };

        check_correct_number_of_kings(self)?;


        todo!()
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
            bn: Bitboard::new(0),
            br: Bitboard::new(0),
            bq: Bitboard::new(0),
            bk: Bitboard::new(32),
        };
        assert!(ps
            .check_valid()
            .contains_err(&PieceStateValidatorErr::PiecesOnSamePosition))
    }
}
