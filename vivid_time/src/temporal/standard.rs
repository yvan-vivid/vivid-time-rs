use derive_more::{Constructor, From, Into};
use numburs::{ConstrainedRep, Fractional, Integer, Real};

use crate::base::{I, R};

////////////////////////////////////////////////////////////////////////////////
// Fixed Duration
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug, Clone, From, Into, Constructor)]
pub struct Days(pub(crate) Integer<I>);

#[derive(PartialEq, Debug, Clone, From, Into, Constructor)]
pub struct RealDays(pub(crate) Real<R>);

#[derive(PartialEq, Debug, Clone, From, Into)]
pub struct DaysFraction(pub(crate) Fractional<R>);

impl From<Days> for RealDays {
    fn from(d: Days) -> Self {
        Real::from(d.0.out() as R).into()
    }
}
