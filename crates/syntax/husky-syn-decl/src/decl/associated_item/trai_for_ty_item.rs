mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;

use super::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemSynNodeDecl {
    AssociatedFn(TraitForTypeAssociatedFnSynNodeDecl),
    MethodFn(TraitForTypeMethodFnSynNodeDecl),
    AssociatedType(TraitForTypeAssociatedTypeSynNodeDecl),
    AssociatedVal(TraitForTypeAssociatedValSynNodeDecl),
}

impl From<TraitForTypeItemSynNodeDecl> for SynNodeDecl {
    fn from(decl: TraitForTypeItemSynNodeDecl) -> Self {
        SynNodeDecl::AssociatedItem(decl.into())
    }
}

impl TraitForTypeItemSynNodeDecl {
    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> TraitForTypeItemSynNodePath {
        match self {
            TraitForTypeItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TraitForTypeItemSynNodeDecl::MethodFn(decl) => decl.syn_node_path(db),
            TraitForTypeItemSynNodeDecl::AssociatedType(decl) => decl.syn_node_path(db),
            TraitForTypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        match self {
            TraitForTypeItemSynNodeDecl::AssociatedFn(decl) => decl.ast_idx(db),
            TraitForTypeItemSynNodeDecl::MethodFn(decl) => decl.ast_idx(db),
            TraitForTypeItemSynNodeDecl::AssociatedType(decl) => decl.ast_idx(db),
            TraitForTypeItemSynNodeDecl::AssociatedVal(decl) => decl.ast_idx(db),
        }
    }

    pub fn template_parameters<'a>(self, _db: &'a dyn SynDeclDb) -> &'a [TemplateParameterObelisk] {
        match self {
            TraitForTypeItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TraitForTypeItemSynNodeDecl::MethodFn(_) => todo!(),
            TraitForTypeItemSynNodeDecl::AssociatedType(_) => todo!(),
            TraitForTypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            TraitForTypeItemSynNodeDecl::AssociatedFn(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynNodeDecl::MethodFn(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynNodeDecl::AssociatedType(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynNodeDecl::AssociatedVal(decl) => decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        match self {
            TraitForTypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => syn_node_decl.errors(db),
            TraitForTypeItemSynNodeDecl::MethodFn(syn_node_decl) => syn_node_decl.errors(db),
            TraitForTypeItemSynNodeDecl::AssociatedType(syn_node_decl) => syn_node_decl.errors(db),
            TraitForTypeItemSynNodeDecl::AssociatedVal(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasSynNodeDecl for TraitForTypeItemSynNodePath {
    type NodeDecl = TraitForTypeItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        trai_for_ty_item_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_item_syn_node_decl(
    db: &dyn SynDeclDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> TraitForTypeItemSynNodeDecl {
    let parser = DeclParser::new(db, syn_node_path.module_path(db));
    parser.parse_trai_for_ty_item_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_trai_for_ty_item_syn_node_decl(
        &self,
        syn_node_path: TraitForTypeItemSynNodePath,
    ) -> TraitForTypeItemSynNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                item_kind:
                    EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TraitForTypeItem(item_kind),
                    },
                saved_stream_state,
                ..
            } => self.parse_trai_for_ty_item_syn_node_decl_aux(
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

    pub(super) fn parse_trai_for_ty_item_syn_node_decl_aux(
        &self,
        syn_node_path: TraitForTypeItemSynNodePath,
        node: TraitForTypeItemSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        trai_item_kind: TraitItemKind,
        saved_stream_state: TokenStreamState,
    ) -> TraitForTypeItemSynNodeDecl {
        match trai_item_kind {
            TraitItemKind::MethodFn => self
                .parse_trai_for_ty_method_fn_node_decl(
                    syn_node_path,
                    node,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
                .into(),
            TraitItemKind::AssociatedType => self
                .parse_trai_for_ty_associated_ty_node_decl(
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
pub enum TraitForTypeItemSynDecl {
    AssociatedFn(TraitForTypeAssociatedFnSynDecl),
    MethodFn(TraitForTypeMethodFnSynDecl),
    AssociatedType(TraitForTypeAssociatedTypeSynDecl),
    AssociatedVal(TraitForTypeAssociatedValSynDecl),
}

impl From<TraitForTypeItemSynDecl> for SynDecl {
    fn from(decl: TraitForTypeItemSynDecl) -> Self {
        SynDecl::AssociatedItem(decl.into())
    }
}

impl TraitForTypeItemSynDecl {
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TraitForTypeItemPath,
        syn_node_decl: TraitForTypeItemSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            TraitForTypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => {
                TraitForTypeAssociatedFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitForTypeItemSynNodeDecl::MethodFn(syn_node_decl) => {
                TraitForTypeMethodFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitForTypeItemSynNodeDecl::AssociatedType(syn_node_decl) => {
                TraitForTypeAssociatedTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitForTypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
        })
    }

    pub fn path(self, db: &dyn SynDeclDb) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemSynDecl::AssociatedFn(decl) => decl.path(db),
            TraitForTypeItemSynDecl::MethodFn(decl) => decl.path(db),
            TraitForTypeItemSynDecl::AssociatedType(decl) => decl.path(db),
            TraitForTypeItemSynDecl::AssociatedVal(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [TemplateParameterObelisk] {
        match self {
            TraitForTypeItemSynDecl::AssociatedFn(decl) => decl.template_parameters(db),
            TraitForTypeItemSynDecl::MethodFn(decl) => decl.template_parameters(db),
            TraitForTypeItemSynDecl::AssociatedType(decl) => decl.template_parameters(db),
            TraitForTypeItemSynDecl::AssociatedVal(_) => &[],
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            TraitForTypeItemSynDecl::AssociatedFn(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynDecl::MethodFn(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynDecl::AssociatedType(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynDecl::AssociatedVal(decl) => decl.syn_expr_region(db),
        }
    }
}

impl HasSynDecl for TraitForTypeItemPath {
    type Decl = TraitForTypeItemSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        trai_for_ty_item_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_item_syn_decl(
    db: &dyn SynDeclDb,
    path: TraitForTypeItemPath,
) -> DeclResult<TraitForTypeItemSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    TraitForTypeItemSynDecl::from_node_decl(db, path, syn_node_decl)
}
