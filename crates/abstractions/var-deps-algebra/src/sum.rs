#[cfg(test)]
use super::*;
#[cfg(test)]
use crate::summand::VarDepsSummand;
use crate::summand::VarDepsSummands;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarDepsSum<A, S> {
    summands: VarDepsSummands<A, S>,
    includes: VarDepsIncludes<S>,
}

pub type VarDepsIncludes<S> = OrderedSmallVecSet<S, INCLUDES_N>;
const INCLUDES_N: usize = 4;

impl<A, S> Default for VarDepsSum<A, S> {
    fn default() -> Self {
        Self {
            summands: Default::default(),
            includes: Default::default(),
        }
    }
}

impl<A, S> From<(Vec<(A, Vec<S>)>, Vec<S>)> for VarDepsSum<A, S>
where
    A: Ord + Copy + std::fmt::Debug,
    S: Ord + Copy + std::fmt::Debug,
{
    fn from((summands, includes): (Vec<(A, Vec<S>)>, Vec<S>)) -> Self {
        Self {
            summands: summands.into_iter().map(Into::into).collect(),
            includes: includes.into(),
        }
    }
}

impl<A, S> From<(&[(A, &[S])], &[S])> for VarDepsSum<A, S>
where
    A: Ord + Copy + std::fmt::Debug,
    S: Ord + Copy + std::fmt::Debug,
{
    fn from((summands, includes): (&[(A, &[S])], &[S])) -> Self {
        Self {
            summands: summands.into_iter().map(Into::into).collect(),
            includes: includes.iter().copied().collect(),
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
        for (i, include) in self.includes.iter().enumerate() {
            if self.summands.len() + i > 0 {
                f.write_str(", ")?;
            }
            include.fmt(f)?
        }
        f.write_str(")")
    }
}

#[test]
fn var_deps_sum_display_works() {
    fn t(sum: (&[(&'static str, &[&'static str])], &[&'static str]), expected: &str) {
        let sum: VarDepsSum0 = sum.into();
        assert_eq!(sum.to_string(), expected);
    }
    t((&[], &[]), "()");
    t((&[("a", &[])], &[]), "(a)");
    t((&[("a", &["s"])], &[]), "(a[s])");
    t((&[], &["s"]), "(s)");
    t((&[("a", &[])], &["s"]), "(a, s)");
    t((&[("a", &["s"])], &["s"]), "(a[s], s)");
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
        let includes = self.includes.union(&other.includes);
        Self { summands, includes }
    }

    pub fn exclude(self, excludes: &[S]) -> Self {
        let Self {
            summands,
            mut includes,
        } = self;
        let summands = summands.map_into_collect_on_entries(|mut summand| {
            summand.excludes.extend(excludes);
            summand
        });
        includes.remove_from_list(excludes);
        Self { summands, includes }
    }

    pub fn rewrite<'a>(&self, f: impl Fn(A) -> &'a Self) -> Self
    where
        A: 'a,
        S: 'a,
    {
        let mut result = Self::default();
        for summand in &self.summands {
            result = result.union(&f(summand.base).clone().exclude(&summand.excludes));
        }
        result
    }

    pub fn eval<'a>(&self, f: impl Fn(A) -> OrderedSmallVecSet<S, 4>) -> OrderedSmallVecSet<S, 4>
    where
        A: 'a,
        S: 'a + Copy,
    {
        let mut result: OrderedSmallVecSet<S, 4> = Default::default();
        for summand in &self.summands {
            let mut deps = f(summand.base);
            for &exclude in &summand.excludes {
                deps.remove(exclude)
            }
            result.extend(&deps)
        }
        result
    }
}

