mod associated_ty;
mod associated_value;
mod function;
mod memo;
mod method;

pub use associated_ty::*;
pub use associated_value::*;
pub use function::*;
pub use memo::*;
pub use method::*;

use crate::*;

pub(crate) fn ty_associated_item_signature(
    db: &dyn SignatureDb,
    decl: TypeItemDecl,
) -> SignatureResultBorrowed<TypeItemSignature> {
    match decl {
        TypeItemDecl::Function(decl) => ty_associated_function_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeItemDecl::Method(decl) => ty_method_signature(db, decl).as_ref().map(|s| (*s).into()),
        TypeItemDecl::AlienType(decl) => ty_associated_ty_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeItemDecl::Value(decl) => ty_associated_value_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeItemDecl::Memo(decl) => ty_memo_signature(db, decl).as_ref().map(|s| (*s).into()),
        // ImplBlockDecl::TypeImplBlock(decl) => ty_impl_block_signature(db, decl).into(),
        // ImplBlockDecl::TypeAsTraitImplBlock(decl) => ty_as_trai_impl_block_signature(db, decl).into(),
        // TypeDecl::Union(decl) => union_ty_signature(db, decl).into(),
    }
}

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
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TypeItemSignature::Function(_) => todo!(),
            TypeItemSignature::Method(_) => todo!(),
            TypeItemSignature::AlienType(_) => todo!(),
            TypeItemSignature::Value(_) => todo!(),
            TypeItemSignature::Memo(_) => todo!(),
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
