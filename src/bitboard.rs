use std::fmt::Binary;
use std::ops::*;
use std::str::Chars;

use crate::api::{File, GameState, Move, PieceState, Rank, Side, Square};
use crate::utils::count_bits;

mod move_gen;

pub const TOP_LEFT: [Square; 64] = [
    Square::from_idx(56),
    Square::from_idx(57),
    Square::from_idx(58),
    Square::from_idx(59),
    Square::from_idx(60),
    Square::from_idx(61),
    Square::from_idx(62),
    Square::from_idx(63),
    Square::from_idx(48),
    Square::from_idx(49),
    Square::from_idx(50),
    Square::from_idx(51),
    Square::from_idx(52),
    Square::from_idx(53),
    Square::from_idx(54),
    Square::from_idx(55),
    Square::from_idx(40),
    Square::from_idx(41),
    Square::from_idx(42),
    Square::from_idx(43),
    Square::from_idx(44),
    Square::from_idx(45),
    Square::from_idx(46),
    Square::from_idx(47),
    Square::from_idx(32),
    Square::from_idx(33),
    Square::from_idx(34),
    Square::from_idx(35),
    Square::from_idx(36),
    Square::from_idx(37),
    Square::from_idx(38),
    Square::from_idx(39),
    Square::from_idx(24),
    Square::from_idx(25),
    Square::from_idx(26),
    Square::from_idx(27),
    Square::from_idx(28),
    Square::from_idx(29),
    Square::from_idx(30),
    Square::from_idx(31),
    Square::from_idx(16),
    Square::from_idx(17),
    Square::from_idx(18),
    Square::from_idx(19),
    Square::from_idx(20),
    Square::from_idx(21),
    Square::from_idx(22),
    Square::from_idx(23),
    Square::from_idx(8),
    Square::from_idx(9),
    Square::from_idx(10),
    Square::from_idx(11),
    Square::from_idx(12),
    Square::from_idx(13),
    Square::from_idx(14),
    Square::from_idx(15),
    Square::from_idx(0),
    Square::from_idx(1),
    Square::from_idx(2),
    Square::from_idx(3),
    Square::from_idx(4),
    Square::from_idx(5),
    Square::from_idx(6),
    Square::from_idx(7),
];

// TODO: This should maybe be a wrapper around a vector instead
#[derive(PartialEq)]
struct BbPieceState {
    wp: Bitboard,
    wr: Bitboard,
    wn: Bitboard,
    wb: Bitboard,
    wq: Bitboard,
    wk: Bitboard,
    bp: Bitboard,
    br: Bitboard,
    bn: Bitboard,
    bb: Bitboard,
    bq: Bitboard,
    bk: Bitboard,
}

impl PieceState for BbPieceState {
    fn pretty_string(&self) -> String {
        let mut s = String::new();
        for sq in TOP_LEFT {
            let coord = Bitboard::get_coord(sq);
            if (self.wp & coord).v > 0 {
                s.push('P')
            } else if (self.wb & coord).v > 0 {
                s.push('B')
            } else if (self.wn & coord).v > 0 {
                s.push('N')
            } else if (self.wr & coord).v > 0 {
                s.push('R')
            } else if (self.wq & coord).v > 0 {
                s.push('Q')
            } else if (self.wk & coord).v > 0 {
                s.push('K')
            } else if (self.bp & coord).v > 0 {
                s.push('p')
            } else if (self.bb & coord).v > 0 {
                s.push('b')
            } else if (self.bn & coord).v > 0 {
                s.push('n')
            } else if (self.br & coord).v > 0 {
                s.push('r')
            } else if (self.bq & coord).v > 0 {
                s.push('q')
            } else if (self.bk & coord).v > 0 {
                s.push('k')
            } else {
                s.push('.')
            }
            if (sq.idx + 1) % 8 == 0 {
                s.push('\n')
            }
        }
        s
    }

