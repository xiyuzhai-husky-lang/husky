mod assoc_ty;
mod assoc_val;
mod function;
mod memo;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use memo::*;
pub use method::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeItemSignature {
    Function(TypeAssociatedFunctionSignature),
    Method(TypeMethodSignature),
    AlienType(TypeAssociatedTypeSignature),
    Value(TypeAssociatedValueSignature),
    Memo(TypeMemoSignature),
}

impl From<TypeMemoSignature> for TypeItemSignature {
    fn from(v: TypeMemoSignature) -> Self {
        Self::Memo(v)
    }
}

impl From<TypeAssociatedFunctionSignature> for TypeItemSignature {
    fn from(v: TypeAssociatedFunctionSignature) -> Self {
        Self::Function(v)
    }
}

impl From<TypeAssociatedValueSignature> for TypeItemSignature {
    fn from(v: TypeAssociatedValueSignature) -> Self {
        Self::Value(v)
    }
}

impl From<TypeAssociatedTypeSignature> for TypeItemSignature {
    fn from(v: TypeAssociatedTypeSignature) -> Self {
        Self::AlienType(v)
    }
}

impl From<TypeMethodSignature> for TypeItemSignature {
    fn from(v: TypeMethodSignature) -> Self {
        Self::Method(v)
    }
}

impl TypeItemSignature {
    pub fn ast_idx(self, db: &dyn SignatureDb) -> AstIdx {
        match self {
            TypeItemSignature::Function(decl) => decl.ast_idx(db),
            TypeItemSignature::Method(decl) => decl.ast_idx(db),
            TypeItemSignature::AlienType(decl) => decl.ast_idx(db),
            TypeItemSignature::Value(decl) => decl.ast_idx(db),
            TypeItemSignature::Memo(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TypeItemSignature::Function(_) => todo!(),
            TypeItemSignature::Method(_) => todo!(),
            TypeItemSignature::AlienType(_) => todo!(),
            TypeItemSignature::Value(_) => todo!(),
            TypeItemSignature::Memo(_) => todo!(),
        }
    }

    pub fn expr_page(self, db: &dyn SignatureDb) -> ExprPage {
        match self {
            TypeItemSignature::Function(defn) => defn.expr_page(db),
            TypeItemSignature::Method(defn) => defn.expr_page(db),
            TypeItemSignature::AlienType(defn) => defn.expr_page(db),
            TypeItemSignature::Value(defn) => defn.expr_page(db),
            TypeItemSignature::Memo(defn) => defn.expr_page(db),
        }
    }

    pub fn path(self, db: &dyn SignatureDb) -> Option<TypeItemPath> {
        match self {
            TypeItemSignature::Function(_) => todo!(),
            TypeItemSignature::Method(defn) => defn.path(db),
            TypeItemSignature::AlienType(_) => todo!(),
            TypeItemSignature::Value(_) => todo!(),
            TypeItemSignature::Memo(defn) => defn.path(db),
        }
    }
}

impl<Db: SignatureDb + ?Sized> salsa::DebugWithDb<Db> for TypeItemSignature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<SignatureJar>>::as_jar_db(db);
        match self {
            TypeItemSignature::Function(decl) => f
                .debug_tuple("Function")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeItemSignature::Method(decl) => f
                .debug_tuple("Method")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeItemSignature::AlienType(decl) => f
                .debug_tuple("AlienType")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeItemSignature::Value(decl) => f
                .debug_tuple("Value")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeItemSignature::Memo(decl) => f
                .debug_tuple("Memo")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
