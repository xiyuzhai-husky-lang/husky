use crate::*;
pub(crate) fn _foldm<Engine, S, I, R>(
    engine: &mut Engine,
    state: S,
    mut iter: I,
    f: &impl Fn(
        &mut Engine,
        S,
        I::Item,
        &dyn Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>,
    h: &dyn Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
) -> MiracleAltMaybeResult<R>
where
    I: Iterator + Clone,
{
    let Some(item) = iter.next() else {
        return h(engine, state);
    };
    f(engine, state, item, &|engine, state| {
        _foldm(engine, state, iter.clone(), f, h)
    })
}

pub fn foldm<'a, Engine, S, I, R>(
    init: S,
    iter: I,
    f: &'a impl Fn(
        &mut Engine,
        S,
        I::Item,
        &dyn Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>,
) -> impl FnOnce(
    &mut Engine,
    &dyn Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
) -> MiracleAltMaybeResult<R>
       + 'a
where
    S: 'a,
    I: IntoIterator + 'a,
    I::IntoIter: Clone,
{
    |engine, heuristic| crate::foldm::_foldm(engine, init, iter.into_iter(), f, heuristic)
}

pub fn mapm_collect<'a, Engine, S, A, I, MA, R>(
    iter: I,
    f: &'a impl Fn(&mut Engine, I::Item) -> MA,
) -> impl FnOnce(
    &mut Engine,
    &dyn Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
) -> MiracleAltMaybeResult<R>
       + 'a
where
    S: Default + Extend<A> + Clone + 'a,
    I: IntoIterator + 'a,
    I::IntoIter: Clone,
    MA: FnOnce(
        &mut Engine,
        &dyn Fn(&mut Engine, A) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>,
{
    |engine, heuristic| {
        crate::foldm::_foldm(
            engine,
            S::default(),
            iter.into_iter(),
            &|engine, state, item, heuristic| {
                let ma = f(engine, item);
                ma(engine, &move |engine: &mut Engine, a| {
                    let mut state = state.clone();
                    state.extend_one(a);
                    heuristic(engine, state)
                })
            },
            heuristic,
        )
    }
}
