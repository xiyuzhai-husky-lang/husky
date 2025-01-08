use crate::*;

pub(crate) fn foldm_batch<Engine, S, I, R>(
    engine: &mut Engine,
    state: S,
    mut iter: impl Iterator<Item = I> + Clone,
    fs: &[impl Fn(&mut Engine, &S, &I) -> Option<S>],
    g: &impl Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
) -> MiracleAltMaybeResult<R>
where
    Engine: HasMiracleFull,
{
    let Some(item) = iter.next() else {
        return g(engine, state);
    };
    let fs = fs
        .iter()
        .map(|f| {
            |engine: &mut Engine| -> MiracleAltMaybeResult<R> {
                let Some(state) = f(engine, &state, &item) else {
                    return AltNothing;
                };
                foldm_batch(engine, state, iter.clone(), fs, g)
            }
        })
        .collect::<Vec<_>>();
    engine.exec_batch2(&fs)
}
