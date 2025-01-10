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