#[test]
fn var_deps_sum_union_works() {
    fn t(
        a: (&[(&'static str, &[&'static str])], &[&'static str]),
        b: (&[(&'static str, &[&'static str])], &[&'static str]),
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

    t((&[], &[]), (&[], &[]), "()", "()", "()");
    t((&[], &["s"]), (&[], &["s"]), "(s)", "(s)", "(s)");
    t((&[], &["s"]), (&[], &["t"]), "(s)", "(t)", "(s, t)");
    t((&[("a", &[])], &[]), (&[], &[]), "(a)", "()", "(a)");
    t(
        (&[("a", &[])], &["s"]),
        (&[], &["s"]),
        "(a, s)",
        "(s)",
        "(a, s)",
    );
    t(
        (&[("a", &[])], &["s"]),
        (&[], &["t"]),
        "(a, s)",
        "(t)",
        "(a, s, t)",
    );
    t(
        (&[("a", &[])], &[]),
        (&[("b", &[])], &[]),
        "(a)",
        "(b)",
        "(a, b)",
    );
    t(
        (&[("a", &[])], &[]),
        (&[("a", &[])], &[]),
        "(a)",
        "(a)",
        "(a)",
    );
    t(
        (&[("a", &["s"])], &[]),
        (&[("a", &[])], &[]),
        "(a[s])",
        "(a)",
        "(a)",
    );
    t(
        (&[("a", &["r", "s"])], &[]),
        (&[("a", &["s", "t"])], &[]),
        "(a[r,s])",
        "(a[s,t])",
        "(a[s])",
    );
    t(
        (&[("a", &["r", "s"]), ("b", &["r"])], &[]),
        (&[("a", &["s", "t"])], &[]),
        "(a[r,s], b[r])",
        "(a[s,t])",
        "(a[s], b[r])",
    );
}

#[test]
fn var_deps_sum_excludes_works() {
    fn t(sum: (&[(A, &[S])], &[S]), sum_str: &str, excludes: &[S], expected: &str) {
        let sum: VarDepsSum0 = sum.into();
        assert_eq!(sum.to_string(), sum_str);
        assert_eq!(sum.exclude(excludes).to_string(), expected);
    }

    t((&[], &[]), "()", &[], "()");
    t((&[], &["s"]), "(s)", &[], "(s)");
    t((&[], &["s"]), "(s)", &["s"], "()");
    t((&[], &["s"]), "(s)", &["t"], "(s)");
    t((&[], &[]), "()", &["r", "s", "t"], "()");
    t((&[("a", &[])], &[]), "(a)", &["r", "s", "t"], "(a[r,s,t])");
    t((&[("a", &["r"])], &[]), "(a[r])", &["s", "t"], "(a[r,s,t])");
    t(
        (&[("a", &["r", "t"])], &[]),
        "(a[r,t])",
        &["r", "s", "t"],
        "(a[r,s,t])",
    );
}

#[test]
fn var_deps_sum_rewrite_works() {
    fn t(
        sum: (&[(A, &[S])], &[S]),
        sum_str: &str,
        substitutes: &[(A, (&[(A, &[S])], &[S]))],
        expected: &str,
    ) {
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

    t((&[], &[]), "()", &[], "()");
    t(
        (&[], &[]),
        "()",
        &[("a", (&[("a", &["r", "s", "t"])], &[]))],
        "()",
    );
    t(
        (&[("a", &[])], &[]),
        "(a)",
        &[("a", (&[("a", &["r", "s", "t"])], &[]))],
        "(a[r,s,t])",
    );
    t(
        (&[("a", &[])], &[]),
        "(a)",
        &[("a", (&[("b", &["r", "s", "t"])], &[]))],
        "(b[r,s,t])",
    );
    t(
        (&[("a", &["s", "t"])], &[]),
        "(a[s,t])",
        &[("a", (&[("b", &["r", "s"])], &[]))],
        "(b[r,s,t])",
    );
}

#[test]
fn var_deps_sum_eval_works() {
    fn t(sum: (&[(A, &[S])], &[S]), sum_str: &str, deps_values: &[(A, &[S])], expected: &[S]) {
        let sum: VarDepsSum0 = sum.into();
        let substitutes: Vec<(A, OrderedSmallVecSet<S, 4>)> = deps_values
            .iter()
            .map(|&(base, substitute)| (base, substitute.iter().copied().collect()))
            .collect();
        assert_eq!(sum.to_string(), sum_str);
        assert_eq!(
            &sum.eval(|base| substitutes
                .iter()
                .find_map(|&(base0, ref substitute)| (base0 == base).then_some(substitute.clone()))
                .unwrap()) as &[_],
            expected
        );
    }

    t((&[], &[]), "()", &[], &[]);
    t((&[], &[]), "()", &[("a", &["r", "s", "t"])], &[]);
    t(
        (&[("a", &[])], &[]),
        "(a)",
        &[("a", &["r", "s", "t"])],
        &["r", "s", "t"],
    );
    t(
        (&[("a", &["r"])], &[]),
        "(a[r])",
        &[("a", &["r", "s", "t"])],
        &["s", "t"],
    );
    t(
        (&[("a", &["r", "s"])], &[]),
        "(a[r,s])",
        &[("a", &["r", "s", "t"])],
        &["t"],
    );
}
