// Algorithm by Brian Kernigham
#[inline]
pub fn count_bits(mut x: u64) -> u8 {
    let mut count: u8 = 0;
    while x != 0 {
        x = x & (x - 1);
        count += 1;
    }
    count
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_bits_max_64() {
        assert!(count_bits(u64::MAX) == 64);
    }

    #[test]
    fn count_bits_zero_zero() {
        assert!(count_bits(0) == 0);
    }

    #[test]
    fn count_bits_1_1() {
        assert!(count_bits(1) == 1);
    }

    #[test]
    fn count_bits_2_1() {
        assert!(count_bits(2) == 1);
    }

    #[test]
    fn count_bits_e_3() {
        assert!(count_bits(0xe) == 3);
    }

    #[test]
    fn count_bits_max_min_1_63() {
        assert!(count_bits(u64::MAX - 1) == 63);
    }

    #[test]
    fn count_bits_max_min_es_48() {
        assert!(count_bits(0xeeeeeeeeeeeeeeee) == 48);
    }

    #[test]
    fn count_bits_33() {
        assert!(
            count_bits(0b0101011101101001100000011011000100110111111010111011000110100001) == 33
        );
    }
}
