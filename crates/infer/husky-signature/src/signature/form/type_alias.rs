use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn type_alias_signature(db: &dyn SignatureDb, decl: TypeAliasDecl) -> TypeAliasSignature {
    // implementation
    TypeAliasSignature::new(db)
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAliasSignature {}
