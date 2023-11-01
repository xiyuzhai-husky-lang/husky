pub(crate) mod major_item;
mod submodule;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
#[enum_class::from_variants]
pub enum RustTranspilation {
    SynItem(syn::Item),
}

pub trait HasItemRustTranspilation: Copy {
    type RustTranspilation;

    fn defn_rust_transpilation(
        self,
        db: &dyn RustTranspilationDb,
    ) -> Option<Self::RustTranspilation>;
}

impl HasItemRustTranspilation for ItemPath {
    type RustTranspilation = RustTranspilation;

    fn defn_rust_transpilation(
        self,
        db: &dyn RustTranspilationDb,
    ) -> Option<Self::RustTranspilation> {
        match self {
            ItemPath::Submodule(submodule_path) => submodule_path
                .defn_rust_transpilation(db)
                .map(syn::Item::Mod)
                .map(Into::into),
            ItemPath::MajorItem(major_item_path) => {
                major_item_path.defn_rust_transpilation(db).map(Into::into)
            }
            ItemPath::AssociatedItem(_) => todo!(),
            ItemPath::TypeVariant(_) => todo!(),
            ItemPath::ImplBlock(_) => todo!(),
            ItemPath::Attr(_) => todo!(),
        }
    }
}

impl HasItemRustTranspilation for ModulePath {
    type RustTranspilation = syn::File;

    fn defn_rust_transpilation(
        self,
        db: &dyn RustTranspilationDb,
    ) -> Option<Self::RustTranspilation> {
        todo!()
    }
}
