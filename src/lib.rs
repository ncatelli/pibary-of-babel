use num::{BigInt, ToPrimitive};
use std::{collections::VecDeque, ops::Rem};

mod prime;

pub trait PiDigitGenerator: Iterator<Item = u8> {}

pub struct ByteGenerator<P>
where
    P: PiDigitGenerator,
{
    generator: P,
}

impl<P> ByteGenerator<P>
where
    P: PiDigitGenerator,
{
    pub fn new(pi_generator: P) -> Self {
        Self {
            generator: pi_generator,
        }
    }
}

impl<P> Default for ByteGenerator<P>
where
    P: PiDigitGenerator + Default,
{
    fn default() -> Self {
        Self::new(P::default())
    }
}

impl<P> Iterator for ByteGenerator<P>
where
    P: PiDigitGenerator,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let next_byte = (&mut self.generator)
            .take(4)
            .map(|b| b & 0b11)
            .fold(0u8, |acc, lower_bits| (acc << 2) | lower_bits);

        Some(next_byte)
    }
}

/// Returns the digits of pi, skipping the decimal, i.e. `31415`, generated
/// using the Spigot algorithm.
pub struct Spigot {
    q: BigInt,
    r: BigInt,
    t: BigInt,
    k: BigInt,
    n: BigInt,
    l: BigInt,
    first: bool,
}

impl Spigot {
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

impl PiDigitGenerator for Spigot {}

impl Default for Spigot {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Spigot {
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

fn mul_mod(a: i64, b: i64, m: i64) -> i64 {
    (a * b).rem(m)
}

fn mod_inverse(mut a: i64, n: i64) -> i64 {
    let mut i = n;
    let mut v = 0;
    let mut d = 1;
    while a > 0 {
        let t = i / a;
        let mut x = a;
        a = i.rem(x);
        i = x;
        x = d;
        d = v - t * x;
        v = x;
    }

    v = v.rem(n);
    if v < 0 {
        v = (v + n).rem(n);
    }
    v
}

fn pow_mod(a: i64, b: i64, m: i64) -> i64 {
    if b == 0 {
        1
    } else if b == 1 {
        a
    } else {
        let temp = pow_mod(a, b / 2, m);
        if b.rem(2) == 0 {
            (temp * temp).rem(m)
        } else {
            (((temp * temp).rem(m)) * a).rem(m)
        }
    }
}

fn get_next_n_dec_digits<const EXP: i32>(n: i32) -> i32 {
    const MANTISA_MAX: i64 = !(0xFFFi64 << f64::MANTISSA_DIGITS);

    let big_n = {
        let big_n_float = f64::from(n + 20) * 10_f64.ln() / 2_f64.ln();
        big_n_float as i64
    };

    let mut t;
    let mut sum = 0_f64;

    let iter = prime::Primes::new(3_i64).take_while(|&a| a <= (2 * big_n) && a < MANTISA_MAX);
    for a in iter {
        let vmax = {
            let vmax_float = ((2 * big_n) as f64).ln() / (a as f64).ln();
            vmax_float as i64
        };
        let mut av = 1;
        for _ in (0..).take_while(|&i| i < vmax) {
            av *= a;
        }

        let mut s = 0;
        let mut num = 1;
        let mut den = 1;
        let mut v = 0;
        let mut kq = 1;
        let mut kq2 = 1;

        for k in (1..).take_while(|&k| k <= big_n) {
            t = k;
            if kq >= a {
                // do atleast once
                t /= a;
                v -= 1;
                while t.rem(a) == 0 {
                    t /= a;
                    v -= 1;
                }
                kq = 0;
            }
            kq += 1;
            num = mul_mod(num, t, av);

            t = 2 * k - 1;
            if kq2 >= a {
                if kq2 == a {
                    // do atleast once
                    t /= a;
                    v += 1;
                    while t.rem(a) == 0 {
                        t /= a;
                        v += 1;
                    }
                }
                kq2 -= a;
            }
            den = mul_mod(den, t, av);
            kq2 += 2;

            if v > 0 {
                t = mod_inverse(den, av);
                t = mul_mod(t, num, av);
                t = mul_mod(t, k, av);
                for _ in (v..).take_while(|&i| i < vmax) {
                    t = mul_mod(t, a, av);
                }
                s += t;
                if s >= av {
                    s -= av;
                }
            }
        }
        t = pow_mod(10, (n as i64) - 1, av);
        s = mul_mod(s, t, av);
        sum = (sum + (s as f64) / (av as f64)).rem(1_f64);
    }

    (sum * (10_f64.powi(EXP))) as i32
}

/// Returns the digits of pi, skipping the decimal, i.e. `31415`, generated
/// using the BBP algorithm.
#[derive(Default)]
pub struct BbpBellard<const BUF: i32> {
    n: i32,
    buffer: VecDeque<u8>,
}

impl<const BUF: i32> PiDigitGenerator for BbpBellard<BUF> {}

impl<const BUF: i32> BbpBellard<BUF> {
    pub fn new(start_position: i32) -> Self {
        let mut buffer = VecDeque::with_capacity((BUF - 1) as usize);
        let mut n = start_position;
        if start_position == 0 {
            buffer.push_back(3);
            n = 1;
        }

        Self { n, buffer }
    }

    pub fn get_n_position(n: i32) -> i32 {
        if n == 0 {
            3
        } else {
            get_next_n_dec_digits::<1>(n)
        }
    }
}

impl<const BUF: i32> Iterator for BbpBellard<BUF> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            self.buffer.pop_front()
        } else {
            let mut num = get_next_n_dec_digits::<BUF>(self.n);
            let mut divisor = 1;
            while num >= divisor * 10 {
                divisor *= 10;
            }

            let mut digits = std::iter::from_fn(move || {
                if divisor == 0 {
                    None
                } else {
                    let v = num / divisor;
                    num %= divisor;
                    divisor /= 10;
                    Some(v as u8)
                }
            });

            let first = digits.next();
            for digit in digits {
                self.buffer.push_back(digit);
            }
            self.n += BUF + 1;

            first
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::*;

    #[test]
    fn should_generate_equivalent_pi_value_to_stdlib_constant_using_spigot() {
        let pi_const = consts::PI.to_string();
        let pi_generated = super::Spigot::new();

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

    #[test]
    fn should_generate_n_digit_of_pi_with_bbp_algorithm() {
        let expected = [9, 2, 1, 6, 4, 2, 0, 1, 9].into_iter();

        // buffer 9 elements starting at position 990 of  pi
        let generator = BbpBellard::<9>::new(990);

        for (idx, (expected, got)) in expected
            .zip(generator)
            .enumerate()
            .map(|(idx, data)| (idx + 990, data))
        {
            assert_eq!(expected, got, "failed at {}, wanted", idx);
        }
    }
}