    fn start() -> Self {
        BbPieceState {
            wp: Bitboard::from_u64(0xff00),
            wr: Bitboard::from_u64(0x81),
            wn: Bitboard::from_u64(0x42),
            wb: Bitboard::from_u64(0x24),
            wq: Bitboard::from_u64(0x8),
            wk: Bitboard::from_u64(0x10),
            bp: Bitboard::from_u64(0xff000000000000),
            br: Bitboard::from_u64(0x8100000000000000),
            bn: Bitboard::from_u64(0x4200000000000000),
            bb: Bitboard::from_u64(0x2400000000000000),
            bq: Bitboard::from_u64(0x0800000000000000),
            bk: Bitboard::from_u64(0x1000000000000000),
        }
    }

    fn empty() -> Self {
        BbPieceState {
            wp: Bitboard::empty(),
            wr: Bitboard::empty(),
            wn: Bitboard::empty(),
            wb: Bitboard::empty(),
            wq: Bitboard::empty(),
            wk: Bitboard::empty(),
            bp: Bitboard::empty(),
            br: Bitboard::empty(),
            bn: Bitboard::empty(),
            bb: Bitboard::empty(),
            bq: Bitboard::empty(),
            bk: Bitboard::empty(),
        }
    }

    fn is_legal(&self) -> bool {
        fn correct_number_of_kings(ps: &BbPieceState) -> bool {
            let kings = ps.bk | ps.wk;
            kings.count_bits() == 2
        }

        fn no_pieces_on_same_square(ps: &BbPieceState) -> bool {
            let ps = ps.wp
                & ps.wr
                & ps.wn
                & ps.wb
                & ps.wq
                & ps.wk
                & ps.bp
                & ps.br
                & ps.bn
                & ps.bb
                & ps.bq
                & ps.bk;
            ps.v == 0
        }

        return correct_number_of_kings(self) && no_pieces_on_same_square(self);
    }

    fn from_pretty_string(s: &str) -> Option<Self> {
        match string_board_iter(s) {
            None => None,
            Some(it) => {
                let mut ps = BbPieceState::empty();
                for (c, sq) in it {
                    let opt_bb = match c {
                        'N' => Some(&mut ps.wn),
                        'B' => Some(&mut ps.wb),
                        'K' => Some(&mut ps.wk),
                        'Q' => Some(&mut ps.wq),
                        'P' => Some(&mut ps.wp),
                        'R' => Some(&mut ps.wr),
                        'n' => Some(&mut ps.bn),
                        'b' => Some(&mut ps.bb),
                        'k' => Some(&mut ps.bk),
                        'q' => Some(&mut ps.bq),
                        'p' => Some(&mut ps.bp),
                        'r' => Some(&mut ps.br),
                        _ => None,
                    };
                    match opt_bb {
                        Some(bb) => {
                            *bb = *bb | (1 << sq.idx);
                        }
                        None => (),
                    }
                }
                Some(ps)
            }
        }
    }
}

impl BbPieceState {
    pub fn white_pieces(&self) -> Bitboard {
        Bitboard {
            v: self.wb.v | self.wk.v | self.wn.v | self.wp.v | self.wq.v | self.wr.v,
        }
    }
    pub fn black_pieces(&self) -> Bitboard {
        Bitboard {
            v: self.bb.v | self.bk.v | self.bn.v | self.bp.v | self.bq.v | self.br.v,
        }
    }

    //pub fn move_from_bitboards(&self, piece: &Piece,& from: &Bitboard, to: &Bitboard) -> Self {
    //    // TODO: debug assert both bitboard are 1 square
    //    let mut ps = BbPieceState {
    //        wp: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        wr: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        wn: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        wb: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        wq: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        wk: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        bp: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        br: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        bn: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        bb: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        bq: Bitboard { v: self.wp.v & !to.v & !from.v},
    //        bk: Bitboard { v: self.wp.v & !to.v & !from.v},
    //    }
    //    piece match {
    //        Piece::P =>
    //        Piece::R =>
    //        Piece::Q =>
    //        Piece::K =>
    //        Piece::N =>
    //        Piece::B =>
    //    }
    //    ps
    //}
}

pub struct BbGameState {
    pieces: BbPieceState,
    to_move: Side,
    en_passant: Option<File>,
    reversable_moves: u8,
    w_kingside_castling: bool,
    w_queenside_castling: bool,
    b_kingside_castling: bool,
    b_queenside_castling: bool,
}

