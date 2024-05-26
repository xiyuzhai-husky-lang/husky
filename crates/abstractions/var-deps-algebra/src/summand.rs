use super::*;

const EXCLUDES_N: usize = 4;
const SUMMANDS_N: usize = 4;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VarDepsSummand<A, S> {
    pub base: A,
    pub excludes: OrderedSmallVecSet<S, EXCLUDES_N>,
}

impl<A, S> From<(A, Vec<S>)> for VarDepsSummand<A, S>
where
    S: Ord + Copy,
{
    fn from((base, excludes): (A, Vec<S>)) -> Self {
        Self {
            base,
            excludes: excludes.into(),
        }
    }
}

impl<A, S> std::fmt::Display for VarDepsSummand<A, S>
where
    A: std::fmt::Display,
    S: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self.base))?;
        if self.excludes.is_empty() {
            return Ok(());
        }
        f.write_str("[")?;
        for (i, exclude) in self.excludes.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            exclude.fmt(f)?
        }
        f.write_str("]")
    }
}

impl<A, S> AsVecMapEntry for VarDepsSummand<A, S> {
    type K = A;

    fn key_ref(&self) -> &Self::K {
        &self.base
    }
}

pub type VarDepsSummands<A, S> = OrderedSmallVecMap<VarDepsSummand<A, S>, SUMMANDS_N>;
