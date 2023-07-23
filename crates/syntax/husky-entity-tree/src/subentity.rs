use crate::*;
use husky_entity_taxonomy::AssociatedItemKind;
use husky_print_utils::p;
use vec_like::VecMapGetEntry;

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn module_subentity_path(
    _db: &dyn EntitySynTreeDb,
    _parent: ModulePath,
    _identifier: Ident,
) -> EntitySynTreeResult<EntityPath> {
    todo!()
}

#[enum_class::from_variants]
pub enum SubentityPath {
    // submodules, module items, type variants
    Principal(PrincipalEntityPath),
    // associated items
    Associated,
}

pub(crate) fn subentity_path(
    db: &dyn EntitySynTreeDb,
    parent: MajorEntityPath,
    ident: Ident,
) -> EntitySynTreeResult<SubentityPath> {
    match parent {
        MajorEntityPath::Module(module_path) => {
            match db
                .entity_syn_tree_sheet(module_path)?
                .module_symbols()
                .resolve_ident(db, ReferenceModulePath::Generic, ident)
            {
                Some(entity_symbol) => Ok(entity_symbol.path(db).into()),
                None => Err(OriginalEntityTreeError::NoVisibleSubentity)?,
            }
        }
        MajorEntityPath::ModuleItem(module_item_path) => {
            let crate_path = module_item_path.crate_path(db);
            let _entity_tree_crate_bundle = db.entity_syn_tree_bundle(crate_path)?;
            match module_item_path {
                ModuleItemPath::Type(path) => {
                    if let Some((_, path)) = path.ty_variant_paths(db).get_entry(ident).copied() {
                        Ok(SubentityPath::Principal(path.into()))
                    } else if let Some((_, node)) = path.item_syn_node_paths(db)?.get_entry(ident) {
                        Ok(SubentityPath::Associated)
                    } else {
                        // todo: check trait impls
                        Err(OriginalEntityTreeError::NoVisibleSubentity)?
                    }
                }
                ModuleItemPath::Trait(_) => todo!(),
                ModuleItemPath::Fugitive(_) => todo!(),
            }
        }
    }
}
