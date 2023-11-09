#[derive(Debug)]
pub struct Rng {
    s0: u64,
    s1: u64,
    cache: Vec<f64>,
}


impl Rng {
    pub fn new(s0: u64, s1: u64) -> Self {
        Self {
            s0,
            s1,
            cache: Vec::new()
        }
    }

    fn step_raw(&mut self) {
        // Copied from Astrid's sandbox
        let mut s1 = self.s0;
        let s0 = self.s1;
        s1 ^= s1 << 23;
        s1 ^= s1 >> 17;
        s1 ^= s0;
        s1 ^= s0 >> 26;
        self.s0 = self.s1;
        self.s1 = s1;
    }

    fn next_raw(&mut self) -> f64 {
        self.step_raw();
        f64::from_bits((self.s0 >> 12) | 0x3FF0000000000000) - 1.0
    }

    fn refill_cache(&mut self) {
        while self.cache.len() < 64 {
            self.cache.push( self.next_raw())
        }
    }

    pub fn next(&mut self) -> f64 {
        if self.cache.is_empty() {
            self.refill_cache();
        }
        self.cache.pop()
            .expect("Cache should have been refilled if it was empty")
    }
}