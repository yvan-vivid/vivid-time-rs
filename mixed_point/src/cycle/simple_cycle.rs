use derive_more::Constructor;
use numburs::{Natural, NumBase, Positive};

use super::types::{Cycle, CyclePoint};

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct SimpleCycle<N: NumBase> {
    size: Positive<N>,
}

impl<N: NumBase> Cycle<N> for SimpleCycle<N> {
    type Cycles = Natural<N>;

    fn size(&self) -> Positive<N> {
        self.size
    }

    fn is_norm(&self, point: &CyclePoint<N, Self::Cycles>) -> bool {
        point.phase < self.size.into()
    }

    fn wind_inner(&self, cycles: &Natural<N>) -> CyclePoint<N, Self::Cycles> {
        CyclePoint::new(*cycles / self.size, *cycles % self.size)
    }

    fn unwind(&self, point: &CyclePoint<N, Self::Cycles>) -> Self::Cycles {
        let CyclePoint { cycle, phase } = point;
        (*cycle) * self.size() + *phase
    }
}

#[cfg(test)]
mod tests {
    use numburs::{integral::testing::*, nat, pos, LowerBoundedRep, Natural, Positive};

    use super::SimpleCycle;
    use crate::cycle::types::{Cycle, CyclePoint};

    #[test]
    fn test_simple_cycle_point() {
        {
            let cycle = SimpleCycle::new(P1);
            assert_eq!(Some(CyclePoint::new(N4, N0)), cycle.point(N4, N0).map(|b| b.point));
            assert_eq!(None, cycle.point(nat!(4), nat!(1)).map(|b| b.point));
        }
        {
            let cycle = SimpleCycle::new(P3);
            assert_eq!(Some(CyclePoint::new(N4, N2)), cycle.point(N4, N2).map(|b| b.point));
            assert_eq!(None, cycle.point(nat!(4), nat!(3)).map(|b| b.point));
        }
    }

    #[test]
    fn test_simple_cycle_wind() {
        for p in 1..10 {
            let cycle = SimpleCycle::new(pos!(p));
            let mut q = 0;
            let mut r = 0;
            for k in 0..20 {
                assert_eq!(cycle.point(nat!(q), nat!(r)), Some(cycle.wind(&nat!(k))));
                r += 1;
                if r == p {
                    q += 1;
                    r = 0;
                }
            }
        }
    }
}
