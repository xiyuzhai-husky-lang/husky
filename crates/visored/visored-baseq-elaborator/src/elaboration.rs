pub mod error;

use self::error::*;
use std::marker::PhantomData;

pub enum VdBsqElaboration<'sess> {
    PhantomData(PhantomData<&'sess ()>),
}

pub struct VdBsqElaborationTracker<'sess> {
    history: (),
    conclusion: Option<VdBsqElaborationResult<'sess, VdBsqElaboration<'sess>>>,
}
