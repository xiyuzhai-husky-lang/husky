use crate::*;
use husky_entity_taxonomy::AssociatedItemKind;
use husky_print_utils::p;
use vec_like::VecMapGetEntry;

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn module_subentity_path(
    _db: &dyn EntityTreeDb,
    _parent: ModulePath,
    _identifier: Ident,
) -> EntityTreeResult<EntityPath> {
    todo!()
}

#[enum_class::from_variants]
pub enum SubentityPath {
    // submodules, module items, type variants
    NonAssociated(EntityPath),
    // associated items
    Associated,
}

pub(crate) fn subentity_path(
    db: &dyn EntityTreeDb,
    parent: EntityPath,
    ident: Ident,
) -> EntityTreeResult<SubentityPath> {
    match parent {
        EntityPath::Module(module_path) => {
            match db
                .entity_tree_sheet(module_path)?
                .module_symbols()
                .resolve_ident(db, ReferenceModulePath::Generic, ident)
            {
                Some(entity_symbol) => Ok(entity_symbol.path(db).into()),
                None => Err(OriginalEntityTreeError::NoVisibleSubentity)?,
            }
        }
        EntityPath::ModuleItem(module_item_path) => {
            let crate_path = module_item_path.crate_path(db);
            let _entity_tree_crate_bundle = db.entity_tree_bundle(crate_path)?;
            match module_item_path {
                ModuleItemPath::Type(path) => {
                    if let Some((_, variant)) = path.variants(db)?.get_entry(ident) {
                        Ok(SubentityPath::NonAssociated(variant.path(db).into()))
                    } else if let Some((_, node)) = path.item_node_paths(db)?.get_entry(ident) {
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
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::TypeVariant(_) => todo!(),
        EntityPath::ImplBlock(_) => todo!(),
    }
}
