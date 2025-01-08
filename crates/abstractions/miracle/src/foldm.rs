use crate::*;

pub(crate) fn foldm_aux<Engine, S, I, F, R>(
    engine: &mut Engine,
    state: S,
    mut iter: impl Iterator<Item = I> + Clone,
    fs: &[F],
    g: &impl Fn(&mut Engine, S) -> MiracleAltMaybeResult<R>,
) -> MiracleAltMaybeResult<R>
where
    Engine: HasMiracleFull,
    F: Fn(&mut Engine, &S, &I) -> Option<S>,
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
                foldm_aux(engine, state, iter.clone(), fs, g)
            }
        })
        .collect::<Vec<_>>();
    engine.exec_batch2(&fs)
}
