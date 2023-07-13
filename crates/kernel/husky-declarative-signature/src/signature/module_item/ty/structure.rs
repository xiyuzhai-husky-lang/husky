use crate::*;

#[salsa::tracked(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct StructureDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: DeclarativeGenericParameters,
}

impl StructureDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        path: TypePath,
        decl: StructureTypeDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let expr_region = decl.expr_region(db);
        let declarative_term_region = declarative_term_region(db, expr_region);
        let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
        let implicit_parameters = DeclarativeGenericParameters::from_decl(
            decl.implicit_parameters(db),
            &declarative_term_region,
            declarative_term_menu,
        );
        Ok(Self::new(db, implicit_parameters))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct StructureTypeDeclarativeSignature {}
