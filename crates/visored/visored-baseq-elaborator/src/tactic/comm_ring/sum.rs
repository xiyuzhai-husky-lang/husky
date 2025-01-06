use crate::term::inum::{sum::VdBsqSumInumTerm, VdBsqNonSumInumTerm};

use super::*;
use monadic_fold::engine::IsMonadicFoldEngineScheme;
use std::marker::PhantomData;

struct Scheme<'db, 'sess>(PhantomData<&'db ()>, PhantomData<&'sess ()>);

impl<'db, 'sess> IsMonadicFoldEngineScheme for Scheme<'db, 'sess>
where
    'db: 'sess,
{
    type Engine = VdBsqElaboratorInner<'db, 'sess>;

    type State = VdBsqSumInumTerm<'sess>;

    type Item = VdBsqNonSumInumTerm<'sess>;

    type Output = VdBsqSumInumTerm<'sess>;

    fn fold_step(
        engine: &mut Self::Engine,
        s: Self::State,
        t: Self::Item,
        f: impl FnMut(&mut Self::Engine, Self::State) -> Self::Output,
    ) -> Self::Output {
        todo!()
    }
}
