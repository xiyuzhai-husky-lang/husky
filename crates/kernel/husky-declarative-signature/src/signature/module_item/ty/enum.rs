use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: DeclarativeGenericParameters,
    pub self_ty: DeclarativeTerm,
}

impl EnumDeclarativeSignatureTemplate {
    pub fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        path: TypePath,
        decl: EnumTypeDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let expr_region = decl.expr_region(db);
        let declarative_term_region = declarative_term_region(db, expr_region);
        let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
        let implicit_parameters = DeclarativeGenericParameters::from_decl(
            decl.implicit_parameters(db),
            &declarative_term_region,
            declarative_term_menu,
        );
        let self_ty = construct_self_ty(db, path, &implicit_parameters);
        Ok(Self::new(db, implicit_parameters, self_ty))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct EnumTypeDeclarativeSignature {}
