pub mod assoc_item;
pub mod attr;
pub mod impl_block;
pub mod major_item;
pub mod script;
pub mod submodule;
pub mod ty_variant;

use self::assoc_item::*;
use self::attr::*;
use self::impl_block::{ImplBlockSynNode, ImplBlockSynNodePath, ImplBlockSynNodePathData};
use self::major_item::*;
use self::script::{ScriptSynNodePath, ScriptSynNodePathData};
use self::submodule::*;
use self::ty_variant::*;
use crate::*;
use enum_class::Room32;
use husky_ast::DefnBlock;
use husky_entity_path::path::ItemPath;
use husky_token::IdentToken;
use husky_vfs::toolchain::Toolchain;
use smallvec::{smallvec, SmallVec};
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum ItemSynNodePath {
    Submodule(Room32, SubmoduleSynNodePath),
    MajorItem(MajorItemSynNodePath),
    TypeVariant(Room32, TypeVariantSynNodePath),
    ImplBlock(ImplBlockSynNodePath),
    AssocItem(AssocItemSynNodePath),
    Attr(Room32, AttrSynNodePath),
    Script(Room32, ScriptSynNodePath),
}

impl std::ops::Deref for ItemSynNodePath {
    type Target = ItemSynNodePathId;

    fn deref(&self) -> &Self::Target {
        let slf: &(u32, u32, ItemSynNodePathId) = unsafe { std::mem::transmute(self) };
        &slf.2
    }
}

#[salsa::interned]
pub struct ItemSynNodePathId {
    pub data: ItemSynNodePathData,
}

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ItemSynNodePathData {
    Submodule(SubmoduleSynNodePathData),
    MajorItem(MajorItemSynNodePathData),
    TypeVariant(TypeVariantSynNodePathData),
    ImplBlock(ImplBlockSynNodePathData),
    AssocItem(AssocItemSynNodePathData),
    Attr(AttrSynNodePathData),
    Script(ScriptSynNodePathData),
}

