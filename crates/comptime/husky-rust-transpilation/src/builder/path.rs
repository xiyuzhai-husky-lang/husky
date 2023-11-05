use super::*;
use husky_entity_path::AssociatedItemPath;

impl TranspileToRust for AssociatedItemPath {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db;
        self.ident(db).transpile_to_rust(builder)
    }
}
