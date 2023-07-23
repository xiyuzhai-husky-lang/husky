use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumDeclarativeSignatureTemplate {
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameterTemplates,
    pub self_ty: DeclarativeTerm,
}

impl EnumDeclarativeSignatureTemplate {
    pub fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        path: TypePath,
        decl: EnumTypeSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let expr_region = decl.expr_region(db);
        let declarative_term_region = declarative_term_region(db, expr_region);
        let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
        let generic_parameters = DeclarativeGenericParameterTemplates::from_decl(
            decl.generic_parameters(db),
            &declarative_term_region,
            declarative_term_menu,
        );
        let self_ty = construct_self_ty(db, path, &generic_parameters);
        Ok(Self::new(db, generic_parameters, self_ty))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct EnumTypeDeclarativeSignature {}
