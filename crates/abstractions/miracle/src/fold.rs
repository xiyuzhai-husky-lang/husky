use crate::{HasMiracleFull, MiracleAltMaybeResult};

pub trait IsMiracleFoldEngine<T, S>: HasMiracleFull {
    /// Expected to obtain multiple states from previous state `s` and current item `t`,
    /// and then apply `f` to  each of them in turn until `f` returns `AltJustOk` or `AltJustErr`.
    fn fold_step<R>(
        &mut self,
        s: S,
        t: T,
        f: impl FnMut(&mut Self, S) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>;
}

pub trait IsMiracleFoldEngineFull<T, S>: HasMiracleFull {
    fn fold<I, R>(
        &mut self,
        initial_state: S,
        iter: I,
        f: impl FnMut(&mut Self, S) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: Clone;
}

impl<T, S> IsMiracleFoldEngineFull<T, S> for T
where
    T: IsMiracleFoldEngine<T, S>,
{
    fn fold<I, R>(
        &mut self,
        initial_state: S,
        iter: I,
        f: impl FnMut(&mut Self, S) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: Clone,
    {
        fold_aux(self, initial_state, iter.into_iter(), f)
    }
}

fn fold_aux<Engine, T, S, R>(
    engine: &mut Engine,
    s: S,
    mut iter: impl Iterator<Item = T> + Clone,
    mut f: impl FnMut(&mut Engine, S) -> MiracleAltMaybeResult<R>,
) -> MiracleAltMaybeResult<R>
where
    Engine: IsMiracleFoldEngine<T, S>,
{
    let mut next = iter.next();
    match next {
        Some(t) => engine.fold_step(s, t, |engine, s| fold_aux(engine, s, iter.clone(), &mut f)),
        None => f(engine, s),
    }
}
