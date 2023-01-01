use vec_like::VecMapGetEntry;

use crate::*;

pub(crate) fn crate_prelude<'a>(
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<CratePrelude<'a>> {
    let toolchain = crate_path.toolchain(db);
    let path_menu = db.path_menu(toolchain)?;
    let core_prelude_module = path_menu.core_prelude();
    Ok(CratePrelude::new(
        entity_tree_sheet(db, core_prelude_module)?.module_symbols(),
        crate_specific_prelude(db, crate_path).as_ref()?,
    ))
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn crate_specific_prelude(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<VecMap<EntitySymbol>> {
    let package_path = crate_path.package_path(db);
    let package_dependencies = db.package_dependencies(package_path)?;
    let mut nodes: VecMap<EntitySymbol> = VecMap::default();
    let crate_word = db.word_menu().crate_word();
    nodes.insert(EntitySymbol::CrateRoot {
        ident: crate_word,
        module_path: ModulePath::new_root(db, crate_path),
    });
    nodes
        .extend(
            package_dependencies
                .iter()
                .map(|package_dependency| todo!()),
        )
        .unwrap();
    Ok(nodes)
}

#[derive(Debug, Clone, Copy)]
pub struct CratePrelude<'a> {
    universal_prelude: &'a [EntitySymbol],
    crate_specific_prelude: &'a [EntitySymbol],
}

impl<'a> CratePrelude<'a> {
    pub(crate) fn new(
        universal_prelude: &'a [EntitySymbol],
        crate_specific_prelude: &'a [EntitySymbol],
    ) -> Self {
        Self {
            universal_prelude,
            crate_specific_prelude,
        }
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Option<&'a EntitySymbol> {
        self.universal_prelude
            .get_entry(ident)
            .or_else(|| self.crate_specific_prelude.get_entry(ident))
    }
}
