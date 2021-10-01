use std::ops::*;

use crate::utils::count_bits;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Bitboard {
    val: u64,
}

impl Bitboard {
    #[inline]
    pub fn count_bits(&self) -> u8 {
        count_bits(self.val)
    }

    pub fn new(x: u64) -> Bitboard {
        Bitboard { val: x }
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.val &= rhs.val
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
        self.val &= rhs.val
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

impl ShlAssign for Bitboard {
    fn shl_assign(&mut self, rhs: Self) {
        self.val &= rhs.val
    }
}

impl Shl for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: Self) -> Self::Output {
        Bitboard {
            val: self.val << rhs.val,
        }
    }
}

impl ShrAssign for Bitboard {
    fn shr_assign(&mut self, rhs: Self) {
        self.val &= rhs.val
    }
}

impl Shr for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: Self) -> Self::Output {
        Bitboard {
            val: self.val << rhs.val,
        }
    }
}
