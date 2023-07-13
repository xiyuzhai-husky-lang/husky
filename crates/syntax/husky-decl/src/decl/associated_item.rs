mod ill_formed_item;
mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::ill_formed_item::*;
pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;
use husky_coword::Ident;
use husky_entity_taxonomy::{AssociatedItemKind, EntityKind, TraitItemKind, TypeItemKind};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum AssociatedItemNodeDecl {
    TypeItem(TypeItemNodeDecl),
    TraitItem(TraitItemNodeDecl),
    TraitForTypeItem(TraitForTypeItemNodeDecl),
    IllFormedItem(IllFormedItemNodeDecl),
}

impl AssociatedItemNodeDecl {
    pub fn node_path(self, db: &dyn DeclDb) -> AssociatedItemNodePath {
        match self {
            AssociatedItemNodeDecl::TypeItem(node_decl) => node_decl.node_path(db).into(),
            AssociatedItemNodeDecl::TraitItem(node_decl) => node_decl.node_path(db).into(),
            AssociatedItemNodeDecl::TraitForTypeItem(node_decl) => node_decl.node_path(db).into(),
            AssociatedItemNodeDecl::IllFormedItem(node_decl) => node_decl.node_path(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            AssociatedItemNodeDecl::TypeItem(node_decl) => node_decl.ast_idx(db),
            AssociatedItemNodeDecl::TraitItem(node_decl) => node_decl.ast_idx(db),
            AssociatedItemNodeDecl::TraitForTypeItem(node_decl) => node_decl.ast_idx(db),
            AssociatedItemNodeDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            AssociatedItemNodeDecl::TypeItem(node_decl) => node_decl.implicit_parameters(db),
            AssociatedItemNodeDecl::TraitItem(node_decl) => node_decl.implicit_parameters(db),
            AssociatedItemNodeDecl::TraitForTypeItem(_) => todo!(),
            AssociatedItemNodeDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            AssociatedItemNodeDecl::TypeItem(node_decl) => node_decl.expr_region(db),
            AssociatedItemNodeDecl::TraitItem(node_decl) => node_decl.expr_region(db),
            AssociatedItemNodeDecl::TraitForTypeItem(node_decl) => node_decl.expr_region(db),
            AssociatedItemNodeDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        match self {
            AssociatedItemNodeDecl::TypeItem(node_decl) => node_decl.errors(db),
            AssociatedItemNodeDecl::TraitItem(node_decl) => node_decl.errors(db),
            AssociatedItemNodeDecl::TraitForTypeItem(node_decl) => node_decl.errors(db),
            AssociatedItemNodeDecl::IllFormedItem(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum AssociatedItemDecl {
    TypeItem(TypeItemDecl),
    TraitItem(TraitItemDecl),
    TraitForTypeItem(TraitForTypeItemDecl),
}

impl AssociatedItemDecl {
    pub fn path(self, db: &dyn DeclDb) -> AssociatedItemPath {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.path(db).into(),
            AssociatedItemDecl::TraitItem(decl) => decl.path(db).into(),
            AssociatedItemDecl::TraitForTypeItem(decl) => decl.path(db).into(),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDecl::TraitItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDecl::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.expr_region(db),
            AssociatedItemDecl::TraitItem(decl) => decl.expr_region(db),
            AssociatedItemDecl::TraitForTypeItem(decl) => decl.expr_region(db),
        }
    }
}

impl HasDecl for AssociatedItemPath {
    type Decl = AssociatedItemDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        todo!()
        // associated_item_decl(db, self).as_ref().copied()
    }
}

// #[salsa::tracked(jar = DeclJar, return_ref)]
// pub(crate) fn associated_item_decl(
//     db: &dyn DeclDb,
//     node: AssociatedItemNode,
// ) -> DeclResult<AssociatedItemDecl> {
//     let parser = DeclParseContext::new(db, node.module_path(db))?;
//     parser.parse_associated_item_decl(node)
// }

impl<'a> DeclParser<'a> {
    // fn parse_associated_item_decl(
    //     &self,
    //     node: AssociatedItemNode,
    // ) -> DeclResult<AssociatedItemDecl> {
    //     let ast_idx = node.ast_idx(self.db());
    //     Ok(match self.ast_sheet()[ast_idx] {
    //         Ast::Defn {
    //             token_group_idx,
    //             entity_kind:
    //                 EntityKind::AssociatedItem {
    //                     associated_item_kind,
    //                 },
    //             saved_stream_state,
    //             ..
    //         } => match associated_item_kind {
    //             AssociatedItemKind::TraitItem(_) => todo!(),
    //             AssociatedItemKind::TypeItem(ty_item_kind) => self
    //                 .parse_ty_item_decl(
    //                     ty_item_kind,
    //                     ast_idx,
    //                     token_group_idx,
    //                     node,
    //                     saved_stream_state,
    //                 )?
    //                 .into(),
    //             AssociatedItemKind::TraitForTypeItem(trai_item_kind) => self
    //                 .parse_trai_for_ty_item_decl(
    //                     trai_item_kind,
    //                     ast_idx,
    //                     token_group_idx,
    //                     node,
    //                     saved_stream_state,
    //                 )?
    //                 .into(),
    //         },
    //         _ => unreachable!(),
    //     })
    // }
}

pub trait HasItemDeclsMap {
    type ItemDecls;

    fn item_decls_map<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Result<Self::ItemDecls, ()>)]>;
}

pub trait HasItemDecls {
    type ItemDecls;

    fn item_decls<'a>(self, db: &'a dyn DeclDb) -> DeclResult<&'a Self::ItemDecls>;
}
