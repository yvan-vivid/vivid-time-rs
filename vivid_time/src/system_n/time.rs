use std::fmt::Debug;

use derive_more::Constructor;
use serde::Serialize;

use super::{
    calendar::Calendar,
    clock::{Clock, ClockWithDay},
    depth::{Depth, DepthWithDay},
    units::{Day, Edge, EdgeFraction, EdgeWithFraction, RealEdge, Year},
};

#[derive(Debug, PartialEq, Eq, Constructor, Serialize, Clone)]
pub struct Date {
    pub depth: Depth,
    pub year: Year,
    pub calendar: Calendar,
}

#[derive(Debug, PartialEq, Eq, Constructor, Serialize, Clone)]
pub struct Time {
    pub date: Date,
    pub clock: Clock,
}

#[derive(Debug, PartialEq, Constructor, Serialize, Clone)]
pub struct TimeWithFraction {
    pub time: Time,
    pub fraction: EdgeFraction,
}

impl From<Day> for Date {
    fn from(value: Day) -> Self {
        let DepthWithDay { depth, day } = value.into();
        let calendar: Calendar = day.into();
        let year: Year = (&depth).into();
        Self { depth, year, calendar }
    }
}

impl From<Edge> for Time {
    fn from(value: Edge) -> Self {
        let ClockWithDay { day, clock } = value.into();
        let date: Date = day.into();
        Self { date, clock }
    }
}

impl From<RealEdge> for TimeWithFraction {
    fn from(value: RealEdge) -> Self {
        let EdgeWithFraction { edge, fraction } = value.into();
        let time: Time = edge.into();
        Self { time, fraction }
    }
}

impl From<Time> for Date {
    fn from(value: Time) -> Self {
        value.date
    }
}

impl From<Time> for Clock {
    fn from(value: Time) -> Self {
        value.clock
    }
}

impl From<TimeWithFraction> for Time {
    fn from(value: TimeWithFraction) -> Self {
        value.time
    }
}

impl From<TimeWithFraction> for Date {
    fn from(value: TimeWithFraction) -> Self {
        Time::from(value).into()
    }
}

impl From<TimeWithFraction> for Clock {
    fn from(value: TimeWithFraction) -> Self {
        Time::from(value).into()
    }
}
