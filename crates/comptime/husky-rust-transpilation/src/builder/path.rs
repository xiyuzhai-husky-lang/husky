use super::*;
use husky_entity_path::AssociatedItemPath;

impl TranspileToRust<HirEagerExprRegion> for AssociatedItemPath {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db;
        self.ident(db).transpile_to_rust(builder)
    }
}
