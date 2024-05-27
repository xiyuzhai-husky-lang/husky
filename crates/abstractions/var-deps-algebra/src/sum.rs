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

impl<A, S> VarDepsSum<A, S>
where
    A: Copy + Ord + std::fmt::Debug,
    S: Copy + Ord,
{
    pub fn union(&self, other: &Self) -> Self {
        let mut summands = self.summands.clone();
        for summand in &other.summands {
            summands.insert_with_or_update(
                summand.base,
                || summand.clone(),
                |existing_summand| {
                    existing_summand.excludes =
                        existing_summand.excludes.interset(&summand.excludes)
                },
            )
        }
        Self { summands }
    }
}

#[test]
fn var_deps_sum_union_works() {
    fn t(
        a: Vec<(&'static str, Vec<&'static str>)>,
        b: Vec<(&'static str, Vec<&'static str>)>,
        a_str: &str,
        b_str: &str,
        expected: &str,
    ) {
        let a: VarDepsSum0 = a.into();
        let b: VarDepsSum0 = b.into();
        assert_eq!(a.to_string(), a_str);
        assert_eq!(b.to_string(), b_str);
        assert_eq!(a.union(&b).to_string(), expected);
    }

    t(vec![], vec![], "()", "()", "()");
    t(vec![("a", vec![])], vec![], "(a)", "()", "(a)");
    t(
        vec![("a", vec![])],
        vec![("b", vec![])],
        "(a)",
        "(b)",
        "(a, b)",
    );
    t(
        vec![("a", vec![])],
        vec![("a", vec![])],
        "(a)",
        "(a)",
        "(a)",
    );
    t(
        vec![("a", vec!["s"])],
        vec![("a", vec![])],
        "(a[s])",
        "(a)",
        "(a)",
    );
    t(
        vec![("a", vec!["r", "s"])],
        vec![("a", vec!["s", "t"])],
        "(a[r,s])",
        "(a[s,t])",
        "(a[s])",
    );
    t(
        vec![("a", vec!["r", "s"]), ("b", vec!["r"])],
        vec![("a", vec!["s", "t"])],
        "(a[r,s], b[r])",
        "(a[s,t])",
        "(a[s], b[r])",
    );
}