impl ItemSynNodePathId {
    pub fn syn_node_path(self, db: &::salsa::Db) -> ItemSynNodePath {
        match self.data(db) {
            ItemSynNodePathData::Submodule(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::MajorItem(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::TypeVariant(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::ImplBlock(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::AssocItem(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::Attr(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::Script(data) => data.syn_node_path(self).into(),
        }
    }

    pub fn unambiguous_item_path(self, db: &::salsa::Db) -> Option<ItemPath> {
        self.data(db).unambiguous_item_path()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.data(db).module_path(db)
    }

    pub(crate) fn opt_ast_idx(self, db: &::salsa::Db) -> Option<AstIdx> {
        self.data(db).opt_ast_idx(self, db)
    }
}

#[test]
fn syn_node_path_id_conversion_works() {
    use crate::helpers::paths::module_item_syn_node_paths;

    DB::ast_plain_test(
        |db, module_path| {
            for &syn_node_path in module_item_syn_node_paths(db, module_path) {
                assert_eq!(syn_node_path.syn_node_path(db), syn_node_path);
            }
        },
        &AstTestConfig::new(
            "syn_node_path_id_conversion",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SYNTAX,
        ),
    )
}

impl ItemSynNodePathData {
    pub fn unambiguous_item_path(self) -> Option<ItemPath> {
        match self {
            ItemSynNodePathData::Submodule(slf) => slf.unambiguous_item_path().map(Into::into),
            ItemSynNodePathData::MajorItem(slf) => slf.unambiguous_item_path().map(Into::into),
            ItemSynNodePathData::TypeVariant(slf) => slf.unambiguous_item_path().map(Into::into),
            ItemSynNodePathData::ImplBlock(slf) => slf.unambiguous_item_path().map(Into::into),
            ItemSynNodePathData::AssocItem(slf) => slf.unambiguous_item_path().map(Into::into),
            ItemSynNodePathData::Attr(slf) => slf.unambiguous_item_path().map(Into::into),
            ItemSynNodePathData::Script(slf) => Some(slf.item_path().into()),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            ItemSynNodePathData::Submodule(slf) => slf.module_path(db),
            ItemSynNodePathData::MajorItem(slf) => slf.module_path(db),
            ItemSynNodePathData::TypeVariant(slf) => slf.module_path(db),
            ItemSynNodePathData::ImplBlock(slf) => slf.module_path(db),
            ItemSynNodePathData::AssocItem(slf) => slf.module_path(db),
            ItemSynNodePathData::Attr(slf) => slf.module_path(db),
            ItemSynNodePathData::Script(slf) => *slf.module_path(db),
        }
    }
    pub fn opt_ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> Option<AstIdx> {
        match self {
            ItemSynNodePathData::Submodule(slf) => Some(slf.ast_idx(id, db)),
            ItemSynNodePathData::MajorItem(slf) => Some(slf.ast_idx(id, db)),
            ItemSynNodePathData::TypeVariant(slf) => Some(slf.ast_idx(id, db)),
            ItemSynNodePathData::ImplBlock(slf) => Some(slf.ast_idx(id, db)),
            ItemSynNodePathData::AssocItem(slf) => Some(slf.ast_idx(id, db)),
            ItemSynNodePathData::Attr(slf) => Some(slf.ast_idx(id, db)),
            ItemSynNodePathData::Script(_) => None,
        }
    }
}

impl ItemSynNodePath {
    pub fn unambiguous_item_path(self, db: &::salsa::Db) -> Option<ItemPath> {
        match self {
            ItemSynNodePath::Submodule(_, syn_node_path) => {
                syn_node_path.unambiguous_item_path(db).map(Into::into)
            }
            ItemSynNodePath::MajorItem(syn_node_path) => {
                syn_node_path.unambiguous_item_path(db).map(Into::into)
            }
            ItemSynNodePath::TypeVariant(_, syn_node_path) => {
                syn_node_path.unambiguous_item_path(db).map(Into::into)
            }
            ItemSynNodePath::ImplBlock(syn_node_path) => {
                syn_node_path.unambiguous_item_path(db).map(Into::into)
            }
            ItemSynNodePath::AssocItem(syn_node_path) => {
                syn_node_path.unambiguous_item_path(db).map(Into::into)
            }
            ItemSynNodePath::Attr(_, syn_node_path) => {
                syn_node_path.unambiguous_item_path(db).map(Into::into)
            }
            ItemSynNodePath::Script(_, syn_node_path) => Some(syn_node_path.item_path().into()),
        }
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.module_path(db).toolchain(db)
    }
}

pub trait HasSynNodePath: Copy {
    type SynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath;
}

impl HasSynNodePath for ItemPath {
    type SynNodePath = ItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        match self {
            ItemPath::Submodule(_, path) => path.syn_node_path(db).into(),
            ItemPath::MajorItem(path) => path.syn_node_path(db).into(),
            ItemPath::AssocItem(path) => path.syn_node_path(db).into(),
            ItemPath::TypeVariant(_, path) => path.syn_node_path(db).into(),
            ItemPath::ImplBlock(path) => path.syn_node_path(db).into(),
            ItemPath::Attr(_, path) => path.syn_node_path(db).into(),
            ItemPath::Script(_, script) => todo!(),
        }
    }
}

#[derive(Default)]
pub(crate) struct ItemSynNodePathRegistry {
    next_disambiguators: VecPairMap<ItemPath, u8>,
}

impl ItemSynNodePathRegistry {
    fn issue_maybe_ambiguous_path<P: Copy + Into<ItemPath>>(
        &mut self,
        path: P,
    ) -> DisambiguatedItemPath<P> {
        let next_disambiguator = self
            .next_disambiguators
            .get_value_mut_or_insert_default(path.into());
        let disambiguator = *next_disambiguator;
        *next_disambiguator += 1;
        DisambiguatedItemPath {
            maybe_ambiguous_item_path: path,
            disambiguator,
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisambiguatedItemPath<P> {
    maybe_ambiguous_item_path: P,
    disambiguator: u8,
}

impl<P> DisambiguatedItemPath<P> {
    fn from_path(maybe_ambiguous_item_path: P) -> Self {
        Self {
            maybe_ambiguous_item_path,
            disambiguator: 0,
        }
    }

    fn unambiguous_item_path(self) -> Option<P> {
        (self.disambiguator == 0).then_some(self.maybe_ambiguous_item_path)
    }

    fn maybe_ambiguous_item_path(self) -> P {
        self.maybe_ambiguous_item_path
    }
}

/// this is pub(crate) because it contains AstIdx which affects incremental computation
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub(crate) enum ItemSynNode {
    Submodule(SubmoduleSynNode),
    MajorItem(MajorItemSynNode),
    TypeVariant(TypeVariantSynNode),
    ImplBlock(ImplBlockSynNode),
    Attr(AttrSynNode),
}

impl ItemSynNode {
    pub(crate) fn try_new_major(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        item_path: ItemPath,
        block: DefnBlock,
    ) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(_, submodule_path) => Some(
                SubmoduleSynNode::new(
                    db,
                    registry,
                    submodule_path,
                    visibility,
                    ast_idx,
                    ident_token,
                )
                .into(),
            ),
            ItemPath::MajorItem(module_item_path) => Some(
                MajorItemSynNode::new(
                    db,
                    registry,
                    module_item_path,
                    visibility,
                    ast_idx,
                    ident_token,
                    block,
                )
                .into(),
            ),
            ItemPath::AssocItem(_) | ItemPath::TypeVariant(_, _) => None,
            ItemPath::ImplBlock(_) | ItemPath::Attr(_, _) => unreachable!(),
            ItemPath::Script(_, _) => todo!(),
        }
    }

    pub fn syn_node_path(&self, _db: &::salsa::Db) -> ItemSynNodePath {
        match self {
            ItemSynNode::Submodule(node) => node.syn_node_path.into(),
            ItemSynNode::MajorItem(node) => node.syn_node_path.into(),
            ItemSynNode::TypeVariant(node) => node.syn_node_path.into(),
            ItemSynNode::ImplBlock(node) => node.syn_node_path().into(),
            ItemSynNode::Attr(node) => node.syn_node_path().into(),
        }
    }
}

pub trait HasAssocItemPaths: Copy {
    type AssocItemPath;

    fn assoc_item_paths(self, db: &::salsa::Db) -> &[(Ident, Self::AssocItemPath)];
}
