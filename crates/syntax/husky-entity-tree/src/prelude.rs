use crate::*;

#[salsa::tracked(jar = EntitySymbolJar, return_ref)]
pub(crate) fn crate_specific_prelude(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<VecMap<EntityNode>> {
    let package_path = crate_path.package_path(db);
    let package_dependencies = db.package_dependencies(package_path)?;
    let mut nodes: VecMap<EntityNode> = VecMap::default();
    let crate_word = db.word_menu().crate_word();
    let crate_root = ModulePath::new_root(db, crate_path);
    nodes.insert(EntityNode::Module {
        ident: crate_word,
        accessibility: Accessibility::PubicUnder(crate_root),
        module_path: ModulePath::new_root(db, crate_path),
    });
    nodes.extend(
        package_dependencies
            .iter()
            .map(|package_dependency| todo!()),
    );
    Ok(nodes)
}
