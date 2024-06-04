pub mod assoc_item;
pub mod attr;
pub mod crate_;
pub mod impl_block;
pub mod major_item;
pub mod package;
pub mod ty_variant;

use self::assoc_item::*;
use self::attr::*;
use self::impl_block::*;
use self::major_item::*;
use self::ty_variant::*;
use crate::*;
use husky_entity_path::path::ItemPath;
use husky_syn_decl::decl::HasSynDecl;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ItemDecTemplate {
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
    type DecTemplate = ItemDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        Ok(match self {
            ItemPath::Submodule(_, _) => ItemDecTemplate::Submodule,
            ItemPath::MajorItem(path) => path.dec_template(db)?.into(),
            ItemPath::AssocItem(path) => path.dec_template(db)?.into(),
            ItemPath::TypeVariant(_, path) => path.dec_template(db)?.into(),
            ItemPath::ImplBlock(path) => path.dec_template(db)?.into(),
            ItemPath::Attr(_, _) => todo!(),
            ItemPath::Script(_, _) => todo!(),
        })
    }
}

/// only crate_path, package_path implement this
pub trait HasDecSignature: Copy {
    type DecSignature;

    fn dec_signature(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecSignature>;
}
