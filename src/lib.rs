use num::{BigInt, ToPrimitive};

pub struct ByteGenerator {
    generator: Pi,
}

impl ByteGenerator {
    pub fn new() -> Self {
        Self {
            generator: Pi::new(),
        }
    }
}

impl Default for ByteGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for ByteGenerator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let next_byte = (&mut self.generator)
            .take(4)
            .map(|b| b & 0b11)
            .fold(0u8, |acc, lower_bits| (acc << 2) | lower_bits);

        Some(next_byte)
    }
}

pub struct Pi {
    q: BigInt,
    r: BigInt,
    t: BigInt,
    k: BigInt,
    n: BigInt,
    l: BigInt,
    first: bool,
}

impl Pi {
    pub fn new() -> Self {
        Self {
            q: BigInt::from(1),
            r: BigInt::from(0),
            t: BigInt::from(1),
            k: BigInt::from(1),
            n: BigInt::from(3),
            l: BigInt::from(3),
            first: true,
        }
    }
}

impl Default for Pi {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Pi {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if &self.q * 4 + &self.r - &self.t < &self.n * &self.t {
                if self.first {
                    self.first = false;
                }
                let tmp_n = self.n.to_u8();
                let nr = (&self.r - &self.n * &self.t) * 10;
                self.n = (&self.q * 3 + &self.r) * 10 / &self.t - &self.n * 10;
                self.q *= 10;
                self.r = nr;
                return tmp_n;
            } else {
                let nr = (&self.q * 2 + &self.r) * &self.l;
                let nn = (&self.q * &self.k * 7 + 2 + &self.r * &self.l) / (&self.t * &self.l);
                self.q *= &self.k;
                self.t *= &self.l;
                self.l += 2;
                self.k += 1;
                self.n = nn;
                self.r = nr;
            }
        }
    }
}
