use super::*;
use husky_token_data::Keyword;
use husky_vfs::SubmodulePath;

impl TranspileToRust for SubmoduleHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.new_semicolon_line(|builder| {
            Keyword::Mod.transpile_to_rust(builder);
            self.hir_decl()
                .path(builder.db())
                .ident(builder.db())
                .transpile_to_rust(builder);
        })
    }
}
