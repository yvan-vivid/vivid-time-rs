use core::{cmp::min, ops::Mul};

use derive_more::Constructor;
use numburs::{ConstrainedRep, LowerBoundedRep, Natural, NumBase, Positive};

use super::types::{Cycle, CyclePoint};

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct LimitedCycle<N: NumBase> {
    size: Positive<N>,
    limit: Positive<N>,
}

impl<N: NumBase> Cycle<N> for LimitedCycle<N> {
    type Cycles = Natural<N>;

    fn size(&self) -> Positive<N> {
        self.size
    }

    fn is_norm(&self, point: &CyclePoint<N, Self::Cycles>) -> bool {
        point.phase < self.size.into() || point.cycle == self.limit.into()
    }

    fn wind_inner(&self, cycles: &Natural<N>) -> CyclePoint<N, Self::Cycles> {
        let q = min(*cycles / self.size, self.limit.into());
        CyclePoint::new(q, Natural::at_least(cycles.out() - q.out() * self.size.out()))
    }

    fn unwind(&self, point: &CyclePoint<N, Self::Cycles>) -> Self::Cycles {
        let CyclePoint { cycle, phase } = point;
        (*cycle) * self.size() + *phase
    }
}

impl<N: NumBase> Mul for LimitedCycle<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.size * rhs.size, rhs.limit)
    }
}

#[cfg(test)]
mod tests {
    use numburs::{nat, pos, LowerBoundedRep, Natural, Positive};

    use super::{super::types::Cycle, LimitedCycle};

    #[test]
    fn test_limited_cycle_wind() {
        for p in 1..7 {
            for l in 1..3 {
                let cycle = LimitedCycle::new(pos!(p), pos!(l));
                let mut q = 0;
                let mut r = 0;
                for k in 0..30 {
                    assert_eq!(cycle.point(nat!(q), nat!(r)), Some(cycle.wind(&nat!(k))));
                    r += 1;
                    if r == p && q < l {
                        q += 1;
                        r = 0;
                    }
                }
            }
        }
    }
}
