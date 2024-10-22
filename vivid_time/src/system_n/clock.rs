use std::sync::LazyLock;

use derive_more::Constructor;
use mixed_point::{
    BoundMixedPhase, IdentityPeriodRepresentation, Mixed, NamedPhase, Phase, PhaseLegend, SimpleCycle, SimpleMixed,
};
use numburs::{pos, HasZero, Integer, LowerBoundedRep, Positive};

use super::units::{Day, Edge, Edges};
use crate::{base::I, temporal::standard::Days};

pub type ClockType = SimpleMixed<IdentityPeriodRepresentation<I>, SimpleCycle<I>, 5>;
pub type ClockPoint = BoundMixedPhase<'static, 5, ClockType>;

pub static CLOCK: LazyLock<ClockType> =
    LazyLock::new(|| SimpleMixed::from_simple_factors([pos!(2), pos!(64), pos!(64), pos!(8), pos!(16)]));

pub static CLOCK_LEGEND: LazyLock<PhaseLegend<'static, 5>> =
    LazyLock::new(|| PhaseLegend::new(["edge", "event", "moment", "beat", "rhythm"]));

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct Clock(pub ClockPoint);

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct ClockDuration {
    pub days: Days,
    pub clock: Clock,
}

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct ClockWithDay {
    pub day: Day,
    pub clock: Clock,
}

impl Clock {
    pub fn from_phase(phase: Phase<I, 5>) -> Option<Self> {
        CLOCK.point(Integer::ZERO, phase).map(|b| Clock(b.phase()))
    }

    pub fn name(&self) -> NamedPhase<'static, I, 5> {
        CLOCK_LEGEND.name(self.0.phase)
    }
}

impl From<Edges> for ClockDuration {
    fn from(edges: Edges) -> Self {
        let point = CLOCK.wind(edges.into());
        Self::new(Days::new(point.point.cycle), Clock(point.phase()))
    }
}

impl From<Edge> for ClockWithDay {
    fn from(edge: Edge) -> Self {
        let ClockDuration { days, clock } = ClockDuration::from(Edges::from(edge));
        Self::new(Day::new(days.into()), clock)
    }
}

#[cfg(test)]
mod test {
    use numburs::{int, integral::testing::*, Integer, Natural};

    use super::{Clock, ClockDuration};
    use crate::{base::I, system_n::units::Edges, temporal::standard::Days};

    fn cons_days_clock(days: Integer<I>, phase: [Natural<I>; 5]) -> ClockDuration {
        ClockDuration::new(Days::new(days), Clock::from_phase(phase).expect("Improper phase"))
    }

    #[test]
    fn calendar() {
        assert_eq!(cons_days_clock(I0, [N0, N0, N0, N0, N0]), Edges::new(I0).into());
        assert_eq!(cons_days_clock(I0, [N1, N0, N0, N0, N0]), Edges::new(I1).into());
        assert_eq!(cons_days_clock(I0, [N1, N6, N1, N0, N0]), Edges::new(int!(141)).into());
    }
}
