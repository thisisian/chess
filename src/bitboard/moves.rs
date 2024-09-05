use crate::api::{Move, MoveType, Side, Square};

use super::{BbGameState, BbPieceState, Bitboard};

fn generate_slides(
    my_pieces: &Bitboard,
    op_pieces: &Bitboard,
    start_sq: &Square,
    start_bb: &Bitboard,
    moves: &mut Vec<(Move, MoveType)>,
    f_it: impl Fn(&mut Bitboard, &mut Square) -> (), // Iterator function
) {
    let mut tgt_bb = *start_bb;
    let mut tgt_sq = *start_sq;
    f_it(&mut tgt_bb, &mut tgt_sq);
    loop {
        if tgt_bb.is_empty() {
            break;
        } else if tgt_bb.is_intersecting(my_pieces) {
            break;
        } else if tgt_bb.is_intersecting(op_pieces) {
            moves.push((Move::from_idxs(start_sq.idx, tgt_sq.idx), MoveType::Capture));
            break;
        } else {
            moves.push((Move::from_idxs(start_sq.idx, tgt_sq.idx), MoveType::Quiet));
        }
        f_it(&mut tgt_bb, &mut tgt_sq);
    }
}

fn generate_rook_slides(
    my_pieces: &Bitboard,
    op_pieces: &Bitboard,
    start_sq: &Square,
    start_bb: &Bitboard,
    moves: &mut Vec<(Move, MoveType)>,
) {
    // Up
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        *bb = bb.shift_v(1);
        sq.idx = sq.idx + 8;
    });
    // Down
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        *bb = bb.shift_v(-1);
        sq.idx = sq.idx - 8;
    });
    // Left
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if (sq.is_on_left_border()) {
            *bb = Bitboard::empty();
        } else {
            *bb = *bb << 1;
            sq.idx = sq.idx - 1;
        }
    });
    // Right
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if (sq.is_on_right_border()) {
            *bb = Bitboard::empty();
        } else {
            *bb = *bb >> 1;
            sq.idx = sq.idx + 1;
        }
    });
}

fn generate_bishop_slides(
    my_pieces: &Bitboard,
    op_pieces: &Bitboard,
    start_sq: &Square,
    start_bb: &Bitboard,
    moves: &mut Vec<(Move, MoveType)>,
) {
    // Upper-right
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if sq.is_on_right_border() {
            *bb = Bitboard::empty();
        } else {
            *bb = bb.shift_v(1) >> 1;
            sq.idx = sq.idx + 8 + 1;
        }
    });
    // Lower-right
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if sq.is_on_right_border() {
            *bb = Bitboard::empty();
        } else {
            *bb = bb.shift_v(-1) >> 1;
            sq.idx = sq.idx - 8 + 1;
        }
    });
    // Lower-left
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if sq.is_on_left_border() {
            *bb = Bitboard::empty();
        } else {
            *bb = bb.shift_v(-1) << 1;
            sq.idx = sq.idx - 8 - 1;
        }
    });
    // Upper-left
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if sq.is_on_left_border() {
            *bb = Bitboard::empty();
        } else {
            *bb = bb.shift_v(1) << 1;
            sq.idx = sq.idx + 8 - 1;
        }
    });
}

pub fn generate_rook_moves(ps: &BbPieceState, side: &Side, moves: &mut Vec<(Move, MoveType)>) {
    let (my_pieces, op_pieces, mut my_rooks) = match side {
        Side::White => (ps.white_pieces(), ps.black_pieces(), ps.wr),
        Side::Black => (ps.black_pieces(), ps.white_pieces(), ps.br),
    };

    generate_moves(&my_rooks, &my_pieces, &op_pieces, moves, generate_rook_slides);
}

pub fn generate_moves(
    my_moving_pieces: &Bitboard,
    my_pieces: &Bitboard,
    op_pieces: &Bitboard,
    moves: &mut Vec<(Move, MoveType)>,
    move_gen: impl Fn(&Bitboard, &Bitboard, &Square, &Bitboard, &mut Vec<(Move, MoveType)>) -> (),
) {
    let mut start_sq: Square = Square::from_idx(1);
    let mut start_bb = Bitboard::from_u64(1);
    let mut _my_moving_pieces = *my_moving_pieces;
    while _my_moving_pieces.is_nonempty() {
        if (_my_moving_pieces.v & 1) != 0 {
            move_gen(&my_pieces, &op_pieces, &start_sq, &start_bb, moves);
        }
        _my_moving_pieces.v = _my_moving_pieces.v >> 1;
        start_bb.v = start_bb.v >> 1;
        start_sq.idx = start_sq.idx + 1;
    }
}

pub fn generate_all_moves(bs: &BbGameState) -> Vec<(Move, MoveType)> {
    let mut moves = Vec::new();
    generate_rook_moves(&bs.pieces, &bs.to_move, &mut moves);
    moves
}
