use crate::api::{Move, MoveType, Side, Square};

use super::{BbPieceState, Bitboard};

mod tests;

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
        *bb = bb.shift_up(1);
        sq.idx = sq.idx + 8;
    });
    // Right
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if sq.file() == 7 {
            *bb = Bitboard::empty();
        } else {
            *bb = *bb << 1;
            sq.idx = sq.idx + 1;
        }
    });
    // Down
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        *bb = bb.shift_down(1);
        sq.idx = sq.idx.saturating_sub(8);
    });
    // Left
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if sq.file() == 0 {
            *bb = Bitboard::empty();
        } else {
            *bb = *bb >> 1;
            sq.idx = sq.idx.saturating_sub(1);
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
        if sq.file() == 7 {
            *bb = Bitboard::empty();
        } else {
            *bb = bb.shift_up(1) << 1;
            sq.idx = sq.idx + 9;
        }
    });
    // Lower-right
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if sq.file() == 7 {
            *bb = Bitboard::empty();
        } else {
            *bb = bb.shift_down(1) << 1;
            sq.idx = sq.idx.saturating_sub(7);
        }
    });
    // Lower-left
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if sq.file() == 0 {
            *bb = Bitboard::empty();
        } else {
            *bb = bb.shift_down(1) >> 1;
            sq.idx = sq.idx.saturating_sub(9);
        }
    });
    // Upper-left
    generate_slides(my_pieces, op_pieces, start_sq, start_bb, moves, |bb, sq| {
        if sq.file() == 0 {
            *bb = Bitboard::empty();
        } else {
            *bb = bb.shift_up(1) >> 1;
            sq.idx = sq.idx + 7;
        }
    });
}

fn generate_rook_moves(ps: &BbPieceState, side: &Side, moves: &mut Vec<(Move, MoveType)>) {
    let (my_pieces, op_pieces, my_rooks) = match side {
        Side::White => (ps.white_pieces(), ps.black_pieces(), ps.wr),
        Side::Black => (ps.black_pieces(), ps.white_pieces(), ps.br),
    };

    generate_moves(
        &my_rooks,
        &my_pieces,
        &op_pieces,
        moves,
        generate_rook_slides,
    );
}

fn generate_bishop_moves(ps: &BbPieceState, side: &Side, moves: &mut Vec<(Move, MoveType)>) {
    let (my_pieces, op_pieces, my_bishops) = match side {
        Side::White => (ps.white_pieces(), ps.black_pieces(), ps.wb),
        Side::Black => (ps.black_pieces(), ps.white_pieces(), ps.bb),
    };

    generate_moves(
        &my_bishops,
        &my_pieces,
        &op_pieces,
        moves,
        generate_bishop_slides,
    );
}

fn generate_queen_moves(ps: &BbPieceState, side: &Side, moves: &mut Vec<(Move, MoveType)>) {
    let (my_pieces, op_pieces, my_queens) = match side {
        Side::White => (ps.white_pieces(), ps.black_pieces(), ps.wq),
        Side::Black => (ps.black_pieces(), ps.white_pieces(), ps.bq),
    };

    generate_moves(
        &my_queens,
        &my_pieces,
        &op_pieces,
        moves,
        generate_bishop_slides,
    );
    generate_moves(
        &my_queens,
        &my_pieces,
        &op_pieces,
        moves,
        generate_rook_slides,
    );
}

fn knight_test_offset_negative(
    my_pieces: &Bitboard,
    op_pieces: &Bitboard,
    start_sq: &Square,
    start_bb: &Bitboard,
    moves: &mut Vec<(Move, MoveType)>,
    offset: u8,
) {
    let tgt_bb = Bitboard::from_u64(start_bb.v >> offset);
    let tgt_sq = Square::from_idx(start_sq.idx - offset);
    if tgt_bb.is_intersecting(op_pieces) {
        moves.push((Move::from_idxs(start_sq.idx, tgt_sq.idx), MoveType::Capture))
    } else if tgt_bb.is_disjoint(my_pieces) {
        moves.push((Move::from_idxs(start_sq.idx, tgt_sq.idx), MoveType::Quiet))
    }
}

