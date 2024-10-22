use std::sync::LazyLock;

use mixed_point::{
    BoundMixedPhase, Cycle, IdentityPeriodRepresentation, Mixed, NamedPhase, Phase, PhaseLegend, SimpleCycle,
    SimpleMixed,
};
use numburs::{pos, ConstrainedRep, HasZero, Integer, LowerBoundedRep, Natural, Positive};

use super::units::YearDay;
use crate::base::I;

pub type CalendarType = SimpleMixed<IdentityPeriodRepresentation<I>, SimpleCycle<I>, 4>;
pub type CalendarPoint = BoundMixedPhase<'static, 4, CalendarType>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Span(pub CalendarPoint);

pub static CALENDAR: LazyLock<CalendarType> =
    LazyLock::new(|| SimpleMixed::from_simple_factors([pos!(8), pos!(3), pos!(3), pos!(5)]));

pub static CALENDAR_LEGEND: LazyLock<PhaseLegend<'static, 4>> =
    LazyLock::new(|| PhaseLegend::new(["point", "arc", "spoke", "period"]));

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Calendar {
    Span(Span),
    Interstice(Natural<I>),
}

impl Span {
    pub fn from_phase(phase: Phase<I, 4>) -> Option<Self> {
        CALENDAR.point(Integer::ZERO, phase).map(|b| Span(b.phase()))
    }

    pub fn fill_phase(day: YearDay) -> Result<Self, Natural<I>> {
        let excess = day.0.out() - CALENDAR.period.size().out();
        if excess >= 0 {
            Err(Natural::at_least(excess))
        } else {
            Ok(Self(CALENDAR.wind(day.0.into()).phase()))
        }
    }

    pub fn name(&self) -> NamedPhase<'static, I, 4> {
        CALENDAR_LEGEND.name(self.0.phase)
    }
}

impl From<YearDay> for Calendar {
    fn from(day: YearDay) -> Self {
        match Span::fill_phase(day) {
            Ok(span) => Calendar::Span(span),
            Err(interstice) => Calendar::Interstice(interstice),
        }
    }
}

#[cfg(test)]
mod test {
    use numburs::{integral::testing::*, nat, LowerBoundedRep, Natural};

    use super::Span;
    use crate::{
        base::I,
        system_n::{calendar::Calendar, units::YearDay},
    };

    fn cons_span(phase: [Natural<I>; 4]) -> Calendar {
        Calendar::Span(Span::from_phase(phase).expect("Bad phase given"))
    }

    #[test]
    fn calendar() {
        assert_eq!(Calendar::Interstice(N0), YearDay::new(nat!(360)).into());
        assert_eq!(Calendar::Interstice(N5), YearDay::new(nat!(365)).into());
        assert_eq!(Calendar::Interstice(nat!(10)), YearDay::new(nat!(370)).into());
        assert_eq!(cons_span([N0, N0, N0, N0]), YearDay::new(N0).into());
        assert_eq!(cons_span([N2, N1, N0, N0]), YearDay::new(nat!(10)).into());
        assert_eq!(cons_span([N7, N2, N2, N4]), YearDay::new(nat!(359)).into());
    }
}
