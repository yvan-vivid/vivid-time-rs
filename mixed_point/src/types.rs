use std::array;

use derive_more::Constructor;
use numburs::{ConstrainedRep, Natural, NumBase};
use serde::{ser::SerializeStruct, Serialize, Serializer};

pub type Phase<U, const N: usize> = [Natural<U>; N];
pub type Factors<C, const N: usize> = [C; N];
pub type NamedPhase<'a, U, const N: usize> = [(&'a str, Natural<U>); N];

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct PhaseLegend<'a, const N: usize>([&'a str; N]);

impl<'a, const N: usize> PhaseLegend<'a, N> {
    pub fn name<U: NumBase>(&self, phase: Phase<U, N>) -> NamedPhase<'a, U, N> {
        array::from_fn(|i| (self.0[i], phase[i]))
    }
}

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct PhaseWithLegend<'a, 'b, U: NumBase, const N: usize> {
    pub phase: Phase<U, N>,
    pub legend: &'a PhaseLegend<'b, N>,
    pub label: &'b str,
}

impl<'a, U: NumBase + Serialize, const N: usize> Serialize for PhaseWithLegend<'a, 'static, U, N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct(self.label, N)?;
        for (name, phase) in self.legend.name(self.phase).iter().rev() {
            state.serialize_field(name, &phase.out())?;
        }
        state.end()
    }
}

#[cfg(test)]
mod test {
    use numburs::{nat, LowerBoundedRep, Natural};

    use super::{NamedPhase, Phase, PhaseLegend};

    #[test]
    fn name_empty_phase() {
        let phase: Phase<i32, 0> = [];
        let legend = PhaseLegend::new([]);
        let expected: NamedPhase<i32, 0> = [];
        assert_eq!(expected, legend.name(phase));
    }

    #[test]
    fn name_phase() {
        let phase = [nat!(1), nat!(2), nat!(3)];
        let legend = PhaseLegend::new(["a", "b", "c"]);
        assert_eq!([("a", nat!(1)), ("b", nat!(2)), ("c", nat!(3))], legend.name(phase));
    }
}
