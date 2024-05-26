#[cfg(test)]
use super::*;
#[cfg(test)]
use crate::summand::VarDepsSummand;
use crate::summand::VarDepsSummands;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VarDepsSum<A, S> {
    summands: VarDepsSummands<A, S>,
}

impl<A, S> From<Vec<(A, Vec<S>)>> for VarDepsSum<A, S>
where
    A: Ord + Copy + std::fmt::Debug,
    S: Ord + Copy + std::fmt::Debug,
{
    fn from(summands: Vec<(A, Vec<S>)>) -> Self {
        Self {
            summands: summands.into_iter().map(Into::into).collect(),
        }
    }
}

impl<A, S> std::fmt::Display for VarDepsSum<A, S>
where
    A: std::fmt::Display,
    S: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn t(sum: Vec<(&'static str, Vec<&'static str>)>, expected: &str) {
        let sum: VarDepsSum0 = sum.into();
        assert_eq!(sum.to_string(), expected);
    }
    t(vec![], "()");
    t(vec![("a", vec![])], "(a)");
    t(vec![("a", vec!["s"])], "(a[s])");
}
