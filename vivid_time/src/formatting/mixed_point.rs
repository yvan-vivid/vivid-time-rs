use core::{cmp::min, marker::PhantomData};
use std::fmt;

use derive_more::Constructor;
use mixed_point::MixedPoint;
use numburs::{Natural, NumBase};

use super::{numburs::NaturalFormatter, types::Formatter};

#[derive(Debug, Constructor)]
pub struct PhaseFormatter<'a, U: NumBase + fmt::Display> {
    pub separator: &'a str,
    pub number: NaturalFormatter<U>,
    pub precision: Option<usize>,
}

impl<'a, U: NumBase + fmt::Display> PhaseFormatter<'a, U> {
    pub fn standard_with_precision(separator: &'a str, precision: Option<usize>) -> Self {
        Self {
            separator,
            number: Default::default(),
            precision,
        }
    }

    pub fn standard(separator: &'a str) -> Self {
        Self {
            separator,
            number: Default::default(),
            precision: None,
        }
    }
}

impl<U: NumBase + fmt::Display> Formatter<[Natural<U>]> for PhaseFormatter<'_, U> {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, phase: &[Natural<U>]) -> fmt::Result {
        let phase_length = phase.len();
        let start = phase_length - min(self.precision.unwrap_or(phase_length), phase_length);
        let mut its = phase[start..phase_length].iter().rev();

        if let Some(m) = its.next() {
            self.number.fmt(buffer, m)?;
        }

        for m in its {
            write!(buffer, "{}", self.separator)?;
            self.number.fmt(buffer, m)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct MixedPointFormatter<'a, U: NumBase + fmt::Display, Cycle, C: Formatter<Cycle>> {
    pub separator: &'a str,
    pub cycle: C,
    pub phase: PhaseFormatter<'a, U>,
    _c_marker: PhantomData<Cycle>,
}

impl<'a, U, Cycle, C> MixedPointFormatter<'a, U, Cycle, C>
where
    U: NumBase + fmt::Display,
    C: Formatter<Cycle>,
{
    pub fn new(separator: &'a str, cycle: C, phase: PhaseFormatter<'a, U>) -> Self {
        Self {
            separator,
            cycle,
            phase,
            _c_marker: Default::default(),
        }
    }
}

impl<U, const N: usize, Cycle, C> Formatter<MixedPoint<U, N, Cycle>> for MixedPointFormatter<'_, U, Cycle, C>
where
    U: NumBase + fmt::Display,
    C: Formatter<Cycle>,
{
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, point: &MixedPoint<U, N, Cycle>) -> fmt::Result {
        self.cycle.fmt(buffer, &point.cycle)?;
        write!(buffer, "{}", self.separator)?;
        self.phase.fmt(buffer, &point.phase)
    }
}
