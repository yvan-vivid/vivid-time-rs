use std::time::{SystemTime, UNIX_EPOCH};

use derive_more::{From, Into};
use numburs::{ConstrainedRep, Integer, Real};

use crate::{
    base::{I, R},
    temporal::standard::{Days, RealDays},
};

////////////////////////////////////////////////////////////////////////////////
// Base units for time representation
// The common currency among time representations will be the 'Unix Second'
// This refers to the number of seconds passed the unix epoch
//     1970-01-01T00:00:00Z
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////

pub const UNIX_DAY_SECONDS: I = 24 * 60 * 60;
pub const REAL_UNIX_DAY_SECONDS: R = UNIX_DAY_SECONDS as R;
pub const REAL_UNIX_SECOND_DAYS: R = 1.0 / REAL_UNIX_DAY_SECONDS;

////////////////////////////////////////////////////////////////////////////////
// Fixed Duration
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug, Clone, From, Into)]
pub struct UnixSeconds(pub(crate) Integer<I>);

#[derive(PartialEq, Debug, Clone, From, Into)]
pub struct RealUnixSeconds(pub(crate) Real<R>);

////////////////////////////////////////////////////////////////////////////////
// Fixed Ordinal
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug, Clone, From, Into)]
pub struct UnixSecond(pub(crate) Integer<I>);

#[derive(PartialEq, Debug, Clone, From, Into)]
pub struct RealUnixSecond(pub(crate) Real<R>);

#[derive(PartialEq, Eq, Debug, Clone, From, Into)]
pub struct Day(pub(crate) Integer<I>);

#[derive(PartialEq, Debug, Clone, From, Into)]
pub struct RealDay(pub(crate) Real<R>);

////////////////////////////////////////////////////////////////////////////////
// Conversion
////////////////////////////////////////////////////////////////////////////////

impl From<Day> for Days {
    fn from(value: Day) -> Self {
        Self(value.0)
    }
}

impl From<UnixSecond> for UnixSeconds {
    fn from(value: UnixSecond) -> Self {
        Self(value.0)
    }
}

impl From<RealDay> for RealDays {
    fn from(value: RealDay) -> Self {
        Self(value.0)
    }
}

impl From<RealUnixSecond> for RealUnixSeconds {
    fn from(value: RealUnixSecond) -> Self {
        Self(value.0)
    }
}

impl From<RealUnixSeconds> for RealDays {
    fn from(value: RealUnixSeconds) -> Self {
        Self(Real::from(REAL_UNIX_SECOND_DAYS * value.0.out()))
    }
}

impl From<RealUnixSecond> for RealDay {
    fn from(value: RealUnixSecond) -> Self {
        Self(RealDays::from(RealUnixSeconds::from(value)).0)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Getting the current time
////////////////////////////////////////////////////////////////////////////////

// TODO: Carry error
pub fn now() -> RealUnixSecond {
    let rs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|now| now.as_secs_f64())
        .unwrap_or_default();
    RealUnixSecond(rs.into())
}

pub trait Now: From<RealUnixSecond> {
    fn now() -> Self {
        now().into()
    }
}

impl Now for RealDay {}

#[cfg(test)]
mod test {
    use ntest::{assert_about_eq, test_case};
    use numburs::{ConstrainedRep, Real};

    use super::{RealDay, RealUnixSecond};
    use crate::base::R;

    fn real_unix_seconds_float(f: R) -> RealUnixSecond {
        Real::from(f).into()
    }

    #[test_case(0.125, 3.)]
    #[test_case(1.0, 24.)]
    #[test_case(2.5, 60.)]
    fn unix_second_to_day(ex: R, ac: R) {
        assert_about_eq!(ex, RealDay::from(real_unix_seconds_float(ac * 3600.)).0.out());
    }
}
