use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeAssociatedValDeclarativeSignatureTemplate {}

impl TraitForTypeAssociatedValDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        decl: TraitForTypeAssociatedValSynDecl,
    ) -> DeclarativeSignatureResult<TraitForTypeAssociatedValDeclarativeSignatureTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = declarative_term_region(db, syn_expr_region);
        let _declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        Ok(TraitForTypeAssociatedValDeclarativeSignatureTemplate::new(
            db,
        ))
    }
}
