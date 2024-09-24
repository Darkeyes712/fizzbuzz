use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Rem;

pub struct Matcher<T> {
    condition: Box<dyn Fn(T) -> bool>,
    substitution: String,
    _phantom: PhantomData<T>,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: 'static + Fn(T) -> bool,
        S: Into<String>,
    {
        Matcher {
            condition: Box::new(matcher),
            substitution: subs.into(),
            _phantom: PhantomData,
        }
    }

    pub fn check(&self, value: T) -> Option<String> {
        if (self.condition)(value) {
            Some(self.substitution.clone())
        } else {
            None
        }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
    _phantom: PhantomData<T>,
}

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
        T: Clone + Display,
    {
        iter.map(move |val| {
            let mut result = String::new();
            for matcher in &self.matchers {
                if let Some(substitution) = matcher.check(val.clone()) {
                    result.push_str(&substitution);
                }
            }
            if result.is_empty() {
                val.to_string()
            } else {
                result
            }
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Copy + Rem<Output = T> + From<u8> + PartialEq + Display,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % T::from(5) == T::from(0), "buzz"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let actual = fizz_buzz::<i32>().apply(1..=16).collect::<Vec<_>>();
        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn u8() {
        let actual = fizz_buzz::<u8>().apply(1_u8..=16).collect::<Vec<_>>();
        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn u64() {
        let actual = fizz_buzz::<u64>().apply(1_u64..=16).collect::<Vec<_>>();
        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn nonsequential() {
        let collatz_12 = &[12, 6, 3, 10, 5, 16, 8, 4, 2, 1];
        let actual = fizz_buzz::<i32>()
            .apply(collatz_12.iter().cloned())
            .collect::<Vec<_>>();
        let expected = vec![
            "fizz", "fizz", "fizz", "buzz", "buzz", "16", "8", "4", "2", "1",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn custom() {
        let expected = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "Bam", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "Bam", "BuzzFizz", "16",
        ];
        let fizzer: Fizzy<i32> = Fizzy::new()
            .add_matcher(Matcher::new(|n: i32| n % 5 == 0, "Buzz"))
            .add_matcher(Matcher::new(|n: i32| n % 3 == 0, "Fizz"))
            .add_matcher(Matcher::new(|n: i32| n % 7 == 0, "Bam"));
        let actual = fizzer.apply(1..=16).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn f64() {
        let actual = fizz_buzz::<f64>()
            .apply(std::iter::successors(Some(1.0), |prev| Some(prev + 1.0)))
            .take(16)
            .collect::<Vec<_>>();
        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn minimal_generic_bounds() {
        use std::fmt;
        use std::ops::{Add, Rem};

        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        struct Fizzable(u8);

        impl From<u8> for Fizzable {
            fn from(i: u8) -> Fizzable {
                Fizzable(i)
            }
        }

        impl fmt::Display for Fizzable {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let Fizzable(ref n) = self;
                write!(f, "{n}")
            }
        }

        impl Add for Fizzable {
            type Output = Fizzable;
            fn add(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;
                let Fizzable(n2) = rhs;
                Fizzable(n1 + n2)
            }
        }

        impl Rem for Fizzable {
            type Output = Fizzable;
            fn rem(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;
                let Fizzable(n2) = rhs;
                Fizzable(n1 % n2)
            }
        }

        let actual = fizz_buzz::<Fizzable>()
            .apply(std::iter::successors(Some(Fizzable(1)), |prev| {
                Some(*prev + 1.into())
            }))
            .take(16)
            .collect::<Vec<_>>();
        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        assert_eq!(actual, expected);
    }
}
