use std::ops::*;

use crate::enums::{File, Rank};
use crate::utils::count_bits;

type FileBits = u8;
type RankBits = u8;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Bitboard {
    val: u64,
}

impl Bitboard {
    pub fn count_bits(&self) -> u8 {
        count_bits(self.val)
    }

    pub fn get_rank(&self, rank: Rank) -> RankBits {
        (self.val >> ((rank as u64) * 8)) as u8
    }

    pub fn get_file(&self, file: File) -> RankBits {
        todo!()
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

    pub fn new(x: u64) -> Bitboard {
        Bitboard { val: x }
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.val ^= rhs.val
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard {
            val: self.val ^ rhs.val,
        }
    }
}
impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.val |= rhs.val
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard {
            val: self.val | rhs.val,
        }
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.val &= rhs.val
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard {
            val: self.val & rhs.val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rank_tests() {
        let b: Bitboard = Bitboard {
            val: 0xfedcba9876543210,
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
        let b = Bitboard { val: 1 };

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
        let b = Bitboard { val: 2_u64.pow(63) };

        for i in 0..64 {
            let x = Bitboard { val: 2 ^ i };
            println!("{}:\n{}\n", i, x.pretty_string());
        }

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
            val: 0xfedcba9876543210,
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

}
