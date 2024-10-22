pub mod integral;
pub mod real;
pub mod representation;
pub use integral::{Integer, Many, Natural, Positive};
pub use real::{Fractional, Real, WithFraction};
pub use representation::{BoundedRepLeft, ConstrainedRep, FloatBase, HasOne, HasZero, LowerBoundedRep, NumBase};
