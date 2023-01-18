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
    pub fn ast_idx(self, db: &dyn SignatureDb) -> AstIdx {
        match self {
            TypeAsTraitItemSignature::Function(decl) => decl.ast_idx(db),
            TypeAsTraitItemSignature::Method(decl) => decl.ast_idx(db),
            TypeAsTraitItemSignature::AlienType(decl) => decl.ast_idx(db),
            TypeAsTraitItemSignature::Value(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TypeAsTraitItemSignature::Function(_) => todo!(),
            TypeAsTraitItemSignature::Method(_) => todo!(),
            TypeAsTraitItemSignature::AlienType(_) => todo!(),
            TypeAsTraitItemSignature::Value(_) => todo!(),
        }
    }

    pub fn expr_page(self, db: &dyn SignatureDb) -> ExprPage {
        match self {
            TypeAsTraitItemSignature::Function(decl) => decl.expr_page(db),
            TypeAsTraitItemSignature::Method(decl) => decl.expr_page(db),
            TypeAsTraitItemSignature::AlienType(decl) => decl.expr_page(db),
            TypeAsTraitItemSignature::Value(decl) => decl.expr_page(db),
        }
    }

    pub fn path(self, db: &dyn SignatureDb) -> Option<TypeAsTraitItemPath> {
        match self {
            TypeAsTraitItemSignature::Function(_) => todo!(),
            TypeAsTraitItemSignature::Method(decl) => decl.path(db),
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
