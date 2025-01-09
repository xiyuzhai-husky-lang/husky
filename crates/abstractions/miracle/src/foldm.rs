use crate::*;

pub(crate) fn foldm<Engine, S, I, R, H>(
    engine: &mut Engine,
    state: S,
    mut iter: I,
    h: &H,
    f: &impl Fn(
        &mut Engine,
        S,
        I::Item,
        &dyn Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>,
) -> MiracleAltMaybeResult<R>
where
    I: Iterator + Clone,
    H: Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
{
    let Some(item) = iter.next() else {
        return h(engine, state);
    };
    f(engine, state, item, &|engine, state| {
        foldm(engine, state, iter.clone(), h, f)
    })
}
