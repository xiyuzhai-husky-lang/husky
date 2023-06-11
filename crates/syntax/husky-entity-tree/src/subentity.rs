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

pub(crate) fn subentity_path(
    db: &dyn EntityTreeDb,
    parent: EntityPath,
    ident: Ident,
) -> EntityTreeResult<EntityPath> {
    match parent {
        EntityPath::Module(module_path) => {
            match db
                .entity_tree_sheet(module_path)?
                .module_symbols()
                .resolve_ident(ident)
            {
                Some(entity_symbol) => Ok(entity_symbol.path(db)),
                None => Err(OriginalEntityTreeError::NoSubentity)?,
            }
        }
        EntityPath::ModuleItem(module_item_path) => {
            let crate_path = module_item_path.crate_path(db);
            let _entity_tree_crate_bundle = db.entity_tree_bundle(crate_path)?;
            match module_item_path {
                ModuleItemPath::Type(path) => {
                    if let Some((_, variant)) = path.variants(db)?.get_entry(ident) {
                        Ok(variant.path(db).into())
                    } else if let Some((_, associated_item)) = path.items(db)?.get_entry(ident) {
                        Ok(match associated_item.associated_item_kind(db) {
                            AssociatedItemKind::TypeItem(ty_item_kind) => {
                                TypeItemPath::new(db, path, ident, ty_item_kind).into()
                            }
                            AssociatedItemKind::TraitItem(_) => todo!(),
                            AssociatedItemKind::TraitForTypeItem(_) => todo!(),
                        })
                    } else {
                        // todo: check trait impls
                        Err(OriginalEntityTreeError::NoSubentity)?
                    }
                }
                ModuleItemPath::Trait(_) => todo!(),
                ModuleItemPath::Form(_) => todo!(),
            }
        }
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::TypeVariant(_) => todo!(),
        EntityPath::ImplBlock(_) => todo!(),
    }
}
