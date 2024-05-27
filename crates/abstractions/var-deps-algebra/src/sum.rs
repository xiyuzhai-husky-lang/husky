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

impl<A, S> Default for VarDepsSum<A, S> {
    fn default() -> Self {
        Self {
            summands: Default::default(),
        }
    }
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

impl<A, S> From<&[(A, &[S])]> for VarDepsSum<A, S>
where
    A: Ord + Copy + std::fmt::Debug,
    S: Ord + Copy + std::fmt::Debug,
{
    fn from(summands: &[(A, &[S])]) -> Self {
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
    fn t(sum: &[(&'static str, &[&'static str])], expected: &str) {
        let sum: VarDepsSum0 = sum.into();
        assert_eq!(sum.to_string(), expected);
    }
    t(&[], "()");
    t(&[("a", &[])], "(a)");
    t(&[("a", &["s"])], "(a[s])");
}

impl<A, S> VarDepsSum<A, S>
where
    A: Copy + Ord + std::fmt::Debug,
    S: Copy + Ord,
{
    pub fn union(self, other: &Self) -> Self {
        let mut summands = self.summands;
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

    pub fn exclude(&self, excludes: &[S]) -> Self {
        let summands = self.summands.map_collect_on_entries(|summand| {
            let mut summand = summand.clone();
            summand.excludes.extend(excludes);
            summand
        });
        Self { summands }
    }

    pub fn rewrite<'a>(&self, f: impl Fn(A) -> &'a Self) -> Self
    where
        A: 'a,
        S: 'a,
    {
        let mut result = Self::default();
        for summand in &self.summands {
            result = result.union(&f(summand.base).exclude(&summand.excludes));
        }
        result
    }
}

#[test]
fn var_deps_sum_union_works() {
    fn t(
        a: &[(&'static str, &[&'static str])],
        b: &[(&'static str, &[&'static str])],
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

    t(&[], &[], "()", "()", "()");
    t(&[("a", &[])], &[], "(a)", "()", "(a)");
    t(&[("a", &[])], &[("b", &[])], "(a)", "(b)", "(a, b)");
    t(&[("a", &[])], &[("a", &[])], "(a)", "(a)", "(a)");
    t(&[("a", &["s"])], &[("a", &[])], "(a[s])", "(a)", "(a)");
    t(
        &[("a", &["r", "s"])],
        &[("a", &["s", "t"])],
        "(a[r,s])",
        "(a[s,t])",
        "(a[s])",
    );
    t(
        &[("a", &["r", "s"]), ("b", &["r"])],
        &[("a", &["s", "t"])],
        "(a[r,s], b[r])",
        "(a[s,t])",
        "(a[s], b[r])",
    );
}

#[test]
fn var_deps_sum_excludes_works() {
    fn t(sum: &[(A, &[S])], sum_str: &str, excludes: &[S], expected: &str) {
        let sum: VarDepsSum0 = sum.into();
        assert_eq!(sum.to_string(), sum_str);
        assert_eq!(sum.exclude(excludes).to_string(), expected);
    }

    t(&[], "()", &[], "()");
    t(&[], "()", &["r", "s", "t"], "()");
    t(&[("a", &[])], "(a)", &["r", "s", "t"], "(a[r,s,t])");
    t(&[("a", &["r"])], "(a[r])", &["s", "t"], "(a[r,s,t])");
    t(
        &[("a", &["r", "t"])],
        "(a[r,t])",
        &["r", "s", "t"],
        "(a[r,s,t])",
    );
}

#[test]
fn var_deps_sum_rewrite_works() {
    fn t(sum: &[(A, &[S])], sum_str: &str, substitutes: &[(A, &[(A, &[S])])], expected: &str) {
        let sum: VarDepsSum0 = sum.into();
        let substitutes: Vec<(A, VarDepsSum0)> = substitutes
            .iter()
            .map(|&(base, substitute)| (base, substitute.into()))
            .collect();
        assert_eq!(sum.to_string(), sum_str);
        assert_eq!(
            sum.rewrite(|base| substitutes
                .iter()
                .find_map(|&(base0, ref substitute)| (base0 == base).then_some(substitute))
                .unwrap())
                .to_string(),
            expected
        );
    }

    t(&[], "()", &[], "()");
    t(&[], "()", &[("a", &[("a", &["r", "s", "t"])])], "()");
    t(
        &[("a", &[])],
        "(a)",
        &[("a", &[("a", &["r", "s", "t"])])],
        "(a[r,s,t])",
    );
    t(
        &[("a", &[])],
        "(a)",
        &[("a", &[("b", &["r", "s", "t"])])],
        "(b[r,s,t])",
    );
    t(
        &[("a", &["s", "t"])],
        "(a[s,t])",
        &[("a", &[("b", &["r", "s"])])],
        "(b[r,s,t])",
    );
}
