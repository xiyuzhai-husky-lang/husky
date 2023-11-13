use crate::*;


use vec_like::VecMapGetEntry;

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn module_subitem_path(
    _db: &dyn EntitySynTreeDb,
    _parent: ModulePath,
    _identifier: Ident,
) -> EntitySynTreeResult<ItemPath> {
    todo!()
}

#[enum_class::from_variants]
pub enum SubitemPath {
    // submodules, module items, type variants
    Principal(PrincipalEntityPath),
    // associated items
    Associated,
}

pub(crate) fn subitem_path(
    db: &dyn EntitySynTreeDb,
    parent: MajorEntityPath,
    ident: Ident,
) -> EntitySynTreeResult<SubitemPath> {
    match parent {
        MajorEntityPath::Module(module_path) => {
            match db
                .item_syn_tree_sheet(module_path)?
                .module_symbols()
                .resolve_ident(db, ReferenceModulePath::Generic, ident)
            {
                Some(item_symbol) => Ok(item_symbol.path(db).into()),
                None => Err(OriginalEntityTreeError::NoVisibleSubitem)?,
            }
        }
        MajorEntityPath::MajorItem(module_item_path) => {
            let crate_path = module_item_path.crate_path(db);
            let _item_tree_crate_bundle = db.item_syn_tree_bundle(crate_path)?;
            match module_item_path {
                MajorItemPath::Type(path) => {
                    if let Some((_, path)) = path.ty_variant_paths(db).get_entry(ident).copied() {
                        Ok(SubitemPath::Principal(path.into()))
                    } else if let Some((_, _node)) = path.item_syn_node_paths(db)?.get_entry(ident) {
                        Ok(SubitemPath::Associated)
                    } else {
                        // todo: check trait impls
                        Err(OriginalEntityTreeError::NoVisibleSubitem)?
                    }
                }
                MajorItemPath::Trait(_) => todo!(),
                MajorItemPath::Fugitive(_) => todo!(),
            }
        }
    }
}
