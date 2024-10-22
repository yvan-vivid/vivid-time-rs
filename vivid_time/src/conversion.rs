use chrono::{DateTime, Utc};
use numburs::Real;

use crate::{
    base::{I, R},
    system_n::{
        time::{Date as VividDate, Time, TimeWithFraction},
        units::{RealDay as RealVividDay, RealEdge},
    },
    temporal::standard::RealDays,
    unix::units::{Now, RealDay as RealUnixDay, RealUnixSecond, REAL_UNIX_SECOND_DAYS},
};

pub type Instant = DateTime<Utc>;

////////////////////////////////////////////////////////////////////////////////
// Conversion Utilities
////////////////////////////////////////////////////////////////////////////////

fn micros_to_real_seconds(micros: i64) -> R {
    micros as f64 / 1_000_000.0
}

////////////////////////////////////////////////////////////////////////////////
// Time Definition
////////////////////////////////////////////////////////////////////////////////

// July 28th, 2005, 05:30 NYC Time
pub const ZERO_UNIX_SECOND_I: I = 1122543000;
pub const ZERO_UNIX_SECOND_R: R = ZERO_UNIX_SECOND_I as R;
pub const ZERO_UNIX_DAY_R: R = ZERO_UNIX_SECOND_R * REAL_UNIX_SECOND_DAYS;

// Conversion between Unix and SystemN
impl From<RealUnixDay> for RealVividDay {
    fn from(value: RealUnixDay) -> Self {
        let day: RealDays = value.into();
        Self::from(day.0 - Real::<R>::from(ZERO_UNIX_DAY_R))
    }
}

impl From<RealUnixSecond> for RealVividDay {
    fn from(value: RealUnixSecond) -> Self {
        RealUnixDay::from(value).into()
    }
}

impl From<RealUnixSecond> for RealEdge {
    fn from(value: RealUnixSecond) -> Self {
        RealVividDay::from(value).into()
    }
}

impl From<RealUnixSecond> for TimeWithFraction {
    fn from(value: RealUnixSecond) -> Self {
        RealEdge::from(value).into()
    }
}

impl From<RealUnixSecond> for Time {
    fn from(value: RealUnixSecond) -> Self {
        TimeWithFraction::from(value).into()
    }
}

impl From<RealUnixSecond> for VividDate {
    fn from(value: RealUnixSecond) -> Self {
        TimeWithFraction::from(value).into()
    }
}

impl Now for RealVividDay {}
impl Now for RealEdge {}
impl Now for TimeWithFraction {}
impl Now for Time {}
impl Now for VividDate {}

// Conversion between chrono and SystemN via Unix

impl From<Instant> for RealUnixSecond {
    fn from(value: Instant) -> Self {
        Real::from(micros_to_real_seconds(value.timestamp_micros())).into()
    }
}

impl From<Instant> for TimeWithFraction {
    fn from(value: Instant) -> Self {
        RealUnixSecond::from(value).into()
    }
}

#[cfg(test)]
mod test {
    use ntest::{assert_about_eq, test_case};
    use numburs::{ConstrainedRep, Real};

    use super::{micros_to_real_seconds, RealUnixDay, RealVividDay};
    use crate::base::R;

    #[test_case(0., 0)]
    #[test_case(0.000_001, 1)]
    #[test_case(-0.000_001, -1)]
    #[test_case(1.234_567, 1_234_567)]
    #[test_case(-1.234_567, -1_234_567)]
    fn real_unix_seconds_from_micros(ex: R, ac: i64) {
        assert_about_eq!(ex, micros_to_real_seconds(ac));
    }

    fn real_unix_day_float(f: R) -> RealUnixDay {
        Real::<R>::from(f).into()
    }

    #[test_case(-12992.395833, 0.)]
    #[test_case(0., 12992.395833)]
    #[test_case(10., 13002.395833)]
    fn unix_to_vivid(ex: R, ac: R) {
        assert_about_eq!(ex, RealVividDay::from(real_unix_day_float(ac)).0.out());
    }
}