impl GameState for BbGameState {
    fn start() -> Self {
        BbGameState {
            pieces: BbPieceState::start(),
            to_move: Side::White,
            en_passant: None,
            reversable_moves: 0,
            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,
        }
    }

    fn pretty_string(&self) -> String {
        self.pieces.pretty_string()
    }

    fn is_legal(&self) -> bool {
        self.pieces.is_legal()
    }

    fn make_move(&self, _m: Move) -> Option<BbGameState> {
        todo!()
    }

    fn to_fen(&self) -> String {
        todo!()
    }

    fn from_fen(s: String) -> Option<Self> {
        let mut chars = s.as_str().chars();
        let mut pieces = BbPieceState::empty();
        parse_line(&mut chars, &mut pieces, Rank::R8)?;
        parse_char(&mut chars, &'/');
        parse_line(&mut chars, &mut pieces, Rank::R7)?;
        parse_char(&mut chars, &'/')?;
        parse_line(&mut chars, &mut pieces, Rank::R6)?;
        parse_char(&mut chars, &'/')?;
        parse_line(&mut chars, &mut pieces, Rank::R5)?;
        parse_char(&mut chars, &'/')?;
        parse_line(&mut chars, &mut pieces, Rank::R4)?;
        parse_char(&mut chars, &'/')?;
        parse_line(&mut chars, &mut pieces, Rank::R3)?;
        parse_char(&mut chars, &'/')?;
        parse_line(&mut chars, &mut pieces, Rank::R2)?;
        parse_char(&mut chars, &'/')?;
        parse_line(&mut chars, &mut pieces, Rank::R1)?;
        parse_char(&mut chars, &' ')?;
        let to_move = parse_to_move(&mut chars)?;
        parse_char(&mut chars, &' ')?;
        let (w_kingside_castling, w_queenside_castling, b_kingside_castling, b_queenside_castling) =
            parse_castling(&mut chars)?;
        let en_passant = parse_en_passant(&mut chars)?;
        parse_char(&mut chars, &' ')?;
        let reversable_moves = parse_num(&mut chars)? as u8;
        //let _whole_moves = parse_num(&mut chars)?;

        fn parse_line(chars: &mut Chars, ps: &mut BbPieceState, r: Rank) -> Option<()> {
            let mut square_ct: u8 = 0;
            while square_ct < 8 {
                let c = chars.next()?;
                match c {
                    '1'..='9' => square_ct += c.to_string().parse::<u8>().unwrap(),
                    'P' => ps.wp.v |= (1 << square_ct) << (r as u8 * 8),
                    'R' => ps.wr.v |= (1 << square_ct) << (r as u8 * 8),
                    'N' => ps.wn.v |= (1 << square_ct) << (r as u8 * 8),
                    'B' => ps.wb.v |= (1 << square_ct) << (r as u8 * 8),
                    'Q' => ps.wq.v |= (1 << square_ct) << (r as u8 * 8),
                    'K' => ps.wk.v |= (1 << square_ct) << (r as u8 * 8),
                    'p' => ps.bp.v |= (1 << square_ct) << (r as u8 * 8),
                    'r' => ps.br.v |= (1 << square_ct) << (r as u8 * 8),
                    'n' => ps.bn.v |= (1 << square_ct) << (r as u8 * 8),
                    'b' => ps.bb.v |= (1 << square_ct) << (r as u8 * 8),
                    'q' => ps.bq.v |= (1 << square_ct) << (r as u8 * 8),
                    'k' => ps.bk.v |= (1 << square_ct) << (r as u8 * 8),
                    _ => return None,
                }
                if !c.is_numeric() {
                    square_ct += 1;
                }
            }
            if square_ct == 8 {
                Some(())
            } else {
                None
            }
        }

        fn parse_char(chars: &mut Chars, char: &char) -> Option<()> {
            chars.next().filter(|x| x == char).map(|_x| ())
        }

        fn parse_to_move(chars: &mut Chars) -> Option<Side> {
            match chars.next()? {
                'w' => Some(Side::White),
                'b' => Some(Side::Black),
                _ => None,
            }
        }

        fn parse_castling(chars: &mut Chars) -> Option<(bool, bool, bool, bool)> {
            let mut c = chars.next()?;
            if c == '-' {
                Some((false, false, false, false))
            } else if c == ' ' {
                None
            } else {
                let mut w_kingside_castling = false;
                let mut w_queenside_castling = false;
                let mut b_kingside_castling = false;
                let mut b_queenside_castling = false;

                while c != ' ' {
                    match c {
                        'K' => w_kingside_castling = true,
                        'Q' => w_queenside_castling = true,
                        'k' => b_kingside_castling = true,
                        'q' => b_queenside_castling = true,
                        _ => return None,
                    }
                    c = chars.next()?;
                }
                Some((
                    w_kingside_castling,
                    w_queenside_castling,
                    b_kingside_castling,
                    b_queenside_castling,
                ))
            }
        }

        fn parse_en_passant(chars: &mut Chars) -> Option<Option<File>> {
            let c = chars.next()?;
            match c {
                '-' => Some(None),
                'a'..='e' => {
                    let c = chars.next()?;
                    if '1' <= c && c <= '8' {
                        Some(Some(c as u8 - '0' as u8))
                    } else {
                        // Failed to parse
                        None
                    }
                }
                _ => None,
            }
        }

        fn parse_num(chars: &mut Chars) -> Option<u8> {
            let mut c = chars.next();
            let mut x = 0;
            if !c.is_some_and(|x| x.is_ascii_digit()) {
                return None;
            }
            while c.is_some_and(|x| x.is_ascii_digit()) {
                x = x * 10 + c?.to_digit(10)?;
                c = chars.next();
            }
            Some(x as u8)
        }

        Some(BbGameState {
            pieces: pieces,
            to_move: to_move,
            en_passant: en_passant,
            reversable_moves: reversable_moves,
            w_kingside_castling: w_kingside_castling,
            w_queenside_castling: w_queenside_castling,
            b_kingside_castling: b_kingside_castling,
            b_queenside_castling: b_queenside_castling,
        })
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Bitboard {
    pub v: u64,
}

type RankBits = u8;

impl Bitboard {
    pub const fn from_u64(v: u64) -> Bitboard {
        Bitboard { v }
    }

    pub const fn from_square(sq: Square) -> Bitboard {
        Bitboard {
            v: 1 >> (sq.idx - 1),
        }
    }

    pub const fn empty() -> Bitboard {
        Bitboard { v: 0 }
    }

    pub const fn full() -> Bitboard {
        Bitboard { v: !0 }
    }

    const fn from_rank(f: File) -> Bitboard {
        Bitboard::from_u64(0xffff << f)
    }

    const fn shift_up(&self, shift: u8) -> Bitboard {
        Bitboard {
            v: self.v << shift * 8,
        }
    }

    const fn shift_down(&self, shift: u8) -> Bitboard {
        Bitboard {
            v: self.v >> shift * 8,
        }
    }

    const fn get_coord(s: Square) -> Bitboard {
        Bitboard::from_u64(1 << s.idx)
    }

    pub fn count_bits(&self) -> u8 {
        count_bits(self.v)
    }

    const fn is_subset(&self, rhs: &Bitboard) -> bool {
        self.v & rhs.v == self.v
    }

    const fn is_disjoint(&self, rhs: &Bitboard) -> bool {
        self.v & rhs.v == 0
    }

    const fn is_intersecting(&self, rhs: &Bitboard) -> bool {
        !self.is_disjoint(rhs)
    }

    fn get_rank(&self, rank: Rank) -> RankBits {
        (self.v >> ((rank as u64) * 8)) as u8
    }

    const fn is_empty(&self) -> bool {
        self.v == 0
    }

    const fn is_nonempty(&self) -> bool {
        !self.is_empty()
    }

    pub fn pretty_string(&self) -> String {
        let r8: String = format!("{:0>8b}", self.get_rank(Rank::R8))
            .chars()
            .rev()
            .collect();
        let r7: String = format!("{:0>8b}", self.get_rank(Rank::R7))
            .chars()
            .rev()
            .collect();
        let r6: String = format!("{:0>8b}", self.get_rank(Rank::R6))
            .chars()
            .rev()
            .collect();
        let r5: String = format!("{:0>8b}", self.get_rank(Rank::R5))
            .chars()
            .rev()
            .collect();
        let r4: String = format!("{:0>8b}", self.get_rank(Rank::R4))
            .chars()
            .rev()
            .collect();
        let r3: String = format!("{:0>8b}", self.get_rank(Rank::R3))
            .chars()
            .rev()
            .collect();
        let r2: String = format!("{:0>8b}", self.get_rank(Rank::R2))
            .chars()
            .rev()
            .collect();
        let r1: String = format!("{:0>8b}", self.get_rank(Rank::R1))
            .chars()
            .rev()
            .collect();
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            r8, r7, r6, r5, r4, r3, r2, r1
        )
    }
}

impl Binary for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.v, f)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.v ^= rhs.v
    }
}

