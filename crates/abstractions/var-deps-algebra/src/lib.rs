#[cfg(test)]
mod tests;

#[cfg(test)]
use self::tests::*;
use vec_like::{
    ordered_small_vec_map::OrderedSmallVecMap, AsVecMapEntry, OrderedSmallVecSet, SmallVecMap,
};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VarDepsSum<A, S> {
    summands: VarDepsSummands<A, S>,
}

impl<A, S> std::fmt::Display for VarDepsSum<A, S>
where
    A: std::fmt::Display,
    S: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Display;

        f.write_str("(")?;
        for (i, summand) in self.summands.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            summand.fmt(f)?
        }
        f.write_str(")")
    }
}

#[test]
fn var_deps_sum_display_works() {
    fn t(sum: &VarDepsSum0, expected: &str) {
        assert_eq!(sum.to_string(), expected);
    }
    t(
        &VarDepsSum {
            summands: [].into(),
        },
        "()",
    );
    t(
        &VarDepsSum {
            summands: [VarDepsSummand {
                base: "a",
                excludes: [].into(),
            }]
            .into(),
        },
        "(a)",
    );
    t(
        &VarDepsSum {
            summands: [VarDepsSummand {
                base: "a",
                excludes: ["s"].into(),
            }]
            .into(),
        },
        "(a[s])",
    );
}

const EXCLUDES_N: usize = 4;
const SUMMANDS_N: usize = 4;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VarDepsSummand<A, S> {
    base: A,
    excludes: OrderedSmallVecSet<S, EXCLUDES_N>,
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

type VarDepsSummands<A, S> = OrderedSmallVecMap<VarDepsSummand<A, S>, SUMMANDS_N>;
