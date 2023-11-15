mod call_list;
mod html;

pub use self::call_list::*;
pub use self::html::*;

use crate::*;
use husky_entity_path::{
    AssociatedItemPath, FugitivePath, MajorItemPath, PrincipalEntityPath, TypePath, TypeVariantPath,
};
use husky_fluffy_term::MethodFluffySignature;
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::HirConstSymbol;
use husky_sema_expr::{SemaExprData, SemaExprIdx};
use husky_term_prelude::TermLiteral;
use idx_arena::ArenaRef;

pub type HirLazyExprArena = Arena<HirLazyExprData>;
pub type HirLazyExprArenaRef<'a> = ArenaRef<'a, HirLazyExprData>;
pub type HirLazyExprIdx = ArenaIdx<HirLazyExprData>;
pub type HirLazyExprIdxRange = ArenaIdxRange<HirLazyExprData>;
pub type HirLazyExprMap<V> = ArenaMap<HirLazyExprData, V>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirLazyExprDb)]
pub enum HirLazyExprData {
    Literal(TermLiteral),
    PrincipalEntityPath(PrincipalEntityPath),
    ConstSymbol(HirConstSymbol),
    Variable(HirLazyVariableIdx),
    Binary {
        lopd: HirLazyExprIdx,
        opr: HirBinaryOpr,
        ropd: HirLazyExprIdx,
    },
    Be {
        src: HirLazyExprIdx,
        target: HirLazyBeVariablesPattern,
    },
    Prefix {
        opr: HirPrefixOpr,
        opd_hir_expr_idx: HirLazyExprIdx,
    },
    Suffix {
        opd_hir_expr_idx: HirLazyExprIdx,
        opr: HirSuffixOpr,
    },
    TypeConstructorFnCall {
        path: TypePath,
        function_hir_lazy_expr_idx: HirLazyExprIdx,
        template_arguments: Option<HirLazyTemplateArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    TypeVariantConstructorFnCall {
        path: TypeVariantPath,
        function_hir_lazy_expr_idx: HirLazyExprIdx,
        template_arguments: Option<HirLazyTemplateArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    FunctionFnItemCall {
        path: FugitivePath,
        function_hir_lazy_expr_idx: HirLazyExprIdx,
        template_arguments: Option<HirLazyTemplateArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    FunctionGnCall {
        path: FugitivePath,
        function_hir_lazy_expr_idx: HirLazyExprIdx,
        template_arguments: Option<HirLazyTemplateArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    AssociatedFunctionFnCall {
        path: AssociatedItemPath,
        function_hir_lazy_expr_idx: HirLazyExprIdx,
        parent_template_arguments: Option<HirLazyTemplateArgumentList>,
        template_arguments: Option<HirLazyTemplateArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    Field {
        owner: HirLazyExprIdx,
        ident: Ident,
    },
    MethodFnCall {
        self_argument: HirLazyExprIdx,
        ident: Ident,
        path: AssociatedItemPath,
        template_arguments: Option<HirLazyTemplateArgumentList>,
        item_groups: SmallVec<[HirLazyCallListItemGroup; 4]>,
    },
    NewTuple {
        /// guaranteed that items.len() > 0
        items: SmallVec<[HirLazyExprIdx; 4]>,
    },
    Index {
        owner: HirLazyExprIdx,
        items: SmallVec<[HirLazyExprIdx; 4]>,
    },
    NewList {
        items: SmallVec<[HirLazyExprIdx; 4]>,
    },
    Block {
        stmts: HirLazyStmtIdxRange,
    },
    // todo: handle container
    EmptyHtmlTag {
        function_ident: Ident,
        arguments: IdentMap<HirLazyHtmlArgumentExpr>,
    },
    Todo,
    AssociatedFn {
        // ad hoc, needs more
        path: AssociatedItemPath,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirLazyTemplateArgumentList {/*todo */}

impl ToHirLazy for SemaExprIdx {
    type Output = HirLazyExprIdx;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        let hir_lazy_expr = match *self.data(builder.sema_expr_arena_ref()) {
            SemaExprData::Literal(_, _) => {
                let EtherealTerm::Literal(lit) = builder.expr_term(*self) else {
                    unreachable!()
                };
                HirLazyExprData::Literal(lit)
            }
            SemaExprData::PrincipalEntityPath {
                path_expr_idx: _,
                path,
                ty_path_disambiguation: _,
            } => HirLazyExprData::PrincipalEntityPath(path),
            SemaExprData::AssociatedItem { .. } => todo!(),
            SemaExprData::InheritedSynSymbol {
                inherited_syn_symbol_idx,
                ..
            } => HirLazyExprData::Variable(
                builder
                    .inherited_syn_symbol_to_hir_lazy_variable(inherited_syn_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::CurrentSynSymbol {
                current_syn_symbol_idx,
                ..
            } => HirLazyExprData::Variable(
                builder
                    .current_syn_symbol_to_hir_lazy_variable(current_syn_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::FrameVarDecl {
                frame_var_symbol_idx,
                ..
            } => HirLazyExprData::Variable(
                builder
                    .current_syn_symbol_to_hir_lazy_variable(frame_var_symbol_idx)
                    .unwrap(),
            ),
            SemaExprData::SelfType(_) => todo!(),
            SemaExprData::SelfValue(_) => todo!(),
            SemaExprData::Binary {
                lopd, opr, ropd, ..
            } => HirLazyExprData::Binary {
                lopd: lopd.to_hir_lazy(builder),
                opr: HirBinaryOpr::from_sema(opr),
                ropd: ropd.to_hir_lazy(builder),
            },
            SemaExprData::Be {
                src,
                be_regional_token_idx: _,
                ref target,
            } => HirLazyExprData::Be {
                src: src.to_hir_lazy(builder),
                target: target.to_hir_lazy(builder),
            },
            SemaExprData::Prefix {
                opr,
                opd_sema_expr_idx,
                ..
            } => HirLazyExprData::Prefix {
                opr: HirPrefixOpr::from_sema(opr),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_lazy(builder),
            },
            SemaExprData::Suffix {
                opd_sema_expr_idx,
                opr,
                ..
            } => HirLazyExprData::Suffix {
                opr: HirSuffixOpr::from_sema(opr),
                opd_hir_expr_idx: opd_sema_expr_idx.to_hir_lazy(builder),
            },
            SemaExprData::FunctionApplication {
                function_sema_expr_idx: _,
                argument_sema_expr_idx: _,
            } => {
                todo!()
            }
            SemaExprData::FunctionFnCall {
                function_sema_expr_idx,
                ref template_arguments,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let function_hir_lazy_expr_idx = function_sema_expr_idx.to_hir_lazy(builder);
                let template_arguments = template_arguments.as_ref().map(|_| todo!());
                let item_groups =
                    builder.new_call_list_item_groups(ritchie_parameter_argument_matches);
                match builder.hir_lazy_expr_arena()[function_hir_lazy_expr_idx] {
                    HirLazyExprData::PrincipalEntityPath(path) => match path {
                        PrincipalEntityPath::Module(_) => unreachable!(),
                        PrincipalEntityPath::MajorItem(path) => match path {
                            MajorItemPath::Type(path) => HirLazyExprData::TypeConstructorFnCall {
                                function_hir_lazy_expr_idx,
                                path,
                                template_arguments,
                                item_groups,
                            },
                            MajorItemPath::Trait(_) => unreachable!(),
                            MajorItemPath::Fugitive(path) => HirLazyExprData::FunctionFnItemCall {
                                function_hir_lazy_expr_idx,
                                path,
                                template_arguments,
                                item_groups,
                            },
                        },
                        PrincipalEntityPath::TypeVariant(path) => {
                            HirLazyExprData::TypeVariantConstructorFnCall {
                                function_hir_lazy_expr_idx,
                                path,
                                template_arguments,
                                item_groups,
                            }
                        }
                    },
                    HirLazyExprData::AssociatedFn { .. } => todo!(),
                    _ => todo!(),
                }
            }
            SemaExprData::FunctionGnCall { .. } => unreachable!(),
            SemaExprData::Ritchie {
                ritchie_kind_regional_token_idx: _,
                ritchie_kind: _,
                lpar_token: _,
                parameter_ty_items: _,
                rpar_regional_token_idx: _,
                light_arrow_token: _,
                return_ty_sema_expr_idx: _,
            } => todo!(),
            SemaExprData::Field {
                owner_sema_expr_idx,
                ident_token,
                ..
            } => HirLazyExprData::Field {
                owner: owner_sema_expr_idx.to_hir_lazy(builder),
                ident: ident_token.ident(),
            },
            SemaExprData::MethodApplication {
                self_argument: _,
                dot_regional_token_idx: _,
                ident_token: _,
                template_arguments: _,
                lpar_regional_token_idx: _,
                items: _,
                rpar_regional_token_idx: _,
            } => {
                todo!()
            }
            SemaExprData::MethodFnCall {
                self_argument_sema_expr_idx,
                ident_token,
                ref dispatch,
                ref template_arguments,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let MethodFluffySignature::MethodFn(signature) = dispatch.signature() else {
                    unreachable!()
                };
                HirLazyExprData::MethodFnCall {
                    self_argument: self_argument_sema_expr_idx.to_hir_lazy(builder),
                    ident: ident_token.ident(),
                    path: signature.path(),
                    template_arguments: template_arguments.as_ref().map(|_| todo!()),
                    item_groups: builder
                        .new_call_list_item_groups(ritchie_parameter_argument_matches),
                }
            }
            SemaExprData::MethodGnCall { .. } => {
                todo!()
            }
            SemaExprData::TemplateInstantiation {
                template: _,
                template_arguments: _,
            } => todo!(),
            SemaExprData::At {
                at_regional_token_idx: _,
                place_label_regional_token: _,
            } => todo!(),
            SemaExprData::Unit {
                lpar_regional_token_idx: _,
                rpar_regional_token_idx: _,
            } => todo!(),
            SemaExprData::Bracketed {
                lpar_regional_token_idx: _,
                item: _,
                rpar_regional_token_idx: _,
            } => todo!(),
            SemaExprData::NewTuple {
                lpar_regional_token_idx: _,
                items: _,
                rpar_regional_token_idx: _,
            } => todo!(),
            SemaExprData::Index {
                owner_sema_expr_idx,
                ref index_sema_list_items,
                ..
            } => HirLazyExprData::Index {
                owner: owner_sema_expr_idx.to_hir_lazy(builder),
                items: index_sema_list_items
                    .iter()
                    .map(|item| item.sema_expr_idx().to_hir_lazy(builder))
                    .collect(),
            },
            SemaExprData::CompositionWithList { .. } => {
                todo!()
            }
            SemaExprData::NewList { ref items, .. } => HirLazyExprData::NewList {
                items: items
                    .iter()
                    .map(|item| item.sema_expr_idx().to_hir_lazy(builder))
                    .collect(),
            },
            SemaExprData::BoxColonList {
                lbox_regional_token_idx: _,
                colon_regional_token_idx: _,
                items: _,
                rbox_regional_token_idx: _,
            } => todo!(),
            SemaExprData::Block { stmts } => HirLazyExprData::Block {
                stmts: stmts.to_hir_lazy(builder),
            },
            SemaExprData::EmptyHtmlTag {
                empty_html_bra_idx: _,
                function_ident: _,
                arguments: _,
                empty_html_ket: _,
            } => todo!(),
            SemaExprData::Sorry {
                regional_token_idx: _,
            } => todo!(),
            SemaExprData::Todo {
                regional_token_idx: _,
            } => todo!(),
            SemaExprData::Unreachable {
                regional_token_idx: _,
            } => todo!(),
            SemaExprData::VecFunctor {
                lbox_regional_token_idx: _,
                rbox_regional_token_idx: _,
            } => todo!(),
            SemaExprData::ArrayFunctor {
                lbox_regional_token_idx: _,
                items: _,
                rbox_regional_token_idx: _,
            } => todo!(),
        };
        builder.alloc_expr(*self, hir_lazy_expr)
    }
}
