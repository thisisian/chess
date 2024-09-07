use std::collections::HashSet;

use crate::{
    api::{Move, MoveType, PieceState, PromotionType, Side, Square},
    bitboard::{move_gen::generate_all_moves, string_board_iter, BbPieceState},
};

#[rustfmt::skip]
#[test]
fn rook_test1() {
    let board =
    "........\n\
     ........\n\
     ........\n\
     ........\n\
     ........\n\
     ........\n\
     ........\n\
     R.......\n";
    let mv =
    "q.......\n\
     q.......\n\
     q.......\n\
     q.......\n\
     q.......\n\
     q.......\n\
     q.......\n\
     sqqqqqqq\n";
    test_move_gen(board, &[mv])
}

#[rustfmt::skip]
#[test]
fn rook_test2() {
    let board =
    ".......R\n\
     ........\n\
     ........\n\
     ........\n\
     ........\n\
     ........\n\
     ........\n\
     ........\n";
    let mv =
    "qqqqqqqs\n\
     .......q\n\
     .......q\n\
     .......q\n\
     .......q\n\
     .......q\n\
     .......q\n\
     .......q\n";
    test_move_gen(board, &[mv])
}

#[rustfmt::skip]
#[test]
fn rook_test3() {
    let board =
    "........\n\
        ........\n\
        .....R..\n\
        ........\n\
        ........\n\
        ..R.....\n\
        ........\n\
        ........\n";
    let mv1 =
    ".....q..\n\
        .....q..\n\
        qqqqqsqq\n\
        .....q..\n\
        .....q..\n\
        .....q..\n\
        .....q..\n\
        .....q..\n";
    let mv2 =
    "..q.....\n\
        ..q.....\n\
        ..q.....\n\
        ..q.....\n\
        ..q.....\n\
        qqsqqqqq\n\
        ..q.....\n\
        ..q.....\n";
    test_move_gen(board, &[mv1, mv2])
}

#[rustfmt::skip]
#[test]
fn bishop_test1() {
    let board =
       "........\n\
        ........\n\
        .....B..\n\
        ........\n\
        ...B....\n\
        ........\n\
        ........\n\
        ........\n";
    let mv1 =
       "........\n\
        q.......\n\
        .q......\n\
        ..q.q...\n\
        ...s....\n\
        ..q.q...\n\
        .q...q..\n\
        q.....q.\n";
    let mv2 =
       "...q...q\n\
        ....q.q.\n\
        .....s..\n\
        ....q.q.\n\
        .......q\n\
        ........\n\
        ........\n\
        ........\n";
    test_move_gen(board, &[mv1, mv2])
}



#[rustfmt::skip]
#[test]
fn knight_test1() {
    let board =
       "........\n\
        ........\n\
        ........\n\
        ...N....\n\
        ........\n\
        ........\n\
        ........\n\
        ........\n";
    let mv1 =
       "........\n\
        ..q.q...\n\
        .q...q..\n\
        ...s....\n\
        .q...q..\n\
        ..q.q...\n\
        ........\n\
        ........\n";
    test_move_gen(board, &[mv1])
}

#[rustfmt::skip]
#[test]
fn move_test1() {
    let mv1 =
    "........\n\
     ........\n\
     ........\n\
     ........\n\
     ........\n\
     q.......\n\
     q.......\n\
     sqqc....\n";
    let mv2 =
    "........\n\
     ........\n\
     .q......\n\
     ..q.....\n\
     s.......\n\
     ........\n\
     .q......\n\
     ........\n";
    let mv3 =
    "........\n\
     ........\n\
     ....c.q.\n\
     ...q...q\n\
     .....s..\n\
     ...q...q\n\
     ....q.q.\n\
     ........\n";
    let board =
    "........\n\
     ........\n\
     ....n...\n\
     ........\n\
     N....N..\n\
     ..N.....\n\
     ........\n\
     R..n....\n";
    let mv4 =
    "........\n\
     ........\n\
     ........\n\
     .q.q....\n\
     ....q...\n\
     ..s.....\n\
     q...q...\n\
     .q.c....\n";
    test_move_gen(board, &[mv1, mv2, mv3, mv4])
}

#[rustfmt::skip]
#[test]
fn move_test2() {
    let board =
    "........\n\
     ..N.....\n\
     .....R..\n\
     ........\n\
     .....n..\n\
     n.R..N..\n\
     ........\n\
     ........\n";
    let mv1 =
    "........\n\
     ........\n\
     ..q.....\n\
     ..q.....\n\
     ..q.....\n\
     cqsqq...\n\
     ..q.....\n\
     ..q.....\n";
    let mv2 =
    ".....q..\n\
     .....q..\n\
     qqqqqsqq\n\
     .....q..\n\
     .....c..\n\
     ........\n\
     ........\n\
     ........\n";
    let mv3 =
    "q...q...\n\
     ..s.....\n\
     q...q...\n\
     .q.q....\n\
     ........\n\
     ........\n\
     ........\n\
     ........\n";
    let mv4 =
    "........\n\
     ........\n\
     ........\n\
     ....q.q.\n\
     ...q...q\n\
     .....s..\n\
     ...q...q\n\
     ....q.q.\n";
    test_move_gen(board, &[mv1, mv2, mv3, mv4])
}

