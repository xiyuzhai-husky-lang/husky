use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn type_alias_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeAliasSynDecl,
) -> TypeAliasDeclarativeSignatureTemplate {
    let syn_expr_region = decl.syn_expr_region(db);
    let _declarative_term_region = declarative_term_region(db, syn_expr_region);
    let _declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    TypeAliasDeclarativeSignatureTemplate::new(db)
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAliasDeclarativeSignatureTemplate {}
