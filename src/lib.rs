use num::{BigInt, ToPrimitive};

pub struct ByteGenerator {
    generator: DigitsOfPi,
}

impl ByteGenerator {
    pub fn new() -> Self {
        Self {
            generator: DigitsOfPi::new(),
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

/// Returns the digits of pi, skipping the decimal, i.e. `31415`
pub struct DigitsOfPi {
    q: BigInt,
    r: BigInt,
    t: BigInt,
    k: BigInt,
    n: BigInt,
    l: BigInt,
    first: bool,
}

impl DigitsOfPi {
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

impl Default for DigitsOfPi {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for DigitsOfPi {
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

#[cfg(test)]
mod tests {

    use std::f64::consts;

    #[test]
    fn should_generate_equivalent_pi_value_to_stdlib_constant() {
        let pi_const = consts::PI.to_string();
        let pi_generated = super::DigitsOfPi::new();

        // append a 3 and skip the `3.` from the pi constant. to normalize to only digits
        let digits_of_pi_const = "3"
            .chars()
            .chain(pi_const.chars().skip(2))
            // it's safe to assume these will always be representable as a u8 as the digit will never exceed `0-9`.
            .map(|ascii_digit| ascii_digit.to_digit(10).map(|digit| digit as u8).unwrap());
        let pairing = digits_of_pi_const.zip(pi_generated);

        for (idx, (expected, got)) in pairing.enumerate() {
            assert_eq!(
                &expected, &got,
                "failed at index: {}\nexpected: {}, got: {}",
                idx, &expected, &got
            )
        }
    }
}
