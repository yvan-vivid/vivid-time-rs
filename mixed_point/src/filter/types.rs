use core::fmt::Debug;

use derive_more::Constructor;
use numburs::{Natural, NumBase};

use crate::{mixed::types::MixedPoint, types::Phase};

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct FilterPoint<U: NumBase, const N: usize, Cycle> {
    pub point: MixedPoint<U, N, Cycle>,
    pub remainder: Natural<U>,
}

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct BoundFilterPoint<'a, const N: usize, P: Filter<N>> {
    pub schema: &'a P,
    pub point: FilterPoint<P::U, N, P::Cycle>,
}

pub trait Filter<const N: usize>: Sized {
    type Cycle;
    type U: NumBase;

    fn is_norm(&self, point: &FilterPoint<Self::U, N, Self::Cycle>) -> bool;
    fn wind_inner(&self, total: &Self::Cycle) -> FilterPoint<Self::U, N, Self::Cycle>;

    fn point(
        &self,
        cycle: Self::Cycle,
        phase: Phase<Self::U, N>,
        remainder: Natural<Self::U>,
    ) -> Option<BoundFilterPoint<N, Self>> {
        let point = MixedPoint::new(cycle, phase);
        let filter_point = FilterPoint::new(point, remainder);
        if self.is_norm(&filter_point) {
            Some(BoundFilterPoint::new(self, filter_point))
        } else {
            None
        }
    }

    fn wind(&self, total: Self::Cycle) -> BoundFilterPoint<N, Self> {
        BoundFilterPoint::new(self, self.wind_inner(&total))
    }
}