impl BitXorAssign<u64> for Bitboard {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.v ^= rhs
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard { v: self.v ^ rhs.v }
    }
}

impl BitXor<u64> for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: u64) -> Self::Output {
        Bitboard { v: self.v ^ rhs }
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.v |= rhs.v
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.v |= rhs
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard { v: self.v | rhs.v }
    }
}

impl BitOr<u64> for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: u64) -> Self::Output {
        Bitboard { v: self.v | rhs }
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.v &= rhs.v
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.v &= rhs
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard { v: self.v & rhs.v }
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: u64) -> Self::Output {
        Bitboard { v: self.v & rhs }
    }
}

impl Shl<u8> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard { v: self.v << rhs }
    }
}

impl ShlAssign<u8> for Bitboard {
    fn shl_assign(&mut self, rhs: u8) {
        self.v <<= rhs;
    }
}

impl Shr<u8> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard { v: self.v >> rhs }
    }
}

impl ShrAssign<u8> for Bitboard {
    fn shr_assign(&mut self, rhs: u8) {
        self.v >>= rhs;
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard { v: !self.v }
    }
}

fn string_board_iter(s: &str) -> Option<impl Iterator<Item = (char, &Square)>> {
    if s.lines().any(|l| l.len() != 8) {
        None
    } else if s.lines().count() != 8 {
        None
    } else {
        Some(
            s.chars()
                .filter(|c| *c != '\n')
                .zip(TOP_LEFT.iter())
                .into_iter(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::bitboard::*;

    #[test]
    fn subset_tests() {
        let b1: Bitboard = Bitboard {
            v: 0xfedcba9876543210,
        };
        let b2: Bitboard = !b1;
        assert!(!b1.is_subset(&b2));
        let b3 = b1 & b2;
        assert!(b3.is_subset(&b2));
        assert!(!Bitboard::from_u64(1).is_subset(&Bitboard::from_u64(0)));
        assert!(Bitboard::from_u64(0).is_subset(&Bitboard::from_u64(1)));
    }

    #[test]
    fn disjoint_tests() {
        assert!(Bitboard::from_u64(1).is_disjoint(&Bitboard::from_u64(0)));
        assert!(Bitboard::from_u64(0).is_disjoint(&Bitboard::from_u64(1)));
        let b1: Bitboard = Bitboard {
            v: 0xfedcba9876543210,
        };
        let b2: Bitboard = !b1;
        assert!(b1.is_disjoint(&b2));
        assert!(b2.is_disjoint(&b1));
    }

    #[test]
    fn get_rank_tests() {
        let b: Bitboard = Bitboard {
            v: 0xfedcba9876543210,
        };
        assert!(b.get_rank(Rank::R1) == 0x10);
        assert!(b.get_rank(Rank::R2) == 0x32);
        assert!(b.get_rank(Rank::R3) == 0x54);
        assert!(b.get_rank(Rank::R4) == 0x76);
        assert!(b.get_rank(Rank::R5) == 0x98);
        assert!(b.get_rank(Rank::R6) == 0xba);
        assert!(b.get_rank(Rank::R7) == 0xdc);
        assert!(b.get_rank(Rank::R8) == 0xfe);
    }

    #[test]
    fn pretty_string_should_show_first_bit() {
        let b = Bitboard { v: 1 };

        let expected = "00000000\n\
         00000000\n\
         00000000\n\
         00000000\n\
         00000000\n\
         00000000\n\
         00000000\n\
         10000000\n";

        let actual = b.pretty_string();
        assert!(
            expected == actual,
            "expected:\n`{}`\n got:\n`{}`\n",
            expected,
            actual
        );
    }

    #[test]
    fn pretty_string_should_show_last_bit() {
        let b = Bitboard { v: 2_u64.pow(63) };

        let expected = "00000001\n\
         00000000\n\
         00000000\n\
         00000000\n\
         00000000\n\
         00000000\n\
         00000000\n\
         00000000\n";

        let actual = b.pretty_string();
        assert!(
            expected == actual,
            "expected:\n`{}`\n got:\n`{}`\n",
            expected,
            actual
        );
    }

    #[test]
    fn pretty_string_should_print_allbits() {
        let b = Bitboard {
            v: 0xfedcba9876543210,
        };
        let expected = "01111111\n\
         00111011\n\
         01011101\n\
         00011001\n\
         01101110\n\
         00101010\n\
         01001100\n\
         00001000\n";

        let actual = b.pretty_string();
        assert!(
            expected == actual,
            "expected:\n`{}`\n got:\n`{}`\n",
            expected,
            actual
        );
    }

    #[test]
    fn bbps_sould_fail_on_empty() {
        let ps = BbPieceState::empty();
        assert!(!ps.is_legal())
    }

    #[test]
    fn bbps_sould_fail_duplicate() {
        let ps = BbPieceState {
            wp: Bitboard::from_u64(0),
            wb: Bitboard::from_u64(0),
            wn: Bitboard::from_u64(2),
            wr: Bitboard::from_u64(0),
            wq: Bitboard::from_u64(0),
            wk: Bitboard::from_u64(2),
            bp: Bitboard::from_u64(0),
            bb: Bitboard::from_u64(0),
            bn: Bitboard::from_u64(2),
            br: Bitboard::from_u64(0),
            bq: Bitboard::from_u64(0),
            bk: Bitboard::from_u64(32),
        };
        assert!(ps.is_legal());
    }

    #[test]
    fn bbps_pretty_string_start() {
        let start = BbPieceState::start();
        let expected = "rnbqkbnr\n\
                             pppppppp\n\
                             ........\n\
                             ........\n\
                             ........\n\
                             ........\n\
                             PPPPPPPP\n\
                             RNBQKBNR\n";
        assert!(start.pretty_string() == expected)
    }

    #[test]
    fn fen_parse_start() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let start = BbGameState::from_fen(fen.to_string()).unwrap();
        assert!(start.w_kingside_castling == true);
        assert!(start.w_queenside_castling == true);
        assert!(start.b_kingside_castling == true);
        assert!(start.b_queenside_castling == true);
        assert!(start.en_passant == None);
        assert!(start.reversable_moves == 0);
        assert!(start.to_move == Side::White);
        assert!(start.pieces == BbPieceState::start());
        assert!(start.pieces.is_legal());
    }

    #[test]
    fn fen_parse_game_1() {
        let fen = "rnbqkbnr/pp2pppp/3p4/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 3";
        let game = BbGameState::from_fen(fen.to_string()).unwrap();
        assert!(game.w_kingside_castling == true);
        assert!(game.w_queenside_castling == true);
        assert!(game.b_kingside_castling == true);
        assert!(game.b_queenside_castling == true);
        assert!(game.en_passant == None);
        assert!(game.reversable_moves == 0);
        assert!(game.to_move == Side::Black);
        let expected = "rnbqkbnr\n\
             pp..pppp\n\
             ...p....\n\
             ..p.....\n\
             ...PP...\n\
             .....N..\n\
             PPP..PPP\n\
             RNBQKB.R\n"
            .to_string();
        assert!(game.pieces.pretty_string() == expected);
        assert!(game.pieces.is_legal());
    }
}
