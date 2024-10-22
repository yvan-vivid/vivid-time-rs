use core::{default::Default, marker::PhantomData};
use std::fmt;

use numburs::{ConstrainedRep, Fractional, Integer, Natural};

use super::types::Formatter;

#[derive(Debug)]
pub struct ConstrainedRepFormatter<U: fmt::Display + Copy, C: ConstrainedRep<U>> {
    _u_marker: PhantomData<U>,
    _c_marker: PhantomData<C>,
}

impl<U: fmt::Display + Copy, C: ConstrainedRep<U>> Default for ConstrainedRepFormatter<U, C> {
    fn default() -> Self {
        Self {
            _u_marker: Default::default(),
            _c_marker: Default::default(),
        }
    }
}

impl<U: fmt::Display + Copy, C: ConstrainedRep<U>> Formatter<C> for ConstrainedRepFormatter<U, C> {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, number: &C) -> fmt::Result {
        write!(buffer, "{}", number.out())
    }
}

pub type IntegerFormatter<U> = ConstrainedRepFormatter<U, Integer<U>>;
pub type NaturalFormatter<U> = ConstrainedRepFormatter<U, Natural<U>>;
pub type FractionalFormatter<U> = ConstrainedRepFormatter<U, Fractional<U>>;
