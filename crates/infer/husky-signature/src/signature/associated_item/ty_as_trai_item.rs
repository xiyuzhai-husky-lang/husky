mod associated_ty;
mod associated_value;
mod function;
mod method;

pub use associated_ty::*;
pub use associated_value::*;
pub use function::*;
pub use method::*;

use super::*;

pub(crate) fn ty_as_trai_associated_item_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitItemDecl,
) -> SignatureOutcomeBorrowed<TypeAsTraitItemSignature> {
    match decl {
        TypeAsTraitItemDecl::Function(decl) => {
            ty_as_trai_associated_function_signature(db, decl).ok_copy_into_abort_as_ref()
        }
        TypeAsTraitItemDecl::Method(decl) => {
            ty_as_trai_method_signature(db, decl).ok_copy_into_abort_as_ref()
        }
        TypeAsTraitItemDecl::AlienType(decl) => {
            ty_as_trai_associated_ty_signature(db, decl).ok_copy_into_abort_as_ref()
        }
        TypeAsTraitItemDecl::Value(decl) => {
            ty_as_trai_associated_value_signature(db, decl).ok_copy_into_abort_as_ref()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeAsTraitItemSignature {
    Function(TypeAsTraitAssociatedFunctionSignature),
    Method(TypeAsTraitMethodSignature),
    AlienType(TypeAsTraitAssociatedTypeSignature),
    Value(TypeAsTraitAssociatedValueSignature),
}

impl From<TypeAsTraitAssociatedValueSignature> for TypeAsTraitItemSignature {
    fn from(v: TypeAsTraitAssociatedValueSignature) -> Self {
        Self::Value(v)
    }
}

impl From<TypeAsTraitAssociatedTypeSignature> for TypeAsTraitItemSignature {
    fn from(v: TypeAsTraitAssociatedTypeSignature) -> Self {
        Self::AlienType(v)
    }
}

impl From<TypeAsTraitMethodSignature> for TypeAsTraitItemSignature {
    fn from(v: TypeAsTraitMethodSignature) -> Self {
        Self::Method(v)
    }
}

impl From<TypeAsTraitAssociatedFunctionSignature> for TypeAsTraitItemSignature {
    fn from(v: TypeAsTraitAssociatedFunctionSignature) -> Self {
        Self::Function(v)
    }
}

impl TypeAsTraitItemSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TypeAsTraitItemSignature::Function(_) => todo!(),
            TypeAsTraitItemSignature::Method(_) => todo!(),
            TypeAsTraitItemSignature::AlienType(_) => todo!(),
            TypeAsTraitItemSignature::Value(_) => todo!(),
        }
    }
}

impl<Db: SignatureDb + ?Sized> salsa::DebugWithDb<Db> for TypeAsTraitItemSignature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<SignatureJar>>::as_jar_db(db);
        match self {
            TypeAsTraitItemSignature::Function(decl) => f
                .debug_tuple("Function")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeAsTraitItemSignature::Method(decl) => f
                .debug_tuple("Method")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeAsTraitItemSignature::AlienType(decl) => f
                .debug_tuple("AlienType")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeAsTraitItemSignature::Value(decl) => f
                .debug_tuple("Value")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
