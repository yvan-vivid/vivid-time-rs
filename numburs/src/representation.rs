use core::cmp::max;

use derive_more::From;
use num_traits::{
    clamp,
    identities::{ConstOne, ConstZero},
    Float, PrimInt,
};

pub trait NumBase: PrimInt + ConstOne + ConstZero {}
impl<N: PrimInt + ConstZero + ConstOne> NumBase for N {}

pub trait FloatBase: Float + ConstOne + ConstZero {}
impl<F: Float + ConstZero + ConstOne> FloatBase for F {}

#[derive(Debug, From, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Zero;

#[derive(Debug, From, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct One;

pub trait ConstrainedRep<N: Copy>: From<N> {
    fn can_rep(x: &N) -> bool;
    fn out(&self) -> N;

    fn rep(x: N) -> Option<Self> {
        Some(x).filter(Self::can_rep).map(Self::from)
    }
}

pub trait LowerBoundedRep<N: Ord>: From<N> {
    fn under_bottom() -> N;

    fn bottom() -> Self {
        Self::from(Self::under_bottom())
    }

    fn at_least(x: N) -> Self {
        Self::from(max(x, Self::under_bottom()))
    }
}

pub trait BoundedRepLeft<N: Ord>: From<N> {
    fn under_bottom() -> N;
    fn under_top() -> N;

    fn bottom() -> Self {
        Self::from(Self::under_bottom())
    }

    fn clamped(x: N) -> Self {
        Self::from(clamp(x, Self::under_bottom(), Self::under_top()))
    }
}

pub trait HasZero<N: ConstZero> {
    const ZERO: Self;
}

pub trait HasOne<N: ConstOne> {
    const ONE: Self;
}

#[macro_export]
macro_rules! has_zero {
    ($x:ident) => {
        impl<N: NumBase> HasZero<N> for $x<N> {
            const ZERO: $x<N> = $x(N::ZERO);
        }

        impl<N: NumBase> From<Zero> for $x<N> {
            fn from(_value: Zero) -> Self {
                $x::ZERO
            }
        }
    };
}

#[macro_export]
macro_rules! has_one {
    ($x:ident) => {
        impl<N: NumBase> HasOne<N> for $x<N> {
            const ONE: $x<N> = $x(N::ONE);
        }

        impl<N: NumBase> From<One> for $x<N> {
            fn from(_value: One) -> Self {
                $x::ONE
            }
        }
    };
}
