use crate::*;

pub(crate) fn multifold<Engine, S, I, R>(
    engine: &mut Engine,
    state: S,
    mut iter: impl Iterator<Item = I> + Clone,
    fs: &[impl Fn(&mut Engine, &S, &I) -> Option<S>],
    h: &dyn Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
) -> MiracleAltMaybeResult<R>
where
    Engine: HasMiracleFull,
{
    let Some(item) = iter.next() else {
        return h(engine, state);
    };
    let fs = fs
        .iter()
        .map(|f| {
            |engine: &mut Engine| -> MiracleAltMaybeResult<R> {
                let Some(state) = f(engine, &state, &item) else {
                    return AltNothing;
                };
                multifold(engine, state, iter.clone(), fs, h)
            }
        })
        .collect::<Vec<_>>();
    engine.exec_batch2(&fs)
}

pub fn multifold2<'a, Engine, S, I, R>(
    init: S,
    iter: I,
    f: &'a [impl Fn(&mut Engine, &S, &I::Item) -> Option<S>],
) -> impl FnOnce(
    &mut Engine,
    &dyn Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
) -> MiracleAltMaybeResult<R>
       + 'a
where
    I: IntoIterator,
    I::IntoIter: Clone,
    S: 'a,
    I: 'a,
    Engine: HasMiracleFull,
{
    |slf, heuristic| multifold(slf, init, iter.into_iter(), f, heuristic)
}

pub trait Multifold {
    type Item;

    fn multifold<'a, Engine, S, R>(
        self,
        init: S,
        f: &'a [impl Fn(&mut Engine, &S, &Self::Item) -> Option<S>],
    ) -> impl FnOnce(
        &mut Engine,
        &dyn Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>
           + 'a
    where
        Self: 'a,
        S: 'a,
        Engine: HasMiracleFull;
}

impl<I: IntoIterator + Clone> Multifold for I
where
    I::IntoIter: Clone,
{
    type Item = I::Item;

    fn multifold<'a, Engine, S, R>(
        self,
        init: S,
        f: &'a [impl Fn(&mut Engine, &S, &Self::Item) -> Option<S>],
    ) -> impl FnOnce(
        &mut Engine,
        &dyn Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>
           + 'a
    where
        Self: 'a,
        S: 'a,
        Engine: HasMiracleFull,
    {
        |slf, heuristic| multifold(slf, init, self.into_iter(), f, heuristic)
    }
}
