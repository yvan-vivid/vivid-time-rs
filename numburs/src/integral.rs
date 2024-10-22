use core::ops::{Add, Div, Mul, Rem};
use std::fmt::Display;

use derive_more::{From, Neg};
use serde::Serialize;

use super::representation::{ConstrainedRep, HasOne, HasZero, LowerBoundedRep, NumBase, One, Zero};
use crate::{has_one, has_zero};

#[derive(Debug, From, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Serialize, Neg)]
pub struct Integer<N: NumBase>(N);

#[derive(Debug, From, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Serialize)]
pub struct Natural<N: NumBase>(N);

#[derive(Debug, From, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Serialize)]
pub struct Positive<N: NumBase>(N);

#[derive(Debug, From, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Serialize)]
pub struct Many<N: NumBase>(N);

impl<N: NumBase> Integer<N> {
    pub const fn new(n: N) -> Self {
        Self(n)
    }
}

impl<N: NumBase> ConstrainedRep<N> for Integer<N> {
    fn can_rep(_: &N) -> bool {
        true
    }

    fn out(&self) -> N {
        self.0
    }
}

impl<N: NumBase> LowerBoundedRep<N> for Natural<N> {
    fn under_bottom() -> N {
        N::ZERO
    }
}

impl<N: NumBase> ConstrainedRep<N> for Natural<N> {
    fn can_rep(x: &N) -> bool {
        Self::under_bottom() <= *x
    }

    fn out(&self) -> N {
        self.0
    }
}

impl<N: NumBase> LowerBoundedRep<N> for Positive<N> {
    fn under_bottom() -> N {
        N::ONE
    }
}

impl<N: NumBase> ConstrainedRep<N> for Positive<N> {
    fn can_rep(x: &N) -> bool {
        Self::under_bottom() <= *x
    }

    fn out(&self) -> N {
        self.0
    }
}

impl<N: NumBase> ConstrainedRep<N> for Many<N> {
    fn can_rep(x: &N) -> bool {
        *x > N::ONE
    }

    fn out(&self) -> N {
        self.0
    }
}

macro_rules! does_map {
    ($x:ident) => {
        impl<U: NumBase> $x<U> {
            pub fn map<L: NumBase, F: FnOnce(U) -> L>(&self, f: F) -> $x<L> {
                $x::from(f(self.0))
            }
        }
    };
}

macro_rules! embedding {
    ($x:ident, $y:ident) => {
        impl<N: NumBase> From<$x<N>> for $y<N> {
            fn from(value: $x<N>) -> Self {
                $y(value.0)
            }
        }
    };
}

macro_rules! constrained_add {
    ($x:ident, $y:ident, $z:ident) => {
        impl<N: NumBase> Add<$y<N>> for $x<N> {
            type Output = $z<N>;

            fn add(self, rhs: $y<N>) -> Self::Output {
                $z(self.0 + rhs.0)
            }
        }
    };
}

macro_rules! constrained_add_sym {
    ($x:ident, $y:ident, $z:ident) => {
        constrained_add!($x, $y, $z);
        constrained_add!($y, $x, $z);
    };
}

macro_rules! constrained_right_inc {
    ($x:ident, $z:ident) => {
        impl<N: NumBase> Add<One> for $x<N> {
            type Output = $z<N>;

            fn add(self, _rhs: One) -> Self::Output {
                $z(self.0 + N::ONE)
            }
        }
    };
}

macro_rules! constrained_left_inc {
    ($y:ident, $z:ident) => {
        impl<N: NumBase> Add<$y<N>> for One {
            type Output = $z<N>;

            fn add(self, rhs: $y<N>) -> Self::Output {
                $z(N::ONE + rhs.0)
            }
        }
    };
}

macro_rules! constrained_inc {
    ($x:ident, $z:ident) => {
        constrained_right_inc!($x, $z);
        constrained_left_inc!($x, $z);
    };
}

macro_rules! constrained_mul {
    ($x:ident, $y:ident, $z:ident) => {
        impl<N: NumBase> Mul<$y<N>> for $x<N> {
            type Output = $z<N>;

            fn mul(self, rhs: $y<N>) -> Self::Output {
                $z(self.0 * rhs.0)
            }
        }
    };
}

macro_rules! constrained_mul_sym {
    ($x:ident, $y:ident, $z:ident) => {
        constrained_mul!($x, $y, $z);
        constrained_mul!($y, $x, $z);
    };
}

macro_rules! constrained_div {
    ($x:ident, $y:ident, $z:ident) => {
        impl<N: NumBase> Div<$y<N>> for $x<N> {
            type Output = $z<N>;

            fn div(self, rhs: $y<N>) -> Self::Output {
                $z(self.0 / rhs.0)
            }
        }

        impl<N: NumBase> Rem<$y<N>> for $x<N> {
            type Output = Self;

            fn rem(self, rhs: $y<N>) -> Self::Output {
                $z(self.0 % rhs.0)
            }
        }
    };
}

