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

    fn next_raw(&mut self) -> u64 {
        // Adapted from https://github.com/evanacox/beryl/blob/4ba11b89891e98b66690a0056f0c9ca1ed528205/src/libs/ksupport/src/xorshift128p.rs#L43
        let mut t = self.s0;
        let s = self.s1;
        self.s0 = s;

        t ^= t << 23;
        t ^= t >> 18;
        t ^= s ^ (s >> 5);

        self.s1 = t;

        t.wrapping_add(s)
    }

    fn refill_cache(&mut self) {
        while self.cache.len() < 64 {
            let raw = self.next_raw();
            let float_bits = (raw >> 12) & 0x3FF0000000000000;
            self.cache.push(f64::from_bits(float_bits))
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