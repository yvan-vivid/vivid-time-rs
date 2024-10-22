use core::fmt::Debug;
use std::marker::PhantomData;

use numburs::{FloatBase, Integer, Natural, NumBase, Positive, Real, WithFraction};

use super::types::{BoundCyclePoint, Cycle, CyclePoint};

pub trait PeriodRepresentation {
    type Phase: NumBase;
    type Cycles: NumBase;

    fn embed_positive(u: Positive<Self::Phase>) -> Positive<Self::Cycles>;
    fn embed(u: Natural<Self::Phase>) -> Natural<Self::Cycles>;
    fn project(l: Natural<Self::Cycles>) -> Natural<Self::Phase>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Period<R: PeriodRepresentation> {
    size: Positive<R::Phase>,
    representation: PhantomData<R>,
}

impl<R: PeriodRepresentation> Period<R> {
    pub fn new(size: Positive<R::Phase>) -> Self {
        Self {
            size,
            representation: PhantomData,
        }
    }
}

impl<R: PeriodRepresentation> Cycle<R::Phase> for Period<R> {
    type Cycles = Integer<R::Cycles>;

    fn size(&self) -> Positive<R::Phase> {
        self.size
    }

    fn is_norm(&self, point: &CyclePoint<R::Phase, Self::Cycles>) -> bool {
        point.phase < self.size.into()
    }

    fn wind_inner(&self, cycles: &Self::Cycles) -> CyclePoint<R::Phase, Self::Cycles> {
        let size_l = R::embed_positive(self.size);
        let (cycle, phase) = size_l.euclid(*cycles);
        CyclePoint::new(cycle, R::project(phase))
    }

    fn unwind(&self, point: &CyclePoint<R::Phase, Self::Cycles>) -> Self::Cycles {
        let CyclePoint { cycle, phase } = point;
        let size: Self::Cycles = R::embed_positive(self.size()).into();
        let c_phase: Self::Cycles = R::embed(*phase).into();
        (*cycle) * size + c_phase
    }
}

impl<R: PeriodRepresentation> Period<R> {
    pub fn wind_with_float<F: FloatBase>(&self, r: Real<F>) -> WithFraction<BoundCyclePoint<R::Phase, Self>, F> {
        let WithFraction { whole, fraction } = r.fractionalize();
        WithFraction::new(self.wind(&whole), fraction)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IdentityPeriodRepresentation<U: NumBase> {
    phantom: PhantomData<U>,
}

impl<U: NumBase> PeriodRepresentation for IdentityPeriodRepresentation<U> {
    type Cycles = U;
    type Phase = U;

    fn embed_positive(u: Positive<Self::Phase>) -> Positive<Self::Cycles> {
        u
    }

    fn embed(u: Natural<Self::Phase>) -> Natural<Self::Cycles> {
        u
    }

    fn project(l: Natural<Self::Cycles>) -> Natural<Self::Phase> {
        l
    }
}

#[cfg(test)]
mod tests {
    use numburs::{
        int, integral::testing::*, nat, pos, Fractional, Integer, LowerBoundedRep, Natural, Positive, WithFraction,
    };

    use super::{Cycle, IdentityPeriodRepresentation, Period};

    #[test]
    fn test_period_construction() {
        let period: Period<IdentityPeriodRepresentation<i64>> = Period::new(P5);
        assert_eq!(period.point(5.into(), N4), Some(period.wind(&int!(29))));
    }

    #[test]
    fn test_period_positive() {
        for p in 1..10 {
            let period: Period<IdentityPeriodRepresentation<i64>> = Period::new(pos!(p));
            let mut q = 0;
            let mut r = 0;
            for k in 0..20 {
                assert_eq!(period.point(q.into(), nat!(r)), Some(period.wind(&k.into())));
                r += 1;
                if r == p {
                    q += 1;
                    r = 0;
                }
            }
        }
    }

    #[test]
    fn test_period_float_positive() {
        // TODO: Tests with fractionals, negative, etc...
        let period: Period<IdentityPeriodRepresentation<i64>> = Period::new(P1);
        assert_eq!(
            WithFraction::new(period.point(1.into(), N0).unwrap(), Fractional::from(0.)),
            period.wind_with_float((1.0).into())
        );
    }

    #[test]
    fn test_period_negative() {
        for p in 1..10 {
            let period: Period<IdentityPeriodRepresentation<i64>> = Period::new(pos!(p));
            let mut q = 0;
            let mut r = 0;
            for k in 0..20 {
                assert_eq!(period.point(q.into(), nat!(r)), Some(period.wind(&(-k).into())));
                if r == 0 {
                    q -= 1;
                    r = p;
                }
                r -= 1;
            }
        }
    }
}
