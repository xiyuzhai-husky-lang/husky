mod assoc_item;
mod attr;
mod impl_block;
mod major_item;
mod ty_variant;

pub use self::assoc_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::ty_variant::*;

use crate::*;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DecTemplate {
    Submodule,
    MajorItem(MajorItemDecTemplate),
    ImplBlock(ImplBlockDecTemplate),
    AssocItem(AssocItemDecTemplate),
    Variant(TypeVariantDecTemplate),
    Attr(AttrDecTemplate),
}

pub trait HasDecTemplate: Copy {
    type DecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate>;
}

impl HasDecTemplate for ItemPath {
    type DecTemplate = DecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        Ok(match self {
            ItemPath::Submodule(_, _) => DecTemplate::Submodule,
            ItemPath::MajorItem(path) => path.dec_template(db)?.into(),
            ItemPath::AssocItem(path) => path.dec_template(db)?.into(),
            ItemPath::TypeVariant(_, path) => path.dec_template(db)?.into(),
            ItemPath::ImplBlock(path) => path.dec_template(db)?.into(),
            ItemPath::Attr(_, _) => todo!(),
            ItemPath::Script(_, _) => todo!(),
        })
    }
}
