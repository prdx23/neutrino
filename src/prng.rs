
// Reference:
// https://prng.di.unimi.it/xoshiro128plus.c
// https://github.com/rust-random/rngs/blob/master/rand_xoshiro/src/xoroshiro128plus.rs


pub struct Xoroshiro128Plus {
    s0: u64,
    s1: u64,
}

impl Xoroshiro128Plus {

    pub fn new(s0: u64, s1: u64) -> Self {
        let mut prng = Self { s0, s1 };
        prng.jump();
        prng
    }

    pub fn random_u64(&mut self) -> u64 {
        let result = self.s0.wrapping_add(self.s1);
        self.s1 ^= self.s0;
        self.s0 = self.s0.rotate_left(24) ^ self.s1 ^ (self.s1 << 16);
        self.s1 = self.s1.rotate_left(37);
        result
    }

    pub fn random_f32(&mut self) -> f32 {
        self.random_u64() as f32 / u64::MAX as f32
    }

    pub fn random_f32_bw(&mut self, a: f32, b: f32) -> f32 {
        (self.random_f32() * (b - a)) + a
    }

    pub fn jump(&mut self) {
        const JUMP: [u64; 2] = [0xdf900294d8f554a5, 0x170865df4b3201fc];
        let mut s0 = 0;
        let mut s1 = 0;
        for j in &JUMP {
            for b in 0..64 {
                if (j & 1 << b) != 0 {
                    s0 ^= self.s0;
                    s1 ^= self.s1;
                }
                self.random_u64();
            }
        }
        self.s0 = s0;
        self.s1 = s1;
    }
}
