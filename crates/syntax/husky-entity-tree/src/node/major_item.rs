pub mod form;
pub mod trai;
pub mod ty;

use self::form::*;
use self::trai::*;
use self::ty::*;
use super::*;
use husky_ast::DefnBlock;
use husky_entity_path::path::major_item::MajorItemPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorItemSynNodePath {
    Trait(TraitSynNodePath),
    Type(TypeSynNodePath),
    Form(FormSynNodePath),
}

impl std::ops::Deref for MajorItemSynNodePath {
    type Target = ItemSynNodePathId;

    fn deref(&self) -> &Self::Target {
        let slf: &(u32, ItemSynNodePathId) = unsafe { std::mem::transmute(self) };
        &slf.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorItemSynNodePathData {
    Trait(TraitSynNodePathData),
    Type(TypeSynNodePathData),
    Form(FormSynNodePathData),
}

impl MajorItemSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: MajorItemPath,
    ) -> Self {
        match path {
            MajorItemPath::Type(path) => TypeSynNodePath::new(db, registry, path).into(),
            MajorItemPath::Trait(path) => TraitSynNodePath::new(db, registry, path).into(),
            MajorItemPath::Form(path) => FormSynNodePath::new(db, registry, path).into(),
        }
    }

    pub fn unambiguous_item_path(self, db: &::salsa::Db) -> Option<MajorItemPath> {
        match (*self).unambiguous_item_path(db) {
            Some(ItemPath::MajorItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        match self {
            MajorItemSynNodePath::Trait(slf) => slf.ident(db),
            MajorItemSynNodePath::Type(slf) => slf.ident(db),
            MajorItemSynNodePath::Form(slf) => slf.ident(db),
        }
    }

    pub(crate) fn attrs(self, db: &::salsa::Db) -> &[(AttrSynNodePath, AttrSynNode)] {
        // ad hoc
        match self {
            MajorItemSynNodePath::Trait(_) => &[],
            MajorItemSynNodePath::Type(syn_node_path) => syn_node_path.attr_syn_nodes(db),
            MajorItemSynNodePath::Form(_) => &[],
        }
    }
}

impl MajorItemSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> MajorItemSynNodePath {
        match self {
            MajorItemSynNodePathData::Trait(slf) => slf.syn_node_path(id).into(),
            MajorItemSynNodePathData::Type(slf) => slf.syn_node_path(id).into(),
            MajorItemSynNodePathData::Form(slf) => slf.syn_node_path(id).into(),
        }
    }

    pub fn unambiguous_item_path(self) -> Option<MajorItemPath> {
        match self {
            MajorItemSynNodePathData::Trait(slf) => slf.path().map(Into::into),
            MajorItemSynNodePathData::Type(slf) => slf.path().map(Into::into),
            MajorItemSynNodePathData::Form(slf) => slf.path().map(Into::into),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            MajorItemSynNodePathData::Trait(slf) => slf.module_path(db),
            MajorItemSynNodePathData::Type(slf) => slf.module_path(db),
            MajorItemSynNodePathData::Form(slf) => slf.module_path(db),
        }
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        match self {
            MajorItemSynNodePathData::Trait(slf) => slf.ast_idx(id, db),
            MajorItemSynNodePathData::Type(slf) => slf.ast_idx(id, db),
            MajorItemSynNodePathData::Form(slf) => slf.ast_idx(id, db),
        }
    }
}

impl HasSynNodePath for MajorItemPath {
    type SynNodePath = MajorItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        match self {
            MajorItemPath::Type(path) => path.syn_node_path(db).into(),
            MajorItemPath::Trait(path) => path.syn_node_path(db).into(),
            MajorItemPath::Form(path) => path.syn_node_path(db).into(),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct MajorItemSynNode {
    pub(crate) syn_node_path: MajorItemSynNodePath,
    pub(crate) visibility: Scope,
    pub(crate) ast_idx: AstIdx,
    pub(crate) ident_token: IdentToken,
    pub(crate) block: DefnBlock,
}

impl MajorItemSynNode {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        module_item_path: MajorItemPath,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        block: DefnBlock,
    ) -> Self {
        MajorItemSynNode {
            syn_node_path: MajorItemSynNodePath::new(db, registry, module_item_path),
            visibility,
            ast_idx,
            ident_token,
            block,
        }
    }

    /// only gives a path when valid
    pub fn unambiguous_item_path(&self, db: &::salsa::Db) -> Option<MajorItemPath> {
        self.syn_node_path.unambiguous_item_path(db)
    }
}
