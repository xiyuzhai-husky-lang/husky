use crate::*;
use elaborator::VdBsqElaboratorInner;
use visored_signature::signature::{separator::base::VdBaseSeparatorSignature, VdSignature};
use visored_term::ty::VdType;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn eq_signature(&self, ty: VdType) -> VdBaseSeparatorSignature {
        let ty_menu = self.ty_menu();
        if ty == ty_menu.nat {
            self.signature_menu().nat_eq
        } else {
            todo!()
        }
    }
}
