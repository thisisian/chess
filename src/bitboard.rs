use std::ops::*;

use crate::enums::{File, Rank};
use crate::utils::count_bits;

type FileBits = u8;
type RankBits = u8;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Bitboard {
    pub val: u64,
}

impl Bitboard {
    pub const fn new(x: u64) -> Bitboard {
        Bitboard { val: x }
    }

    pub fn empty() -> Bitboard {
        Bitboard { val: 0 }
    }

    pub fn universe() -> Bitboard {
        Bitboard { val: !0 }
    }

    pub fn count_bits(&self) -> u8 {
        count_bits(self.val)
    }

    pub fn is_subset(&self, rhs: Bitboard) -> bool {
        *self & rhs == *self
    }

    pub fn is_disjoint(&self, rhs: Bitboard) -> bool {
        *self & rhs == Bitboard::empty()
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
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.val ^= rhs.val
    }
}

impl BitXorAssign<u64> for Bitboard {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.val ^= rhs
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

impl BitXor<u64> for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: u64) -> Self::Output {
        Bitboard {
            val: self.val ^ rhs,
        }
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.val |= rhs.val
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.val |= rhs
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

impl BitOr<u64> for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: u64) -> Self::Output {
        Bitboard {
            val: self.val | rhs,
        }
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.val &= rhs.val
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.val &= rhs
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

impl BitAnd<u64> for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: u64) -> Self::Output {
        Bitboard {
            val: self.val & rhs,
        }
    }
}

impl Shl<u8> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard {
            val: self.val << rhs,
        }
    }
}

impl ShlAssign<u8> for Bitboard {
    fn shl_assign(&mut self, rhs: u8) {
        self.val <<= rhs;
    }
}

impl Shr<u8> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard {
            val: self.val >> rhs,
        }
    }
}

impl ShrAssign<u8> for Bitboard {
    fn shr_assign(&mut self, rhs: u8) {
        self.val >>= rhs;
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard { val: !self.val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subset_tests() {
        let b1: Bitboard = Bitboard { val: 0xfedcba9876543210 };
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
        let b1: Bitboard = Bitboard { val: 0xfedcba9876543210 };
        let b2: Bitboard = !b1;
        assert!(b1.is_disjoint(b2));
        assert!(b2.is_disjoint(b1));
    }

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
