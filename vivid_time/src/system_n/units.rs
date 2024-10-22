use derive_more::{Constructor, From, Into};
use numburs::{ConstrainedRep, Fractional, Integer, Natural, Real, WithFraction};

use crate::{
    base::{I, R},
    temporal::standard::{Days, RealDays},
};

////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////

// 4 + 3 + 6 + 6 + 1 = 20, 2^20 = 1_048_576
const EDGES_PER_DAY: I = 16 * 8 * 64 * 64 * 2;
const REAL_EDGES_PER_DAY: R = EDGES_PER_DAY as R;

////////////////////////////////////////////////////////////////////////////////
// Fixed Duration
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug, Clone, From, Into, Constructor)]
pub struct Edges(pub(crate) Integer<I>);

#[derive(PartialEq, Debug, Clone, From, Into, Constructor)]
pub struct RealEdges(pub(crate) Real<R>);

#[derive(PartialEq, Debug, Clone, From, Into, Constructor)]
pub struct EdgesFraction(pub(crate) Fractional<R>);

////////////////////////////////////////////////////////////////////////////////
// Variable Duration
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug, Clone, From, Into, Constructor)]
pub struct Years(pub(crate) Integer<I>);

////////////////////////////////////////////////////////////////////////////////
// Fixed Ordinal
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug, Clone, From, Into, Constructor)]
pub struct Edge(pub(crate) Integer<I>);

#[derive(PartialEq, Debug, Clone, From, Into, Constructor)]
pub struct RealEdge(pub(crate) Real<R>);

#[derive(PartialEq, Eq, Debug, Clone, From, Into, Constructor)]
pub struct Day(pub(crate) Integer<I>);

#[derive(PartialEq, Debug, Clone, From, Into, Constructor)]
pub struct RealDay(pub(crate) Real<R>);

////////////////////////////////////////////////////////////////////////////////
// Variable Ordinal
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug, Clone, From, Into, Constructor)]
pub struct Year(pub(crate) Integer<I>);

////////////////////////////////////////////////////////////////////////////////
// Relative Fixed Ordinal
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug, Clone, From, Into, Constructor)]
pub struct YearDay(pub(crate) Natural<I>);

#[derive(PartialEq, Debug, Clone, From, Into, Constructor)]
pub struct EdgeFraction(pub(crate) Fractional<R>);

////////////////////////////////////////////////////////////////////////////////
// Composite
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug, Clone, From, Into, Constructor)]
pub struct EdgeWithFraction {
    pub edge: Edge,
    pub fraction: EdgeFraction,
}

////////////////////////////////////////////////////////////////////////////////
// Conversion
////////////////////////////////////////////////////////////////////////////////

impl From<Edge> for Edges {
    fn from(value: Edge) -> Self {
        Self(value.into())
    }
}

impl From<RealEdge> for RealEdges {
    fn from(value: RealEdge) -> Self {
        Self(value.into())
    }
}

impl From<Day> for Days {
    fn from(value: Day) -> Self {
        Self(value.into())
    }
}

impl From<RealDay> for RealDays {
    fn from(value: RealDay) -> Self {
        Self(value.into())
    }
}

impl From<YearDay> for Days {
    fn from(value: YearDay) -> Self {
        Self::from(Integer::<I>::from(value.0))
    }
}

impl From<RealEdge> for EdgeWithFraction {
    fn from(value: RealEdge) -> Self {
        let WithFraction::<Integer<I>, f64> { whole, fraction } = value.0.fractionalize();
        Self::new(Edge::new(whole), EdgeFraction::new(fraction))
    }
}

impl From<RealDays> for RealEdges {
    fn from(value: RealDays) -> Self {
        Self(Real::from(REAL_EDGES_PER_DAY * value.0.out()))
    }
}

impl From<RealDay> for RealEdge {
    fn from(value: RealDay) -> Self {
        Self(RealEdges::from(RealDays::from(value)).0)
    }
}

#[cfg(test)]
mod test {
    use ntest::{assert_about_eq, test_case};
    use numburs::{ConstrainedRep, Real};

    use super::{RealDay, RealEdge};
    use crate::base::R;

    fn real_day_float(f: R) -> RealDay {
        Real::from(f).into()
    }

    #[test_case(0., 0.)]
    #[test_case(1048576., 1.)]
    fn real_edge_from_real_day(ex: R, ac: R) {
        assert_about_eq!(ex, RealEdge::from(real_day_float(ac)).0.out());
    }
}
