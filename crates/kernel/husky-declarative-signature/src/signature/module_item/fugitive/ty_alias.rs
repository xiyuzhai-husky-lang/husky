use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAliasFugitiveDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
}

impl TypeAliasFugitiveDeclarativeSignatureTemplate {
    pub(super) fn from_decl(db: &dyn DeclarativeSignatureDb, decl: TypeAliasSynDecl) -> Self {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = declarative_term_region(db, syn_expr_region);
        let _declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = todo!();
        TypeAliasFugitiveDeclarativeSignatureTemplate::new(db, template_parameters)
    }
}
