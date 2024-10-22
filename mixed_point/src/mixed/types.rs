use core::fmt::Debug;

use derive_more::Constructor;
use numburs::NumBase;

use crate::types::{Phase, PhaseLegend};

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct MixedPoint<U: NumBase, const N: usize, Cycles> {
    pub cycle: Cycles,
    pub phase: Phase<U, N>,
}

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct BoundMixedPhase<'a, const N: usize, P: Mixed<N>> {
    pub schema: &'a P,
    pub phase: Phase<P::U, N>,
}

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct BoundMixedPoint<'a, const N: usize, P: Mixed<N>> {
    pub schema: &'a P,
    pub point: MixedPoint<P::U, N, P::Cycles>,
}

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct MixedPointLegend<'a, const N: usize> {
    pub cycle: &'a str,
    pub phase: PhaseLegend<'a, N>,
}

impl<'a, const N: usize, P: Mixed<N>> BoundMixedPoint<'a, N, P> {
    pub fn phase(&self) -> BoundMixedPhase<'a, N, P> {
        BoundMixedPhase::new(self.schema, self.point.phase)
    }
}

impl<'a, const N: usize> MixedPointLegend<'a, N> {
    pub fn with_names(cycle: &'a str, phase: [&'a str; N]) -> Self {
        Self {
            cycle,
            phase: PhaseLegend::new(phase),
        }
    }
}

pub trait Mixed<const N: usize>: Sized {
    type Cycles;
    type U: NumBase;

    fn is_norm(&self, point: &MixedPoint<Self::U, N, Self::Cycles>) -> bool;
    fn wind_inner(&self, total: &Self::Cycles) -> MixedPoint<Self::U, N, Self::Cycles>;
    fn unwind(&self, point: &MixedPoint<Self::U, N, Self::Cycles>) -> Self::Cycles;

    fn bind(&self, point: MixedPoint<Self::U, N, Self::Cycles>) -> Option<BoundMixedPoint<N, Self>> {
        Some(point)
            .filter(|p| self.is_norm(p))
            .map(|p| BoundMixedPoint::new(self, p))
    }

    fn point(&self, cycle: Self::Cycles, phase: Phase<Self::U, N>) -> Option<BoundMixedPoint<N, Self>> {
        self.bind(MixedPoint::new(cycle, phase))
    }

    fn wind(&self, total: Self::Cycles) -> BoundMixedPoint<N, Self> {
        BoundMixedPoint::new(self, self.wind_inner(&total))
    }
}
