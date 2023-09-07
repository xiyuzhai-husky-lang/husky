mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TraitItemSynNodeDecl {
    AssociatedFn(TraitAssociatedFnSynNodeDecl),
    MethodFn(TraitMethodFnSynNodeDecl),
    AssociatedType(TraitAssociatedTypeSynNodeDecl),
    AssociatedVal(TraitAssociatedValSynNodeDecl),
}

impl TraitItemSynNodeDecl {
    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> TraitItemSynNodePath {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(decl) => decl.syn_node_path(db),
            TraitItemSynNodeDecl::MethodFn(decl) => decl.syn_node_path(db),
            TraitItemSynNodeDecl::AssociatedType(decl) => decl.syn_node_path(db),
            TraitItemSynNodeDecl::AssociatedVal(decl) => decl.syn_node_path(db),
        }
    }

    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(decl) => decl.ast_idx(db),
            TraitItemSynNodeDecl::MethodFn(decl) => decl.ast_idx(db),
            TraitItemSynNodeDecl::AssociatedType(decl) => decl.ast_idx(db),
            TraitItemSynNodeDecl::AssociatedVal(decl) => decl.ast_idx(db),
        }
    }

    pub fn template_parameters<'a>(self, _db: &'a dyn SynDeclDb) -> &'a [TemplateParameterObelisk] {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TraitItemSynNodeDecl::MethodFn(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedType(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TraitItemSynNodeDecl::MethodFn(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TraitItemSynNodeDecl::AssociatedType(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db)
            }
            TraitItemSynNodeDecl::AssociatedVal(syn_node_decl) => syn_node_decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TraitItemSynNodeDecl::MethodFn(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedType(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedVal(_) => todo!(),
        }
    }
}

impl HasSynNodeDecl for TraitItemSynNodePath {
    type NodeDecl = TraitItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        trai_item_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
fn trai_item_syn_node_decl(
    db: &dyn SynDeclDb,
    syn_node_path: TraitItemSynNodePath,
) -> TraitItemSynNodeDecl {
    let parser = DeclParserFactory::new(db, syn_node_path.module_path(db));
    parser.parse_trai_item_syn_node_decl(syn_node_path)
}

impl<'a> DeclParserFactory<'a> {
    fn parse_trai_item_syn_node_decl(
        &self,
        syn_node_path: TraitItemSynNodePath,
    ) -> TraitItemSynNodeDecl {
        let db = self.db();
        let node = syn_node_path.syn_node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                item_kind:
                    EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TraitItem(item_kind),
                    },
                saved_stream_state,
                ..
            } => self.parse_trai_item_syn_node_decl_aux(
                syn_node_path,
                node,
                ast_idx,
                token_group_idx,
                item_kind,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    pub(super) fn parse_trai_item_syn_node_decl_aux(
        &self,
        syn_node_path: TraitItemSynNodePath,
        node: TraitItemSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        trai_item_kind: TraitItemKind,
        saved_stream_state: TokenStreamState,
    ) -> TraitItemSynNodeDecl {
        match trai_item_kind {
            TraitItemKind::MethodFn => self
                .parse_trai_method_fn_node_decl(
                    syn_node_path,
                    node,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
                .into(),
            TraitItemKind::AssociatedType => self
                .parse_trai_associated_ty_node_decl(
                    syn_node_path,
                    node,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
                .into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TraitItemSynDecl {
    AssociatedFn(TraitAssociatedFnSynDecl),
    MethodFn(TraitMethodFnSynDecl),
    AssociatedType(TraitAssociatedTypeSynDecl),
    AssociatedVal(TraitAssociatedValSynDecl),
}

impl TraitItemSynDecl {
    pub fn path(self, _db: &dyn SynDeclDb) -> TraitItemPath {
        match self {
            TraitItemSynDecl::AssociatedFn(_) => todo!(),
            TraitItemSynDecl::MethodFn(_) => todo!(),
            TraitItemSynDecl::AssociatedType(_) => todo!(),
            TraitItemSynDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn template_parameters<'a>(self, _db: &'a dyn SynDeclDb) -> &'a [TemplateParameterObelisk] {
        match self {
            TraitItemSynDecl::AssociatedFn(_) => todo!(),
            TraitItemSynDecl::MethodFn(_) => todo!(),
            TraitItemSynDecl::AssociatedType(_) => todo!(),
            TraitItemSynDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, _db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            TraitItemSynDecl::AssociatedFn(_) => todo!(),
            TraitItemSynDecl::MethodFn(_) => todo!(),
            TraitItemSynDecl::AssociatedType(_) => todo!(),
            TraitItemSynDecl::AssociatedVal(_) => todo!(),
        }
    }
}

impl HasSynDecl for TraitItemPath {
    type Decl = TraitItemSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        todo!()
    }
}

impl<'a> DeclParserFactory<'a> {}
