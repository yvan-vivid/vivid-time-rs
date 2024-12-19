use std::fmt;

use numburs::Integer;

use super::{
    mixed_point::{MixedPointFormatter, PhaseFormatter},
    numburs::{FractionalFormatter, IntegerFormatter},
    types::Formatter,
};
use crate::{
    base::{I, R},
    system_n::{
        calendar::Calendar,
        clock::Clock,
        depth::Depth,
        time::{Date, Time, TimeWithFraction},
        units::{EdgeFraction, Year},
    },
};

pub enum DepthStyle {
    Short,
    Long,
}

////////////////////////////////////////////////////////////////////////////////
// Formatters for Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct YearFormatter(IntegerFormatter<I>);

impl Formatter<Year> for YearFormatter {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, data: &Year) -> fmt::Result {
        self.0.fmt(buffer, &data.0)
    }
}

pub enum DepthFormatter<'a> {
    Depth(MixedPointFormatter<'a, I, Integer<I>, IntegerFormatter<I>>),
    Year(YearFormatter),
}

impl Formatter<Depth> for DepthFormatter<'_> {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, data: &Depth) -> fmt::Result {
        match self {
            DepthFormatter::Depth(formatter) => formatter.fmt(buffer, &data.0.point),
            DepthFormatter::Year(formatter) => formatter.fmt(buffer, &Year::from(data)),
        }
    }
}

pub struct CalendarFormatter<'a> {
    phase: PhaseFormatter<'a, I>,
}

impl<'a> CalendarFormatter<'a> {
    pub fn standard(separator: &'a str) -> Self {
        Self {
            phase: PhaseFormatter::standard(separator),
        }
    }
}

impl Formatter<Calendar> for CalendarFormatter<'_> {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, data: &Calendar) -> fmt::Result {
        match data {
            Calendar::Span(span) => self.phase.fmt(buffer, &span.0.phase),
            Calendar::Interstice(i) => write!(buffer, "Interstice {:?}", i),
        }
    }
}

pub struct ClockFormatter<'a> {
    phase: PhaseFormatter<'a, I>,
}

impl<'a> ClockFormatter<'a> {
    pub fn standard(separator: &'a str, precision: Option<usize>) -> Self {
        Self {
            phase: PhaseFormatter::standard_with_precision(separator, precision),
        }
    }
}

impl Formatter<Clock> for ClockFormatter<'_> {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, data: &Clock) -> fmt::Result {
        self.phase.fmt(buffer, &data.0.phase)
    }
}

pub struct DateFormatter<'a> {
    depth: DepthFormatter<'a>,
    calendar: CalendarFormatter<'a>,
    separator: &'a str,
}

impl<'a> DateFormatter<'a> {
    pub fn standard(phase_separator: &'a str, depth_style: DepthStyle) -> Self {
        let depth = match depth_style {
            DepthStyle::Short => DepthFormatter::Year(Default::default()),
            DepthStyle::Long => DepthFormatter::Depth(MixedPointFormatter::new(
                ": ",
                Default::default(),
                PhaseFormatter::standard(phase_separator),
            )),
        };
        Self {
            depth,
            calendar: CalendarFormatter::standard(phase_separator),
            separator: ": ",
        }
    }
}

impl Formatter<Date> for DateFormatter<'_> {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, data: &Date) -> fmt::Result {
        write!(buffer, "∆ ")?;
        self.depth.fmt(buffer, &data.depth)?;
        write!(buffer, "{}", self.separator)?;
        self.calendar.fmt(buffer, &data.calendar)
    }
}

pub struct TimeFormatter<'a> {
    date: DateFormatter<'a>,
    clock: ClockFormatter<'a>,
    separator: &'a str,
}

impl<'a> TimeFormatter<'a> {
    pub fn standard(phase_separator: &'a str, depth_style: DepthStyle, clock_precision: Option<usize>) -> Self {
        Self {
            date: DateFormatter::standard(phase_separator, depth_style),
            clock: ClockFormatter::standard(phase_separator, clock_precision),
            separator: " / ",
        }
    }
}

impl Formatter<Time> for TimeFormatter<'_> {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, data: &Time) -> fmt::Result {
        self.date.fmt(buffer, &data.date)?;
        write!(buffer, "{}", self.separator)?;
        self.clock.fmt(buffer, &data.clock)
    }
}

#[derive(Debug, Default)]
pub struct EdgeFractionFormatter(FractionalFormatter<R>);

impl Formatter<EdgeFraction> for EdgeFractionFormatter {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, data: &EdgeFraction) -> fmt::Result {
        self.0.fmt(buffer, &data.0)
    }
}

pub struct TimeWithFractionFormatter<'a> {
    separator: &'a str,
    time: TimeFormatter<'a>,
    fraction: EdgeFractionFormatter,
}

impl<'a> TimeWithFractionFormatter<'a> {
    pub fn standard(separator: &'a str, phase_separator: &'a str, depth_style: DepthStyle) -> Self {
        Self {
            separator,
            time: TimeFormatter::standard(phase_separator, depth_style, None),
            fraction: Default::default(),
        }
    }
}

impl Formatter<TimeWithFraction> for TimeWithFractionFormatter<'_> {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, data: &TimeWithFraction) -> fmt::Result {
        self.time.fmt(buffer, &data.time)?;
        write!(buffer, "{}", self.separator)?;
        self.fraction.fmt(buffer, &data.fraction)
    }
}
