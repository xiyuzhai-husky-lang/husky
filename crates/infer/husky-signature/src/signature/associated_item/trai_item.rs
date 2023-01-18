mod assoc_ty;
mod assoc_val;
mod function;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use method::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TraitItemSignature {
    Function(TraitAssociatedFunctionSignature),
    Method(TraitMethodSignature),
    AlienType(TraitAssociatedTypeSignature),
    Value(TraitAssociatedValueSignature),
}

impl TraitItemSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TraitItemSignature::Function(_) => todo!(),
            TraitItemSignature::Method(_) => todo!(),
            TraitItemSignature::AlienType(_) => todo!(),
            TraitItemSignature::Value(_) => todo!(),
        }
    }

    pub fn expr_page(self, db: &dyn SignatureDb) -> ExprPage {
        match self {
            TraitItemSignature::Function(_) => todo!(),
            TraitItemSignature::Method(_) => todo!(),
            TraitItemSignature::AlienType(_) => todo!(),
            TraitItemSignature::Value(_) => todo!(),
        }
    }

    pub fn path(self, db: &dyn SignatureDb) -> TraitItemPath {
        match self {
            TraitItemSignature::Function(_) => todo!(),
            TraitItemSignature::Method(_) => todo!(),
            TraitItemSignature::AlienType(_) => todo!(),
            TraitItemSignature::Value(_) => todo!(),
        }
    }
}

impl<Db: SignatureDb + ?Sized> salsa::DebugWithDb<Db> for TraitItemSignature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
