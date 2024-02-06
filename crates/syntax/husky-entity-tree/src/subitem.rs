use crate::*;

use vec_like::VecMapGetEntry;

#[enum_class::from_variants]
pub enum SubitemPath {
    // submodules, module items, type variants
    Principal(PrincipalEntityPath),
    // associated items
    Assoc,
}

pub(crate) fn subitem_path(
    db: &::salsa::Db,
    parent: MajorEntityPath,
    ident: Ident,
) -> EntityTreeResult<SubitemPath> {
    match parent {
        MajorEntityPath::Module(module_path) => {
            match db
                .item_syn_tree_sheet(module_path)
                .module_symbols()
                .resolve_ident(db, ReferenceModulePath::Generic, ident)
            {
                Some(item_symbol) => Ok(item_symbol.principal_entity_path(db).into()),
                None => Err(OriginalEntityTreeError::NoVisibleSubitem)?,
            }
        }
        MajorEntityPath::MajorItem(major_item_path) => {
            let crate_path = major_item_path.crate_path(db);
            let _item_tree_crate_bundle = db.item_syn_tree_bundle(crate_path);
            match major_item_path {
                MajorItemPath::Type(path) => {
                    if let Some((_, path)) = path.ty_variant_paths(db).get_entry(ident).copied() {
                        Ok(SubitemPath::Principal(path.into()))
                    } else {
                        Ok(SubitemPath::Assoc)
                    }
                }
                MajorItemPath::Trait(_) => Ok(SubitemPath::Assoc),
                MajorItemPath::Fugitive(_) => {
                    Err(OriginalEntityTreeError::NoSubitemForFugitive.into())
                }
            }
        }
    }
}
