use num::integer::Roots;

pub(crate) trait MaybePrime {
    fn is_prime(&self) -> bool;
}

impl MaybePrime for i64 {
    fn is_prime(&self) -> bool {
        let n = *self;

        if n == 2 || n == 3 {
            return true;
        } else if n.rem_euclid(2) == 0 || n.rem_euclid(3) == 0 || n < 2 {
            return false;
        };

        let sqrt = n.sqrt() + 1;

        let range = (6..).step_by(6).take_while(|val| val <= &sqrt);
        for i in range {
            if (n.rem_euclid(i - 1) == 0) || (n.rem_euclid(i + 1) == 0) {
                return false;
            }
        }

        true
    }
}

impl MaybePrime for i32 {
    fn is_prime(&self) -> bool {
        let n = *self;

        if n == 2 || n == 3 {
            return true;
        } else if n.rem_euclid(2) == 0 || n.rem_euclid(3) == 0 || n < 2 {
            return false;
        };

        let sqrt = n.sqrt() + 1;

        let range = (6..).step_by(6).take_while(|val| val <= &sqrt);
        for i in range {
            if (n.rem_euclid(i - 1) == 0) || (n.rem_euclid(i + 1) == 0) {
                return false;
            }
        }

        true
    }
}

pub(crate) struct Primes<T>
where
    T: MaybePrime + PartialOrd<T> + std::ops::AddAssign,
{
    base: T,
}

impl<T> Primes<T>
where
    T: MaybePrime + PartialOrd<T> + std::ops::AddAssign,
{
    pub(crate) fn new(base: T) -> Self {
        Self { base }
    }
}

impl std::iter::Iterator for Primes<i64> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.base;
        let primes = if n < 2 {
            Some(2)
        } else {
            (n..).find(MaybePrime::is_prime)
        }?;

        self.base = primes + 1;
        Some(primes)
    }
}

impl std::iter::Iterator for Primes<i32> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.base;
        let primes = if n < 2 {
            Some(2)
        } else {
            (n..).find(MaybePrime::is_prime)
        }?;

        self.base = primes + 1;
        Some(primes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_on_primes() {
        for value in [0, 1, 728] {
            assert!(!value.is_prime(), "{} is not a prime number", value);
        }

        for value in [2, 3, 5, 7, 11, 13, 17, 19, 23, 727] {
            assert!(value.is_prime(), "{} is a prime number", value);
        }
    }

    #[test]
    fn should_find_next_prime() {
        let first_five = Primes::new(1).take(5).collect::<Vec<_>>();
        assert_eq!(&first_five, &[2, 3, 5, 7, 11]);

        assert_eq!(Primes::new(728).next(), Some(733))
    }
}
