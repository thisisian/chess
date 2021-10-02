use std::ops::*;

use crate::enums::Rank;
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
        (self.val << (rank as u64) * 8) as u8
    }

    pub fn pretty_string(&self) -> String {
        let r8 = self.get_rank(Rank::R1);
        let r7 = self.get_rank(Rank::R2);
        let r6 = self.get_rank(Rank::R3);
        let r5 = self.get_rank(Rank::R4);
        let r4 = self.get_rank(Rank::R5);
        let r3 = self.get_rank(Rank::R6);
        let r2 = self.get_rank(Rank::R7);
        let r1 = self.get_rank(Rank::R8);
        format!(
            "{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n{:b}\n",
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
