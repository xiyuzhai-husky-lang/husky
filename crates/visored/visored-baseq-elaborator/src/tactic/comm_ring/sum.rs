use super::*;
use crate::term::inum::{sum::VdBsqSumInumTerm, VdBsqNonSumInumTerm};
use crate::term::{builder::sum::VdBsqSumBuilder, rnum::VdBsqRnumTerm};
use monadic_fold::engine::{IsMonadicFoldEngineScheme, IsMonadicFoldEngineSchemeFull as _};
use std::marker::PhantomData;

struct Scheme<'db, 'sess>(PhantomData<&'db ()>, PhantomData<&'sess ()>);

impl<'db, 'sess> IsMonadicFoldEngineScheme for Scheme<'db, 'sess>
where
    'db: 'sess,
{
    type Engine = VdBsqElaboratorInner<'db, 'sess>;

    type State = VdBsqSumBuilder<'sess>;

    type Item = (VdBsqRnumTerm, VdBsqNonSumInumTerm<'sess>);

    type Output = AltOption<VdBsqHypothesisIdx<'sess>>;

    fn fold_step(
        engine: &mut Self::Engine,
        s: Self::State,
        t: Self::Item,
        f: impl FnMut(&mut Self::Engine, Self::State) -> Self::Output,
    ) -> Self::Output {
        todo!()
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(super) fn comm_ring_fold_sum(
        &mut self,
        terms: &[(VdBsqRnumTerm, VdBsqNonSumInumTerm<'sess>)],
    ) -> AltOption<VdBsqHypothesisIdx<'sess>> {
        Scheme::fold(
            self,
            VdBsqSumBuilder::new(self.floater_db()),
            terms.iter().copied(),
            |elaborator, builder| todo!(),
        )
    }
}
