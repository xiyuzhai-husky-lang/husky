use crate::*;

pub(crate) fn foldm_aux<Engine, S, I, R>(
    engine: &mut Engine,
    state: &S,
    mut iter: impl Iterator<Item = I> + Clone,
    fs: &[&dyn Fn(&mut Engine, &S, &I) -> S],
    g: &impl Fn(&mut Engine, &S) -> MiracleAltMaybeResult<R>,
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
                let state = f(engine, state, &item);
                foldm_aux(engine, &state, iter.clone(), fs, g)
            }
        })
        .collect::<Vec<_>>();
    engine.exec_batch2(&fs)
}
