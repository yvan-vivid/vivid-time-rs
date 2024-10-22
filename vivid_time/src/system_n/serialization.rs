use mixed_point::{MixedPoint, PhaseWithLegend};
use numburs::ConstrainedRep;
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use super::{
    calendar::{Calendar, CALENDAR_LEGEND},
    clock::{Clock, CLOCK_LEGEND},
    depth::{Depth, DEPTH_LEGEND},
    units::{EdgeFraction, Year},
};

impl Serialize for Depth {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let MixedPoint { cycle, phase } = self.0.point;
        let mut state = s.serialize_struct("Depth", 4)?;
        state.serialize_field(DEPTH_LEGEND.cycle, &cycle.out())?;
        state.serialize_field("phase", &PhaseWithLegend::new(phase, &DEPTH_LEGEND.phase, "DepthPhase"))?;
        state.end()
    }
}

impl Serialize for Year {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_newtype_struct("Year", &self.0.out())
    }
}

impl Serialize for Calendar {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Calendar::Span(span) => {
                let mut state = s.serialize_struct("Calendar", 2)?;
                state.serialize_field("span", &PhaseWithLegend::new(span.0.phase, &CALENDAR_LEGEND, "Span"))?;
                state.end()
            }
            Calendar::Interstice(i) => {
                let mut state = s.serialize_struct("Calendar", 2)?;
                state.serialize_field("interstice", &i.out())?;
                state.end()
            }
        }
    }
}

impl Serialize for Clock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        PhaseWithLegend::new(self.0.phase, &CLOCK_LEGEND, "Clock").serialize(s)
    }
}

impl Serialize for EdgeFraction {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_newtype_struct("EdgeFraction", &self.0.out())
    }
}
