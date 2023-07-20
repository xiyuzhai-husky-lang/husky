use crate::*;

#[salsa::tracked(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct StructureDeclarativeSignatureTemplate {
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameterTemplates,
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
        let generic_parameters = DeclarativeGenericParameterTemplates::from_decl(
            decl.generic_parameters(db),
            &declarative_term_region,
            declarative_term_menu,
        );
        Ok(Self::new(db, generic_parameters))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct StructureTypeDeclarativeSignature {}
