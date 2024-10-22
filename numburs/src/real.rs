use derive_more::{Add, AddAssign, Constructor, Div, From, Mul, Neg, Rem, Sub};
use serde::Serialize;

use super::{
    integral::Integer,
    representation::{FloatBase, NumBase},
};
use crate::ConstrainedRep;

#[derive(Debug, Mul, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, From, Serialize)]
pub struct Fractional<F: FloatBase>(F);

#[derive(
    Debug, Add, AddAssign, Sub, Mul, Div, Neg, Rem, From, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Serialize,
)]
pub struct Real<F: FloatBase>(F);

#[derive(Debug, Constructor, PartialEq, Serialize, Clone)]
pub struct WithFraction<X, F: FloatBase> {
    pub whole: X,
    pub fraction: Fractional<F>,
}

impl<F: FloatBase> ConstrainedRep<F> for Fractional<F> {
    fn can_rep(x: &F) -> bool {
        F::ZERO <= *x && *x < F::ONE
    }

    fn out(&self) -> F {
        self.0
    }
}

impl<F: FloatBase> ConstrainedRep<F> for Real<F> {
    fn can_rep(_: &F) -> bool {
        true
    }

    fn out(&self) -> F {
        self.0
    }
}

impl<F: FloatBase> Real<F> {
    pub fn fractionalize<N: NumBase>(self) -> WithFraction<Integer<N>, F> {
        let w = self.0.floor();
        WithFraction::new(N::from(w).unwrap_or(N::zero()).into(), Fractional(self.0 - w))
    }
}

pub mod testing {
    use paste::paste;

    use super::{Fractional, Real};

    macro_rules! generate_real_constants {
        ([$x:expr], [$($y:expr),*]) => {
            $(
                paste!{ pub const [<R $x _ $y>]: Real<f32> = Real(($x*10 + $y) as f32 / 10.); }
            )*
        };

        ([$($x:expr),*], $ys:tt) => {
            $(
                generate_real_constants!([$x], $ys);
            )*
        };
    }
    generate_real_constants!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    macro_rules! generate_fractional_constants {
        ($($val:expr),+) => {
            $(
                paste!{ pub const [<F_ $val>]: Fractional<f32> = Fractional($val as f32 / 10.); }
            )+
        };
    }
    generate_fractional_constants!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use assert_approx_eq::assert_approx_eq;

    use super::testing::*;
    use crate::{integral::testing::*, WithFraction};

    fn assert_with_fraction_eq<X: Eq + Debug>(l: WithFraction<X, f32>, r: WithFraction<X, f32>) {
        assert_eq!(l.whole, r.whole);
        assert_approx_eq!(l.fraction.0, r.fraction.0);
    }

    #[test]
    fn test_real_fractionalize_positive() {
        assert_with_fraction_eq(WithFraction::new(I0, F_0), R0_0.fractionalize());
        assert_with_fraction_eq(WithFraction::new(I0, F_5), R0_5.fractionalize());
        assert_with_fraction_eq(WithFraction::new(I5, F_0), R5_0.fractionalize());
        assert_with_fraction_eq(WithFraction::new(I5, F_5), R5_5.fractionalize());
    }

    #[test]
    fn test_real_fractionalize_negative() {
        assert_with_fraction_eq(WithFraction::new(-I1, F_9), (-R0_1).fractionalize());
        assert_with_fraction_eq(WithFraction::new(-I1, F_5), (-R0_5).fractionalize());
        assert_with_fraction_eq(WithFraction::new(-I5, F_0), (-R5_0).fractionalize());
        assert_with_fraction_eq(WithFraction::new(-I6, F_9), (-R5_1).fractionalize());
        assert_with_fraction_eq(WithFraction::new(-I6, F_5), (-R5_5).fractionalize());
    }
}
