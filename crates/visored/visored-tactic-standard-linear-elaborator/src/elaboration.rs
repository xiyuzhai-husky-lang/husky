pub mod error;

use self::error::*;
use std::marker::PhantomData;

pub enum VdMirTacticStandardLinearElaboration<'sess> {
    PhantomData(PhantomData<&'sess ()>),
}

pub struct VdMirTacticStandardLinearElaborationTracker<'sess> {
    history: (),
    conclusion: Option<
        VdMirTacticStandardLinearElaborationResult<
            'sess,
            VdMirTacticStandardLinearElaboration<'sess>,
        >,
    >,
}
