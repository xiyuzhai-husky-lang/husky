use super::*;
use crate::builder::keyword::RustKeyword;
use husky_hir_defn::defn::submodule::SubmoduleHirDefn;

impl<E> TranspileToRustWith<E> for SubmoduleHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        builder.on_fresh_semicolon_line(|builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Mod);
            let db = builder.db();
            self.hir_decl()
                .path(db)
                .ident(db)
                .transpile_to_rust(builder);
        })
    }
}
