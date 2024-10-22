use std::sync::LazyLock;

use derive_more::{Constructor, From, Into};
use mixed_point::{
    BoundMixedPoint, Filter, IdentityPeriodRepresentation, LimitedCycle, Mixed, MixedPointLegend, Period, SimpleCycle,
    SimpleFilter, SimpleMixed,
};
use numburs::{pos, LowerBoundedRep, Positive};

use super::units::{Day, Year, YearDay};
use crate::base::I;

pub type DepthType = SimpleMixed<IdentityPeriodRepresentation<I>, SimpleCycle<I>, 3>;
pub type DepthDaysType = SimpleFilter<IdentityPeriodRepresentation<I>, LimitedCycle<I>, 3>;
pub type DepthPoint = BoundMixedPoint<'static, 3, DepthType>;

pub static DEPTH_YEARS: LazyLock<DepthType> =
    LazyLock::new(|| SimpleMixed::from_simple_factors([pos!(8), pos!(2), pos!(16)]));

pub static DEPTH_LEGEND: LazyLock<MixedPointLegend<'static, 3>> =
    LazyLock::new(|| MixedPointLegend::with_names("aeon", ["unade", "octade", "hexade"]));

pub static DEPTH_DAYS: LazyLock<DepthDaysType> = LazyLock::new(|| {
    SimpleFilter::new(
        Period::new(pos!(93502)),
        [
            LimitedCycle::new(pos!(5844), pos!(15)),
            LimitedCycle::new(pos!(2922), pos!(1)),
            LimitedCycle::new(pos!(365), pos!(7)),
        ],
    )
});

#[derive(Debug, PartialEq, Eq, Constructor, Into, From, Clone)]
pub struct Depth(pub(crate) DepthPoint);

impl From<Year> for Depth {
    fn from(year: Year) -> Self {
        Self(DEPTH_YEARS.wind(year.into()))
    }
}

impl From<&Depth> for Year {
    fn from(value: &Depth) -> Self {
        let Depth(bound_point) = value;
        Year::from(DEPTH_YEARS.unwind(&bound_point.point))
    }
}

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct DepthWithDay {
    pub depth: Depth,
    pub day: YearDay,
}

impl From<Day> for DepthWithDay {
    fn from(day: Day) -> Self {
        let point = DEPTH_DAYS.wind(day.into());
        Self::new(
            Depth::new(BoundMixedPoint::new(&*DEPTH_YEARS, point.point.point)),
            YearDay::new(point.point.remainder),
        )
    }
}

#[cfg(test)]
mod test {
    use mixed_point::{BoundMixedPoint, MixedPoint};
    use numburs::{int, integral::testing::*, nat, Integer, LowerBoundedRep, Natural};

    use super::{Depth, DepthWithDay, DEPTH_YEARS};
    use crate::{
        base::I,
        system_n::units::{Day, Year, YearDay},
    };

    fn cons_depth(aeons: Integer<I>, phase: [Natural<I>; 3]) -> Depth {
        Depth::new(BoundMixedPoint::new(&*DEPTH_YEARS, MixedPoint::new(aeons, phase)))
    }

    fn cons_depth_with_day(aeons: Integer<I>, phase: [Natural<I>; 3], day: Natural<I>) -> DepthWithDay {
        DepthWithDay::new(cons_depth(aeons, phase), YearDay::new(day))
    }

    #[test]
    fn depth_years() {
        assert_eq!(cons_depth(I0, [N0, N0, N0]), Year::new(I0).into());
        assert_eq!(cons_depth(I2, [N3, N1, N5]), Year::new(int!(603)).into());
        assert_eq!(cons_depth(-I1, [N7, N1, nat!(15)]), Year::new(-I1).into());
    }

    #[test]
    fn years_from_depth() {
        assert_eq!(Year::new(I0), (&cons_depth(I0, [N0, N0, N0])).into());
        assert_eq!(Year::new(int!(603)), (&cons_depth(I2, [N3, N1, N5])).into());
        assert_eq!(Year::new(-I1), (&cons_depth(-I1, [N7, N1, nat!(15)])).into());
    }

    #[test]
    fn depth_day() {
        let i364 = int!(364);
        let i365 = int!(365);
        let n15 = nat!(15);
        let n364 = nat!(364);
        let n365 = nat!(365);
        let n366 = nat!(366);
        assert_eq!(cons_depth_with_day(I0, [N0, N0, N0], N0), Day::new(I0).into());
        assert_eq!(cons_depth_with_day(I0, [N0, N0, N0], N1), Day::new(I1).into());
        assert_eq!(cons_depth_with_day(I0, [N0, N0, N0], n364), Day::new(i364).into());
        assert_eq!(cons_depth_with_day(I0, [N1, N0, N0], N0), Day::new(i365).into());
        assert_eq!(cons_depth_with_day(I0, [N7, N0, N0], n365), Day::new(int!(2920)).into());
        assert_eq!(cons_depth_with_day(I0, [N7, N0, N0], n366), Day::new(int!(2921)).into());
        assert_eq!(cons_depth_with_day(I0, [N0, N1, N0], N0), Day::new(int!(2922)).into());
        assert_eq!(cons_depth_with_day(-I1, [N7, N1, n15], n364), Day::new(-I1).into());
    }
}
