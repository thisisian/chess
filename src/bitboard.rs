use std::fmt::Binary;
use std::ops::*;
use std::str::Chars;

use crate::api::{File, PieceState, Rank, Side, Square};
use crate::utils::count_bits;

pub const TOP_LEFT: [Square; 64] = [
    Square::new(56),
    Square::new(57),
    Square::new(58),
    Square::new(59),
    Square::new(60),
    Square::new(61),
    Square::new(62),
    Square::new(63),
    Square::new(48),
    Square::new(49),
    Square::new(50),
    Square::new(51),
    Square::new(52),
    Square::new(53),
    Square::new(54),
    Square::new(55),
    Square::new(40),
    Square::new(41),
    Square::new(42),
    Square::new(43),
    Square::new(44),
    Square::new(45),
    Square::new(46),
    Square::new(47),
    Square::new(32),
    Square::new(33),
    Square::new(34),
    Square::new(35),
    Square::new(36),
    Square::new(37),
    Square::new(38),
    Square::new(39),
    Square::new(24),
    Square::new(25),
    Square::new(26),
    Square::new(27),
    Square::new(28),
    Square::new(29),
    Square::new(30),
    Square::new(31),
    Square::new(16),
    Square::new(17),
    Square::new(18),
    Square::new(19),
    Square::new(20),
    Square::new(21),
    Square::new(22),
    Square::new(23),
    Square::new(8),
    Square::new(9),
    Square::new(10),
    Square::new(11),
    Square::new(12),
    Square::new(13),
    Square::new(14),
    Square::new(15),
    Square::new(0),
    Square::new(1),
    Square::new(2),
    Square::new(3),
    Square::new(4),
    Square::new(5),
    Square::new(6),
    Square::new(7),
];
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

pub struct BbBoardState {
    pieces: BbPieceState,
    to_move: Side,
    en_passant: Option<File>,
    reversable_moves: u8,
    w_kingside_castling: bool,
    w_queenside_castling: bool,
    b_kingside_castling: bool,
    b_queenside_castling: bool,
}

impl PieceState for BbPieceState {
    fn pretty_print(&self) -> String {
        let mut s = String::new();
        for square in TOP_LEFT {
            let coord = Bitboard::get_coord(square);
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
            if (square.v + 1) % 8 == 0 {
                s.push('\n')
            }
        }
        s
    }

    fn start() -> Self {
        BbPieceState {
            wp: Bitboard::new(0xff00),
            wr: Bitboard::new(0x81),
            wn: Bitboard::new(0x42),
            wb: Bitboard::new(0x24),
            wq: Bitboard::new(0x8),
            wk: Bitboard::new(0x10),
            bp: Bitboard::new(0xff000000000000),
            br: Bitboard::new(0x8100000000000000),
            bn: Bitboard::new(0x4200000000000000),
            bb: Bitboard::new(0x2400000000000000),
            bq: Bitboard::new(0x0800000000000000),
            bk: Bitboard::new(0x1000000000000000),
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

    fn make_move(&self, m: crate::api::Move) -> Self {
        todo!()
    }
}

pub fn parse_fen(s: String) -> Option<BbBoardState> {
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
                    Some(File::from_char(c))
                } else {
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

    Some(BbBoardState {
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

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Bitboard {
    pub v: u64,
}

type RankBits = u8;

impl Bitboard {
    pub const fn new(v: u64) -> Bitboard {
        Bitboard { v }
    }

    pub const fn empty() -> Bitboard {
        Bitboard { v: 0 }
    }

    pub const fn full() -> Bitboard {
        Bitboard { v: !0 }
    }

    const fn rank(r: Rank) -> Bitboard {
        todo!()
    }

    const fn file(f: File) -> Bitboard {
        todo!()
    }

    const fn shift_h(&self) -> Bitboard {
        todo!()
    }

    const fn shift_v(&self) -> Bitboard {
        todo!()
    }

    const fn get_coord(s: Square) -> Bitboard {
        Bitboard::new(1 << s.v)
    }

    pub fn count_bits(&self) -> u8 {
        count_bits(self.v)
    }

    const fn is_subset(&self, rhs: Bitboard) -> bool {
        self.v & rhs.v == self.v
    }

    const fn is_disjoint(&self, rhs: Bitboard) -> bool {
        self.v & rhs.v == 0
    }

    fn get_rank(&self, rank: Rank) -> RankBits {
        (self.v >> ((rank as u64) * 8)) as u8
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

//// Convert (North, 8) to Movement Squares
//fn direction_to_move((s1, direction, dist): Compass) -> Square {
//    if (direction == Equal) {
//        return Bitboard ;
//    }
//
//    let x = match direction {
//        N => 8,
//        NE => 9,
//        E => 1,
//        SE => -7,
//        S => -8,
//        SW => -9,
//        W => -1,
//        Equal => 0,
//    };
//    s + x * dist

#[cfg(test)]
mod tests {
    use crate::bitboard::*;

    #[test]
    fn subset_tests() {
        let b1: Bitboard = Bitboard {
            v: 0xfedcba9876543210,
        };
        let b2: Bitboard = !b1;
        assert!(!b1.is_subset(b2));
        let b3 = b1 & b2;
        assert!(b3.is_subset(b2));
        assert!(!Bitboard::new(1).is_subset(Bitboard::new(0)));
        assert!(Bitboard::new(0).is_subset(Bitboard::new(1)));
    }

    #[test]
    fn disjoint_tests() {
        assert!(Bitboard::new(1).is_disjoint(Bitboard::new(0)));
        assert!(Bitboard::new(0).is_disjoint(Bitboard::new(1)));
        let b1: Bitboard = Bitboard {
            v: 0xfedcba9876543210,
        };
        let b2: Bitboard = !b1;
        assert!(b1.is_disjoint(b2));
        assert!(b2.is_disjoint(b1));
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
        assert!(ps.is_legal());
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
        assert!(start.pretty_print() == expected)
    }

    #[test]
    fn fen_parse_start() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let start = parse_fen(fen.to_string()).unwrap();
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
        let game = parse_fen(fen.to_string()).unwrap();
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
        assert!(game.pieces.pretty_print() == expected);
        assert!(game.pieces.is_legal());
    }
}