#[rustfmt::skip]
#[test]
fn move_test3() {
    let board =
    "........\n\
     .....n..\n\
     .R...R..\n\
     ........\n\
     .n.Q....\n\
     .....n..\n\
     nQ.n....\n\
     ....B...\n";
    let mv1 =
    ".q......\n\
     .q......\n\
     qsqqq...\n\
     .q......\n\
     .c......\n\
     ........\n\
     ........\n\
     ........\n";
    let mv2 =
    "........\n\
     .....c..\n\
     ..qqqsqq\n\
     .....q..\n\
     .....q..\n\
     .....c..\n\
     ........\n\
     ........\n";
    let mv3 =
    "...q....\n\
     ...q....\n\
     ...q....\n\
     ..qqq...\n\
     .cqsqqqq\n\
     ..qqq...\n\
     ...c.q..\n\
     ......q.\n";
    let mv4 =
    "........\n\
     ........\n\
     ........\n\
     ........\n\
     .c......\n\
     qqq.....\n\
     csqc....\n\
     qqq.....\n";
    let mv5 =
    "........\n\
     ........\n\
     ........\n\
     ........\n\
     .......q\n\
     ......q.\n\
     ...c.q..\n\
     ....s...\n";
    test_move_gen(board, &[mv1, mv2, mv3, mv4, mv5])
}

#[rustfmt::skip]
#[test]
fn move_test4() {
    let board =
    "........\n\
     ........\n\
     ..n.....\n\
     ........\n\
     ........\n\
     ..Q..N..\n\
     ........\n\
     ........\n";
    let mv1 =
    ".......q\n\
     ......q.\n\
     ..c..q..\n\
     q.q.q...\n\
     .qqq....\n\
     qqsqq...\n\
     .qqq....\n\
     q.q.q...\n";
    let mv2 =
    "........\n\
     ........\n\
     ........\n\
     ....q.q.\n\
     ...q...q\n\
     .....s..\n\
     ...q...q\n\
     ....q.q.\n";
    test_move_gen(board, &[mv1, mv2])
}

fn test_move_gen(s: &str, mv_strs: &[&str]) {
    let ps = BbPieceState::from_pretty_string(s).unwrap();
    let expected_mvs = moves_from_pretty_string(mv_strs);
    let actual_mvs: HashSet<_> = generate_all_moves(&ps, &Side::White).into_iter().collect();
    dbg!(actual_mvs.difference(&expected_mvs).collect::<Vec<_>>());
    dbg!(expected_mvs.difference(&actual_mvs).collect::<Vec<_>>());
    assert_eq!(expected_mvs, actual_mvs);
}

fn moves_from_pretty_string(mv_strs: &[&str]) -> std::collections::HashSet<(Move, MoveType)> {
    let mut mvs = HashSet::<_>::new();
    for mv_str in mv_strs {
        let mut tgts = Vec::<(Square, MoveType)>::new();
        let mut opt_start: Option<Square> = None;
        for (c, sq) in string_board_iter(mv_str).unwrap() {
            match c.to_ascii_lowercase() {
                'q' => tgts.push((*sq, MoveType::Quiet)),
                'c' => tgts.push((*sq, MoveType::Capture)),
                'j' => tgts.push((*sq, MoveType::CastleQueen)),
                'k' => tgts.push((*sq, MoveType::CastleKing)),
                'p' => {
                    tgts.push((*sq, MoveType::Promote(PromotionType::Bishop)));
                    tgts.push((*sq, MoveType::Promote(PromotionType::Rook)));
                    tgts.push((*sq, MoveType::Promote(PromotionType::Queen)));
                    tgts.push((*sq, MoveType::Promote(PromotionType::Knight)));
                }
                'b' => {
                    tgts.push((*sq, MoveType::PromoteCapture(PromotionType::Bishop)));
                    tgts.push((*sq, MoveType::PromoteCapture(PromotionType::Rook)));
                    tgts.push((*sq, MoveType::PromoteCapture(PromotionType::Queen)));
                    tgts.push((*sq, MoveType::PromoteCapture(PromotionType::Knight)));
                }
                's' => opt_start = Some(*sq),
                _ => (),
            }
        }
        match opt_start {
            None => (),
            Some(start) => {
                for (sq, mt) in tgts {
                    mvs.insert((Move::from_idxs(start.idx, sq.idx), mt));
                }
            }
        }
    }
    mvs
}
