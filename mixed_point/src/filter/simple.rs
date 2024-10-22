use derive_more::Constructor;
use numburs::{HasZero, Integer, Natural};

use super::types::{Filter, FilterPoint};
use crate::{
    cycle::{
        period::{Period, PeriodRepresentation},
        types::{Cycle, CyclePoint},
    },
    types::Factors,
    MixedPoint,
};

#[derive(PartialEq, Eq, Constructor)]
pub struct SimpleFilter<R: PeriodRepresentation, C: Cycle<R::Phase, Cycles = Natural<R::Phase>>, const N: usize> {
    pub period: Period<R>,
    pub factors: Factors<C, N>,
}

impl<R, const N: usize, C> Filter<N> for SimpleFilter<R, C, N>
where
    R: PeriodRepresentation,
    C: Cycle<R::Phase, Cycles = Natural<R::Phase>>,
{
    type Cycle = Integer<R::Cycles>;
    type U = R::Phase;

    fn is_norm(&self, _point: &FilterPoint<Self::U, N, Self::Cycle>) -> bool {
        // TODO: implement
        true
    }

    fn wind_inner(&self, total: &Self::Cycle) -> FilterPoint<Self::U, N, Self::Cycle> {
        let CyclePoint { cycle, mut phase } = self.period.wind(total).point;
        let mut phases = [Natural::ZERO; N];
        for k in 0..N {
            let point = self.factors[k].wind(&phase).point;
            phases[N - k - 1] = point.cycle;
            phase = point.phase;
        }

        FilterPoint::new(MixedPoint::new(cycle, phases), phase)
    }
}

#[cfg(test)]
mod tests {
    use numburs::{int, nat, pos, Integer, LowerBoundedRep, Natural, Positive};

    use super::SimpleFilter;
    use crate::{
        cycle::{limited_cycle::LimitedCycle, period::Period, simple_cycle::SimpleCycle},
        filter::types::{Filter, FilterPoint},
        IdentityPeriodRepresentation, MixedPoint,
    };

    fn filter_point(c: i32, p0: i32, p1: i32, r: i32) -> FilterPoint<i32, 2, Integer<i32>> {
        FilterPoint::new(MixedPoint::new(int!(c), [nat!(p0), nat!(p1)]), nat!(r))
    }

    #[test]
    fn example_filter_scheme() {
        let filter = SimpleFilter::new(
            Period::<IdentityPeriodRepresentation<i32>>::new(pos!(12)),
            [SimpleCycle::new(pos!(6)), SimpleCycle::new(pos!(2))],
        );
        let point = |k: i32| filter.wind(int!(k)).point;
        assert_eq!(filter_point(0, 0, 0, 0), point(0));
        assert_eq!(filter_point(0, 0, 0, 1), point(1));
        assert_eq!(filter_point(0, 1, 0, 0), point(2));
        assert_eq!(filter_point(0, 1, 0, 1), point(3));
        assert_eq!(filter_point(0, 2, 0, 0), point(4));
        assert_eq!(filter_point(0, 2, 0, 1), point(5));
        assert_eq!(filter_point(0, 0, 1, 0), point(6));
        assert_eq!(filter_point(0, 0, 1, 1), point(7));
        assert_eq!(filter_point(3, 0, 1, 1), point(43));
        assert_eq!(filter_point(3, 1, 1, 1), point(45));
        assert_eq!(filter_point(-1, 2, 1, 1), point(-1));
    }

    #[test]
    fn example_limit_filter_scheme() {
        let filter = SimpleFilter::new(
            Period::<IdentityPeriodRepresentation<i32>>::new(pos!(50)),
            [
                LimitedCycle::new(pos!(10), pos!(3)),
                LimitedCycle::new(pos!(3), pos!(2)),
            ],
        );
        let point = |k: i32| filter.wind(int!(k)).point;
        assert_eq!(filter_point(0, 0, 0, 0), point(0));
        assert_eq!(filter_point(0, 0, 0, 1), point(1));
        assert_eq!(filter_point(0, 2, 0, 2), point(8));
        assert_eq!(filter_point(0, 2, 0, 3), point(9));
        assert_eq!(filter_point(0, 0, 1, 0), point(10));
        assert_eq!(filter_point(0, 1, 2, 2), point(25));
        assert_eq!(filter_point(0, 2, 2, 3), point(29));
        assert_eq!(filter_point(0, 2, 3, 13), point(49));
        assert_eq!(filter_point(1, 0, 0, 0), point(50));
        assert_eq!(filter_point(-1, 2, 3, 13), point(-1));
        assert_eq!(filter_point(-1, 2, 3, 1), point(-13));
        assert_eq!(filter_point(-1, 2, 3, 0), point(-14));
        assert_eq!(filter_point(-1, 1, 3, 2), point(-15));
    }
}