macro_rules! display_with {
    ($x:ident, $y:expr) => {
        impl<N: NumBase + Display> Display for $x<N> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", $y, self.0)
            }
        }
    };
}

display_with!(Integer, "I");
display_with!(Natural, "N");
display_with!(Positive, "P");
display_with!(Many, "M");

has_zero!(Integer);
has_zero!(Natural);
has_one!(Integer);
has_one!(Natural);
has_one!(Positive);

does_map!(Integer);

embedding!(Many, Positive);
embedding!(Many, Natural);
embedding!(Many, Integer);
embedding!(Positive, Natural);
embedding!(Positive, Integer);
embedding!(Natural, Integer);

constrained_add!(Integer, Integer, Integer);
constrained_add_sym!(Integer, Natural, Integer);
constrained_add_sym!(Integer, Positive, Integer);
constrained_add_sym!(Integer, Many, Integer);
constrained_add!(Natural, Natural, Natural);
constrained_add_sym!(Natural, Positive, Positive);
constrained_add_sym!(Natural, Many, Many);
constrained_add!(Positive, Positive, Many);
constrained_add_sym!(Positive, Many, Many);
constrained_add!(Many, Many, Many);

constrained_inc!(Natural, Positive);
constrained_inc!(Positive, Many);
constrained_inc!(Many, Many);

constrained_mul!(Integer, Integer, Integer);
constrained_mul_sym!(Integer, Natural, Integer);
constrained_mul_sym!(Integer, Positive, Integer);
constrained_mul_sym!(Integer, Many, Integer);
constrained_mul!(Natural, Natural, Natural);
constrained_mul_sym!(Natural, Positive, Natural);
constrained_mul_sym!(Natural, Many, Natural);
constrained_mul!(Positive, Positive, Positive);
constrained_mul_sym!(Positive, Many, Many);
constrained_mul!(Many, Many, Many);

constrained_div!(Natural, Positive, Natural);
constrained_div!(Natural, Many, Natural);

impl<N: NumBase> Positive<N> {
    pub fn euclid(self, x: Integer<N>) -> (Integer<N>, Natural<N>) {
        let q = (x.0 - (if x.0 < N::ZERO { self.0 - N::ONE } else { N::ZERO })) / self.0;
        (Integer(q), Natural(x.0 - q * self.0))
    }
}

#[macro_export]
macro_rules! int {
    ($x:expr) => {
        Integer::new($x)
    };
}

#[macro_export]
macro_rules! nat {
    ($x:expr) => {
        Natural::at_least($x)
    };
}

#[macro_export]
macro_rules! pos {
    ($x:expr) => {
        Positive::at_least($x)
    };
}

pub mod testing {
    use paste::paste;

    use super::{Integer, Many, Natural, Positive};

    macro_rules! generate_constants {
        ($p:ident, $type:ident, $($val:expr),+) => {
            $(
                paste!{ pub const [<$p $val>]: $type<i64> = $type($val); }
            )+
        };
    }

    generate_constants!(I, Integer, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    generate_constants!(N, Natural, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    generate_constants!(P, Positive, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    generate_constants!(M, Many, 2, 3, 4, 5, 6, 7, 8, 9, 10);
}

#[cfg(test)]
mod tests {
    use super::{testing::*, Integer, LowerBoundedRep, Natural, One, Positive, Zero};

    #[test]
    fn embedding() {
        assert_eq!(I2, N2.into());
        assert_eq!(I2, P2.into());
        assert_eq!(I2, M2.into());
        assert_eq!(N2, P2.into());
        assert_eq!(N2, M2.into());
        assert_eq!(pos!(2), M2.into());
    }

    #[test]
    fn embedding_constants() {
        assert_eq!(I0, Zero.into());
        assert_eq!(N0, Zero.into());
        assert_eq!(I1, One.into());
        assert_eq!(N1, One.into());
        assert_eq!(P1, One.into());
    }

    #[test]
    fn add_positives() {
        assert_eq!(P5, (P2 + P3).into());
    }

    #[test]
    fn mul_positives() {
        assert_eq!(P6, P2 * P3);
    }

    #[test]
    fn does_map() {
        let x: i32 = 5;
        let y: i64 = 5;
        assert_eq!(int!(y), int!(x).map(i64::from));
        assert_eq!(int!(x), int!(y).map(|v| v as i32));
    }

    #[test]
    fn test_positive_euclid() {
        for p in 1..10 {
            let positive = pos!(p);
            let mut q = 0;
            let mut r = 0;
            for k in 0..20 {
                assert_eq!((q.into(), nat!(r)), positive.euclid(k.into()));
                r += 1;
                if r == p {
                    q += 1;
                    r = 0;
                }
            }
        }
    }

    #[test]
    fn test_negative_euclid() {
        for p in 1..10 {
            let positive = pos!(p);
            let mut q = 0;
            let mut r = 0;
            for k in 0..20 {
                assert_eq!((q.into(), nat!(r)), positive.euclid((-k).into()));
                if r == 0 {
                    q -= 1;
                    r = p;
                }
                r -= 1;
            }
        }
    }
}
