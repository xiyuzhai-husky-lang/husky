mod builder;

use self::builder::ItemHirDefnDependenciesBuilder;
use crate::*;
use vec_like::VecSet;

#[derive(Default)]
pub(crate) struct ItemHirDefnDependencies {
    item_paths_in_current_crate: VecSet<ItemPath>,
    item_paths_in_other_local_crates: VecSet<ItemPath>,
}

impl ItemHirDefn {
    pub(crate) fn dependencies(self, db: &dyn HirDefnDb) -> Option<&ItemHirDefnDependencies> {
        match self {
            ItemHirDefn::Submodule(_) => None,
            ItemHirDefn::MajorItem(hir_defn) => match hir_defn {
                MajorItemHirDefn::Type(_) => todo!(),
                MajorItemHirDefn::Trait(_) => todo!(),
                MajorItemHirDefn::Fugitive(_) => todo!(),
            },
            // ask its parent
            ItemHirDefn::TypeVariant(_) => None,
            ItemHirDefn::ImplBlock(hir_defn) => match hir_defn {
                ImplBlockHirDefn::Type(_) => None,
                ImplBlockHirDefn::TraitForType(hir_defn) => todo!(),
            },
            ItemHirDefn::AssociatedItem(_) => todo!(),
            ItemHirDefn::Attr(_) => None,
        }
    }
}
