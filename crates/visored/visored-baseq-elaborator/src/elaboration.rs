pub mod error;

use self::error::*;
use std::marker::PhantomData;

pub enum VdBaseqElaboration<'sess> {
    PhantomData(PhantomData<&'sess ()>),
}

pub struct VdBaseqElaborationTracker<'sess> {
    history: (),
    conclusion: Option<VdBaseqElaborationResult<'sess, VdBaseqElaboration<'sess>>>,
}
