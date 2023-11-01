use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub enum RustTranspilation {
    SynItem(syn::Item),
}

pub trait HasRustTranspilation: Copy {
    type RustTranspilation;

    fn rust_transpilation(self, db: &dyn RustTranspilationDb) -> Option<Self::RustTranspilation>;
}

impl HasRustTranspilation for ItemPath {
    type RustTranspilation = RustTranspilation;

    fn rust_transpilation(self, db: &dyn RustTranspilationDb) -> Option<Self::RustTranspilation> {
        match self {
            ItemPath::Submodule(_) => todo!(),
            ItemPath::MajorItem(_) => todo!(),
            ItemPath::AssociatedItem(_) => todo!(),
            ItemPath::TypeVariant(_) => todo!(),
            ItemPath::ImplBlock(_) => todo!(),
            ItemPath::Attr(_) => todo!(),
        }
    }
}
