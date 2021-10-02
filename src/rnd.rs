pub trait RndGen<T> {
    fn initialize(x: T) -> Self;

    fn next(&mut self) -> T;
}

type State = [u64; 4];

struct Xoshiro256p {
    s: State,
}

impl RndGen<u64> for Xoshiro256p {
    fn initialize(x: u64) -> Self {
        Xoshiro256p {
            s: generate_init_state(x),
        }
    }

    fn next(&mut self) -> u64 {
        xoshiro256p(&mut self.s)
    }
}

#[inline]
fn xoshiro256p(state: &mut State) -> u64 {
    let res = state[0].wrapping_add(state[3]);
    let t = state[1] << 17;

    state[2] ^= state[0];
    state[3] ^= state[1];
    state[1] ^= state[2];
    state[0] ^= state[3];

    state[2] ^= t;
    state[3] = rol64(state[3], 45);
    res
}

#[inline]
fn rol64(x: u64, k: u32) -> u64 {
    (x << k) | (x >> (64 - k))
}

#[inline]
fn generate_init_state(mut init: u64) -> State {
    let a = splitmix64(&mut init);
    let b = splitmix64(&mut init);
    let c = splitmix64(&mut init);
    let d = splitmix64(&mut init);
    [a, b, c, d]
}

// SplitMix64... for generating initial values
#[inline]
fn splitmix64(x: &mut u64) -> u64 {
    *x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut ret = *x;
    ret = (ret ^ (ret >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    ret = (ret ^ (ret >> 27)).wrapping_mul(0x94D049BB133111EB);
    ret ^ (ret >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix_should_match_reference() {
        let mut x = 12345;
        let reference = [
            0x22118258a9d111a0,
            0x346edce5f713f8ed,
            0x1e9a57bc80e6721d,
            0x2d160e7e5c3f42ca,
            0x81c2e6dc980d78eb,
            0x5647e55ad933f62e,
            0x1f6622b40cb38e42,
            0x6e7411b06820371c,
            0x7ad34039583ab917,
            0xde15eab5ce53fecf,
        ];
        for r in reference {
            assert!(splitmix64(&mut x) == r);
        }
    }

    #[test]
    fn should_match_reference() {
        let mut rnd = Xoshiro256p::initialize(12345);
        let reference = [
            0x4f2790d70610546a,
            0xd2ae33f21d5120ec,
            0xa28f6ee203d01e40,
            0x213ef47a5a3a7584,
            0x6fca7fc6dca44620,
            0x551ccf8e381c05d8,
            0x923ea5d5a99f0222,
            0x6b01089f2b43f45b,
            0x3ba539cf67a16a4f,
            0x576042cbf1a61fab,
        ];
        for r in reference {
            let x = rnd.next();
            assert!(r == x);
        }
    }
}
