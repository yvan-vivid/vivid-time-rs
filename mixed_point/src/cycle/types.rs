use derive_more::Constructor;
use numburs::{Natural, NumBase, Positive};

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct CyclePoint<N: NumBase, Cycles> {
    pub cycle: Cycles,
    pub phase: Natural<N>,
}

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct BoundCyclePoint<'a, N: NumBase, C: Cycle<N>> {
    pub scheme: &'a C,
    pub point: CyclePoint<N, C::Cycles>,
}

pub trait Cycle<N: NumBase>: Sized {
    type Cycles;

    fn size(&self) -> Positive<N>;
    fn is_norm(&self, point: &CyclePoint<N, Self::Cycles>) -> bool;
    fn wind_inner(&self, cycles: &Self::Cycles) -> CyclePoint<N, Self::Cycles>;
    fn unwind(&self, point: &CyclePoint<N, Self::Cycles>) -> Self::Cycles;

    fn bind(&self, point: CyclePoint<N, Self::Cycles>) -> Option<BoundCyclePoint<N, Self>> {
        Some(point)
            .filter(|p| self.is_norm(p))
            .map(|p| BoundCyclePoint::new(self, p))
    }

    fn point(&self, cycle: Self::Cycles, phase: Natural<N>) -> Option<BoundCyclePoint<N, Self>> {
        self.bind(CyclePoint::new(cycle, phase))
    }

    fn wind(&self, cycles: &Self::Cycles) -> BoundCyclePoint<N, Self> {
        BoundCyclePoint::new(self, self.wind_inner(cycles))
    }
}
