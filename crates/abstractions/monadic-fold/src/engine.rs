use sealed::sealed;

pub trait IsMonadicFoldEngineScheme {
    type Engine;
    type State;
    type Item;
    type Output;

    /// Expected to obtain multiple states from previous state `s` and current item `t`,
    /// and then apply `f` to  each of them in turn until `f` returns `AltJustOk` or `AltJustErr`.
    fn fold_step(
        engine: &mut Self::Engine,
        s: Self::State,
        t: Self::Item,
        f: impl FnMut(&mut Self::Engine, Self::State) -> Self::Output,
    ) -> Self::Output;
}

#[sealed]
pub trait IsMonadicFoldEngineSchemeFull: IsMonadicFoldEngineScheme {
    fn fold<I>(
        engine: &mut Self::Engine,
        initial_state: Self::State,
        iter: I,
        f: impl FnMut(&mut Self::Engine, Self::State) -> Self::Output,
    ) -> Self::Output
    where
        I: IntoIterator<Item = Self::Item>,
        I::IntoIter: Clone;
}

#[sealed]
impl<Scheme> IsMonadicFoldEngineSchemeFull for Scheme
where
    Scheme: IsMonadicFoldEngineScheme,
{
    fn fold<I>(
        engine: &mut Self::Engine,
        initial_state: Self::State,
        iter: I,
        mut f: impl FnMut(&mut Self::Engine, Self::State) -> Self::Output,
    ) -> Self::Output
    where
        I: IntoIterator<Item = Self::Item>,
        I::IntoIter: Clone,
    {
        fold_aux::<Self>(engine, initial_state, iter.into_iter(), &mut f)
    }
}

fn fold_aux<Scheme>(
    engine: &mut Scheme::Engine,
    s: Scheme::State,
    mut iter: impl Iterator<Item = Scheme::Item> + Clone,
    mut f: &mut impl FnMut(&mut Scheme::Engine, Scheme::State) -> Scheme::Output,
) -> Scheme::Output
where
    Scheme: IsMonadicFoldEngineScheme,
{
    let mut next = iter.next();
    match next {
        Some(t) => Scheme::fold_step(engine, s, t, |engine, s| {
            fold_aux::<Scheme>(engine, s, iter.clone(), f)
        }),
        None => f(engine, s),
    }
}

#[test]
fn monadic_fold_engine_scheme_works() {
    struct Engine {}

    struct Scheme;

    impl IsMonadicFoldEngineScheme for Scheme {
        type Engine = Engine;
        type State = String;
        type Item = char;
        type Output = Vec<String>;

        fn fold_step(
            engine: &mut Self::Engine,
            s: Self::State,
            t: Self::Item,
            mut f: impl FnMut(&mut Self::Engine, Self::State) -> Self::Output,
        ) -> Self::Output {
            let mut result = vec![];
            for c in [t.to_ascii_lowercase(), t.to_ascii_uppercase()] {
                result.extend(f(engine, format!("{s}{c}")));
            }
            result
        }
    }

    fn t(chars: &str) -> Vec<String> {
        Scheme::fold(&mut Engine {}, "".to_string(), chars.chars(), |_, s| {
            vec![s]
        })
    }

    assert_eq!(
        t("abc"),
        vec!["abc", "abC", "aBc", "aBC", "Abc", "AbC", "ABc", "ABC"]
    );
}
