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
