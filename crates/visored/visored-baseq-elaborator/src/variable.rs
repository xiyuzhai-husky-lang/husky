use crate::elaborator::VdBsqElaboratorInner;
use crate::*;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    // ad hoc
    pub fn eval_variable(&self) -> Option<()> {
        None
    }
}
