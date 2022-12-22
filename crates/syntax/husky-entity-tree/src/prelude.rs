use crate::*;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn crate_specific_prelude(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<VecMap<EntitySymbol>> {
    let package_path = crate_path.package_path(db);
    let package_dependencies = db.package_dependencies(package_path)?;
    let mut nodes: VecMap<EntitySymbol> = VecMap::default();
    let crate_word = db.word_menu().crate_word();
    let crate_root = ModulePath::new_root(db, crate_path);
    nodes.insert(EntitySymbol::Module {
        ident: crate_word,
        accessibility: Accessibility::PublicUnder(crate_root),
        module_path: ModulePath::new_root(db, crate_path),
    });
    nodes.extend(
        package_dependencies
            .iter()
            .map(|package_dependency| todo!()),
    );
    Ok(nodes)
}
