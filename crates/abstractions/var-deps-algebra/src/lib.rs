use vec_like::{AsVecMapEntry, OrderedSmallVecSet};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VarDepsSum<A, S>
where
    S: std::fmt::Debug + Ord + Copy,
{
    summands: VarDepsSummands<A, S>,
}

const EXCLUDES_N: usize = 4;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VarDepsSummand<A, S>
where
    S: std::fmt::Debug + Ord + Copy,
{
    base: A,
    excludes: OrderedSmallVecSet<S, EXCLUDES_N>,
}

impl<A, S> AsVecMapEntry for VarDepsSummand<A, S>
where
    S: std::fmt::Debug + Ord + Copy,
{
    type K = A;

    fn key_ref(&self) -> &Self::K {
        &self.base
    }
}

type VarDepsSummands<A, S> = Vec<VarDepsSummand<A, S>>;
