use husky_entity_taxonomy::AssociatedItemKind;

use crate::*;

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn module_subentity_path(
    db: &dyn EntityTreeDb,
    parent: ModulePath,
    identifier: Identifier,
) -> EntityTreeResult<EntityPath> {
    todo!()
}

pub(crate) fn subentity_path(
    db: &dyn EntityTreeDb,
    parent: EntityPath,
    ident: Identifier,
) -> EntityTreeResult<EntityPath> {
    match parent {
        EntityPath::Module(module_path) => {
            match db
                .entity_tree_sheet(module_path)?
                .module_specific_symbols()
                .get_entry(ident)
            {
                Some(_) => todo!(),
                None => todo!(),
            }
        }
        EntityPath::ModuleItem(module_item_path) => {
            let crate_path = module_item_path.crate_path(db);
            let entity_tree_crate_bundle = db.entity_tree_crate_bundle(crate_path)?;
            match module_item_path {
                ModuleItemPath::Type(ty) => {
                    let ty_associated_items = match ty_associated_items(db, ty) {
                        Ok(ty_associated_items) => ty_associated_items,
                        Err(_) => todo!(),
                    };
                    if let Some((_, associated_item)) = ty_associated_items.get_entry(ident) {
                        Ok(match associated_item.associated_item_kind(db) {
                            AssociatedItemKind::TypeItem(ty_item_kind) => {
                                TypeItemPath::new(db, ty, ident, ty_item_kind).into()
                            }
                            AssociatedItemKind::TraitItem(_) => todo!(),
                            AssociatedItemKind::TypeAsTraitItem(_) => todo!(),
                        })
                    } else {
                        // todo: check trait impls
                        Err(EntityTreeError::NoSubentity)
                    }
                }
                ModuleItemPath::Trait(_) => todo!(),
                ModuleItemPath::Form(_) => todo!(),
            }
        }
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::Variant(_) => todo!(),
    }
}
