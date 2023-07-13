use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct ExternDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: DeclarativeGenericParameters,
}

impl ExternDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        path: TypePath,
        decl: ExternTypeDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let expr_region = decl.expr_region(db);
        let declarative_term_region = declarative_term_region(db, expr_region);
        let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
        Ok(Self::new(
            db,
            DeclarativeGenericParameters::from_decl(
                decl.implicit_parameters(db),
                &declarative_term_region,
                declarative_term_menu,
            ),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct ExternTypeDeclarativeSignature {}
