use core::fmt::Debug;

use derive_more::Constructor;
use numburs::{HasOne, HasZero, Integer, Natural, Positive};

use super::types::{Mixed, MixedPoint};
use crate::cycle::{
    period::{Period, PeriodRepresentation},
    simple_cycle::SimpleCycle,
    types::{Cycle, CyclePoint},
};

#[derive(PartialEq, Eq, Constructor, Clone)]
pub struct SimpleMixed<R: PeriodRepresentation, C: Cycle<R::Phase, Cycles = Natural<R::Phase>>, const N: usize> {
    pub period: Period<R>,
    pub factors: [C; N],
}

impl<R, C, const N: usize> Debug for SimpleMixed<R, C, N>
where
    R: PeriodRepresentation + Debug,
    R::Phase: Debug,
    C: Cycle<R::Phase, Cycles = Natural<R::Phase>> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleMixed")
            .field("period", &self.period)
            .field("factors", &self.factors)
            .finish()
    }
}

impl<R, C, const N: usize> SimpleMixed<R, C, N>
where
    R: PeriodRepresentation,
    C: Cycle<R::Phase, Cycles = Natural<R::Phase>>,
{
    pub fn from_factors(factors: [C; N]) -> Self {
        let mut prod = Positive::ONE;
        for c in factors.as_slice() {
            prod = prod * c.size();
        }
        Self::new(Period::new(prod), factors)
    }

    pub fn num_factors(&self) -> usize {
        self.factors.len()
    }
}

impl<R, const N: usize> SimpleMixed<R, SimpleCycle<R::Phase>, N>
where
    R: PeriodRepresentation,
{
    pub fn from_simple_factors(sizes: [Positive<R::Phase>; N]) -> Self {
        Self::from_factors(sizes.map(SimpleCycle::new))
    }
}

impl<R, const N: usize, C> Mixed<N> for SimpleMixed<R, C, N>
where
    R: PeriodRepresentation,
    C: Cycle<R::Phase, Cycles = Natural<R::Phase>>,
{
    type Cycles = Integer<R::Cycles>;
    type U = R::Phase;

    fn is_norm(&self, point: &MixedPoint<Self::U, N, Self::Cycles>) -> bool {
        for k in 0..N {
            if Natural::from(self.factors[k].size()) <= point.phase[k] {
                return false;
            }
        }
        true
    }

    fn wind_inner(&self, total: &Self::Cycles) -> MixedPoint<Self::U, N, Self::Cycles> {
        let CyclePoint { cycle, mut phase } = self.period.wind(total).point;
        let mut phases = [Natural::ZERO; N];
        for (k, factor) in self.factors.iter().enumerate() {
            let point = factor.wind(&phase).point;
            phases[k] = point.phase;
            phase = point.cycle;
        }

        MixedPoint::new(cycle, phases)
    }

    fn unwind(&self, point: &MixedPoint<Self::U, N, Self::Cycles>) -> Self::Cycles {
        let MixedPoint { cycle, phase } = point;
        let mut total = *cycle;
        for k in (0..N).rev() {
            total = Period::<R>::new(self.factors[k].size()).unwind(&CyclePoint::new(total, phase[k]));
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use numburs::{integral::testing::*, Integer};

    use super::SimpleMixed;
    use crate::{
        cycle::{period::IdentityPeriodRepresentation, simple_cycle::SimpleCycle},
        mixed::types::{BoundMixedPoint, Mixed, MixedPoint},
    };

    static MIXED: LazyLock<SimpleMixed<IdentityPeriodRepresentation<i64>, SimpleCycle<i64>, 2>> =
        LazyLock::new(|| SimpleMixed::from_factors([SimpleCycle::new(P2), SimpleCycle::new(P3)]));

    const I11: Integer<i64> = Integer::new(11);
    const I16: Integer<i64> = Integer::new(16);

    #[test]
    fn example_point_scheme() {
        assert_eq!(MixedPoint::new(I0, [N0, N0]), MIXED.wind(I0).point);
        assert_eq!(MixedPoint::new(I0, [N1, N0]), MIXED.wind(I1).point);
        assert_eq!(MixedPoint::new(I0, [N0, N1]), MIXED.wind(I2).point);
        assert_eq!(MixedPoint::new(I1, [N1, N0]), MIXED.wind(I7).point);
        assert_eq!(MixedPoint::new(I1, [N1, N2]), MIXED.wind(I11).point);
    }

    #[test]
    fn simple_mixed_bind() {
        let good_point = MixedPoint::new(I1, [N1, N2]);
        let bad_point = MixedPoint::new(I1, [N2, N2]);
        assert_eq!(
            Some(BoundMixedPoint::new(&*MIXED, good_point.clone())),
            MIXED.bind(good_point)
        );
        assert_eq!(None, MIXED.bind(bad_point));
    }

    #[test]
    fn simple_mixed_unwind() {
        assert_eq!(I0, MIXED.unwind(&MixedPoint::new(I0, [N0, N0])));
        assert_eq!(I1, MIXED.unwind(&MixedPoint::new(I0, [N1, N0])));
        assert_eq!(I2, MIXED.unwind(&MixedPoint::new(I0, [N0, N1])));
        assert_eq!(I16, MIXED.unwind(&MixedPoint::new(I2, [N0, N2])));
        assert_eq!(I11, MIXED.unwind(&MixedPoint::new(I1, [N1, N2])));
        assert_eq!(-I1, MIXED.unwind(&MixedPoint::new(-I1, [N1, N2])));
        assert_eq!(-I3, MIXED.unwind(&MixedPoint::new(-I1, [N1, N1])));
    }
}
