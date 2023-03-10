#![allow(clippy::integer_arithmetic)]

#[cfg(not(feature = "std"))]
use crate::prelude::*;
use crate::{ParticipationFlags, Unsigned, VariableList};

/// Wrapper type allowing the implementation of `CachedTreeHash`.
#[derive(Debug)]
pub struct ParticipationList<'a, N: Unsigned> {
    pub inner: &'a VariableList<ParticipationFlags, N>,
}

impl<'a, N: Unsigned> ParticipationList<'a, N> {
    pub fn new(inner: &'a VariableList<ParticipationFlags, N>) -> Self {
        Self { inner }
    }
}