fn knight_test_offset_positive(
    my_pieces: &Bitboard,
    op_pieces: &Bitboard,
    start_sq: &Square,
    start_bb: &Bitboard,
    moves: &mut Vec<(Move, MoveType)>,
    offset: u8,
) {
    let tgt_bb = Bitboard::from_u64(start_bb.v << offset);
    let tgt_sq = Square::from_idx(start_sq.idx + offset);
    if tgt_bb.is_intersecting(op_pieces) {
        moves.push((Move::from_idxs(start_sq.idx, tgt_sq.idx), MoveType::Capture))
    } else if tgt_bb.is_disjoint(my_pieces) {
        moves.push((Move::from_idxs(start_sq.idx, tgt_sq.idx), MoveType::Quiet))
    }
}

// TODO better name... i'm too tired right now...
fn generate_knight_moves_(
    my_pieces: &Bitboard,
    op_pieces: &Bitboard,
    start_sq: &Square,
    start_bb: &Bitboard,
    moves: &mut Vec<(Move, MoveType)>,
) {
    // 01234567
    // ........ 7
    // ........ 6
    // ..8.1... 5
    // .7...2.. 4
    // ...n.... 3
    // .6...3.. 2
    // ..5.4... 1
    // ........ 0
    let rank = start_sq.rank();
    let file = start_sq.file();
    if file <= 6 && rank <= 5 {
        //1
        knight_test_offset_positive(my_pieces, op_pieces, start_sq, start_bb, moves, 17);
    }
    if file <= 5 && rank <= 6 {
        // 2
        knight_test_offset_positive(my_pieces, op_pieces, start_sq, start_bb, moves, 10);
    }
    if file <= 5 && rank >= 1 {
        // 01234567
        // ........ 7
        // ........ 6
        // ........ 5
        // ........ 4
        // ........ 3
        // ........ 2
        // .....n.. 1
        // .......3 0
        // 3
        knight_test_offset_negative(my_pieces, op_pieces, start_sq, start_bb, moves, 6);
    }
    if file <= 6 && rank >= 2 {
        // 4
        knight_test_offset_negative(my_pieces, op_pieces, start_sq, start_bb, moves, 15);
    }
    if file >= 1 && rank >= 2 {
        // 5
        knight_test_offset_negative(my_pieces, op_pieces, start_sq, start_bb, moves, 17);
    }
    if file >= 2 && rank >= 1 {
        // 6
        knight_test_offset_negative(my_pieces, op_pieces, start_sq, start_bb, moves, 10);
    }
    if file >= 2 && rank <= 6 {
        // 7
        knight_test_offset_positive(my_pieces, op_pieces, start_sq, start_bb, moves, 6);
    }
    if file >= 1 && rank <= 5 {
        // 8
        knight_test_offset_positive(my_pieces, op_pieces, start_sq, start_bb, moves, 15);
    }
}

fn generate_knight_moves(ps: &BbPieceState, side: &Side, moves: &mut Vec<(Move, MoveType)>) {
    let (my_pieces, op_pieces, my_knights) = match side {
        Side::White => (ps.white_pieces(), ps.black_pieces(), ps.wn),
        Side::Black => (ps.black_pieces(), ps.white_pieces(), ps.bn),
    };

    generate_moves(
        &my_knights,
        &my_pieces,
        &op_pieces,
        moves,
        generate_knight_moves_,
    );
}

// Iterate through all pieces of a particular type and use the
// move_gen function to generate the moves
fn generate_moves(
    my_moving_pieces: &Bitboard,
    my_pieces: &Bitboard,
    op_pieces: &Bitboard,
    moves: &mut Vec<(Move, MoveType)>,
    move_gen: impl Fn(&Bitboard, &Bitboard, &Square, &Bitboard, &mut Vec<(Move, MoveType)>) -> (),
) {
    let mut start_sq: Square = Square::from_idx(0);
    let mut start_bb = Bitboard::from_u64(1);
    let mut _my_moving_pieces = *my_moving_pieces;
    while _my_moving_pieces.is_nonempty() {
        if (_my_moving_pieces.v & 1) != 0 {
            move_gen(&my_pieces, &op_pieces, &start_sq, &start_bb, moves);
        }
        _my_moving_pieces.v = _my_moving_pieces.v >> 1;
        start_bb.v = start_bb.v << 1;
        start_sq.idx = start_sq.idx + 1;
    }
}

fn generate_all_moves(ps: &BbPieceState, side: &Side) -> Vec<(Move, MoveType)> {
    let mut moves = Vec::new();
    generate_knight_moves(&ps, side, &mut moves);
    generate_rook_moves(&ps, side, &mut moves);
    generate_bishop_moves(&ps, side, &mut moves);
    generate_queen_moves(&ps, side, &mut moves);
    moves
}
