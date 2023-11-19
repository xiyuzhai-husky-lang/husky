use super::*;
use husky_entity_path::AssociatedItemPath;

impl TranspileToRust<HirEagerExprRegion> for AssociatedItemPath {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db;
        match self {
            AssociatedItemPath::TypeItem(path) => {
                path.impl_block(db).ty_path(db).transpile_to_rust(builder)
            }
            AssociatedItemPath::TraitItem(_) => todo!(),
            AssociatedItemPath::TraitForTypeItem(path) => {
                todo!()
            }
        }
        builder.opr(RustOpr::ColonColon);
        self.ident(db).transpile_to_rust(builder)
    }
}
