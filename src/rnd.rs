type State = [u64; 4];

pub trait RndGen<T> {
    fn initialize(x: T) -> Self;

    fn next(&mut self) -> T;
}

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
    let res = state[0] + state[3];
    let t = state[1] << 17;

    state[2] ^= state[0];
    state[3] ^= state[1];
    state[1] ^= state[2];
    state[0] ^= state[3];

    state[2] ^= t;
    state[3] ^= rol64(state[3], 45);
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
    *x += 0x9E3779B97f4A7C15;
    let mut ret = *x;
    ret = (ret ^ (ret >> 30)) * 0xBF58476D1CE4E5B9;
    ret = (ret ^ (ret >> 27)) * 0x94D049BB133111EB;
    ret ^ (ret >> 31)
}
