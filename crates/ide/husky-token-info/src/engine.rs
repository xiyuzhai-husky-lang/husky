use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_regional_token::{RegionalTokenIdx, RegionalTokenIdxBase};
use husky_syn_decl::HasSynNodeDecl;
use husky_syn_defn::*;

use husky_entity_syn_tree::{
    helpers::{
        paths::module_item_syn_node_paths,
        tokra_region::{HasDeclTokraRegion, HasSynDefnTokraRegion},
    },
    ParentUseExpr,
};
use husky_entity_taxonomy::EntityKind;
use husky_sema_expr::SemaExprRegion;
use husky_syn_expr::*;

pub(crate) struct TokenInfoEngine<'a> {
    db: &'a dyn TokenInfoDb,
    module_path: ModulePath,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    item_tree_presheet: &'a EntitySynTreePresheet,
    item_tree_sheet: &'a EntitySynTreeSheet,
    module_symbol_context: ModuleSymbolContext<'a>,
    sheet: TokenInfoSheet,
}

impl<'a> TokenInfoEngine<'a> {
    pub(crate) fn new(
        db: &'a dyn TokenInfoDb,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<Self> {
        let token_sheet_data = &db.token_sheet_data(module_path)?;
        Ok(Self {
            db,
            module_path,
            token_sheet_data,
            ast_sheet: db.ast_sheet(module_path)?,
            item_tree_presheet: db.item_syn_tree_presheet(module_path)?,
            item_tree_sheet: db.item_syn_tree_sheet(module_path)?,
            sheet: TokenInfoSheet::new(token_sheet_data),
            module_symbol_context: db.module_symbol_context(module_path)?,
        })
    }

    pub(crate) fn visit_all(mut self) -> EntitySynTreeResult<TokenInfoSheet> {
        self.visit_syn_nodes()?;
        self.visit_once_use_rules();
        Ok(self.sheet)
    }

    fn visit_syn_nodes(&mut self) -> EntitySynTreeResult<()> {
        for syn_node_path in module_item_syn_node_paths(self.db, self.module_path)
            .as_ref()
            .expect("deprecated error")
            .iter()
            .copied()
        {
            self.visit_syn_node(syn_node_path)
        }
        Ok(())
    }

    fn visit_once_use_rules(&mut self) {
        for (rule_idx, rule) in self.item_tree_sheet.once_use_rule_indexed_iter() {
            self.visit_once_use_rule(rule, rule_idx);
        }
    }

    fn visit_once_use_rule(&mut self, rule: &OnceUseRule, rule_idx: OnceUseRuleIdx) {
        let use_expr_idx = rule.use_expr_idx();
        let use_expr = &self.item_tree_presheet[use_expr_idx];
        match use_expr {
            UseExpr::All { star_token } => self
                .sheet
                .add(star_token.token_idx(), TokenInfo::UseExprStar),
            UseExpr::Leaf { ident_token } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfo::UseExpr {
                    use_expr_idx,
                    rule_idx,
                    state: rule.state(),
                },
            ),
            UseExpr::Parent(ParentUseExpr {
                parent_name_token, ..
            }) => self.sheet.add(
                parent_name_token.token_idx(),
                TokenInfo::UseExpr {
                    use_expr_idx,
                    rule_idx,
                    state: rule.state(),
                },
            ),
            UseExpr::Err(_) => (),
            UseExpr::SelfOne { self_mod_token: _ } => todo!(),
        }
    }

    fn visit_syn_node(&mut self, syn_node_path: ItemSynNodePath) {
        let db = self.db;
        if let Some(syn_expr_region) = syn_node_path.syn_node_decl(db).syn_expr_region(db) {
            self.visit_expr_region(syn_expr_region)
        }
        let syn_node_defn = syn_node_path.syn_node_defn(db);
        if let Some(syn_expr_region) = syn_node_defn.syn_expr_region(db) {
            self.visit_expr_region(syn_expr_region)
        }
        match self.ast_sheet[syn_node_path.decl_ast_idx(db)] {
            Ast::Identifiable {
                ident_token,
                item_kind,
                ..
            } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfo::EntityNode(syn_node_path, item_kind),
            ),
            Ast::ImplBlock { .. } => (),
            // ad hoc
            Ast::Attr { .. } => (),
            _ => unreachable!(),
        }
        match syn_node_defn {
            SynNodeDefn::MajorItem(defn) => self.visit_module_item_node(defn),
            SynNodeDefn::AssociatedItem(defn) => self.visit_associated_item(defn),
            SynNodeDefn::TypeVariant(_) => todo!(),
            SynNodeDefn::ImplBlock(_) => (),
            SynNodeDefn::Submodule(_) => (),
            SynNodeDefn::Attr(_) => (),
        }
    }

    fn visit_expr_region(&mut self, syn_expr_region: SynExprRegion) {
        DeclTokenInfoEngine::new(self, syn_expr_region).visit_all()
    }

    fn visit_module_item_node(&mut self, defn: MajorItemSynNodeDefn) {
        match defn {
            MajorItemSynNodeDefn::Type(defn) => self.visit_ty(defn),
            MajorItemSynNodeDefn::Trait(defn) => self.visit_trai(defn),
            MajorItemSynNodeDefn::Fugitive(defn) => self.visit_fugitive_syn_node(defn),
        }
    }

    fn visit_ty(&mut self, defn: TypeSynNodeDefn) {
        match defn {
            TypeSynNodeDefn::Enum(defn) => self.visit_enum_ty(defn),
            TypeSynNodeDefn::Inductive(defn) => self.visit_inductive_ty(defn),
            TypeSynNodeDefn::Record(defn) => self.visit_record_ty(defn),
            TypeSynNodeDefn::UnitStruct(defn) => self.visit_unit_struct_ty(defn),
            TypeSynNodeDefn::TupleStruct(defn) => self.visit_tuple_struct_ty(defn),
            TypeSynNodeDefn::PropsStruct(defn) => self.visit_props_struct_ty(defn),
            TypeSynNodeDefn::Structure(defn) => self.visit_structure_ty(defn),
            TypeSynNodeDefn::Extern(defn) => self.visit_alias_ty(defn),
            TypeSynNodeDefn::Union(_) => todo!(),
        }
    }

    fn visit_enum_ty(&mut self, _defn: EnumTypeSynNodeDefn) {
        // todo!()
    }

    fn visit_inductive_ty(&mut self, _defn: InductiveTypeSynNodeDefn) {
        // todo!()
    }

    fn visit_record_ty(&mut self, _defn: RecordTypeSynNodeDefn) {
        // todo!()
    }

    fn visit_unit_struct_ty(&mut self, _defn: UnitStructTypeSynNodeDefn) {
        // todo!()
    }

    fn visit_tuple_struct_ty(&mut self, _defn: TupleStructTypeSynNodeDefn) {
        // todo!()
    }

    fn visit_props_struct_ty(&mut self, _defn: PropsStructTypeSynNodeDefn) {
        // todo!()
    }

    fn visit_structure_ty(&mut self, _defn: StructureTypeSynNodeDefn) {
        // todo!()
    }

    fn visit_alias_ty(&mut self, _defn: ExternTypeSynNodeDefn) {
        // todo!()
    }

    fn visit_trai(&mut self, _defn: TraitSynNodeDefn) {
        //todo!()
    }

    fn visit_fugitive_syn_node(&mut self, defn: FugitiveSynNodeDefn) {
        match defn {
            FugitiveSynNodeDefn::Fn(defn) => self.visit_fn_node(defn),
            FugitiveSynNodeDefn::Val(defn) => self.visit_val_node(defn),
            FugitiveSynNodeDefn::Gn(defn) => self.visit_gn_node(defn),
        }
    }

    fn visit_fn_node(&mut self, syn_node_defn: FnSynNodeDefn) {}

    fn visit_val_node(&mut self, syn_node_defn: ValSynNodeDefn) {}

    fn visit_gn_node(&mut self, syn_node_defn: GnSynNodeDefn) {
        let syn_node_decl = syn_node_defn.syn_node_decl(self.db);
        // todo!()
    }

    fn visit_value(&mut self, syn_node_defn: ValSynNodeDefn) {
        let syn_node_decl = syn_node_defn.syn_node_decl(self.db);
        // todo!()
    }

    fn visit_associated_item(&mut self, syn_node_defn: AssociatedItemSynNodeDefn) {
        match syn_node_defn {
            AssociatedItemSynNodeDefn::TypeItem(syn_node_defn) => {
                self.visit_ty_item_syn_node(syn_node_defn)
            }
            AssociatedItemSynNodeDefn::TraitItem(syn_node_defn) => {
                self.visit_trai_item_node(syn_node_defn)
            }
            AssociatedItemSynNodeDefn::TraitForTypeItem(syn_node_defn) => {
                self.visit_trai_for_ty_item_syn_node(syn_node_defn)
            }
        }
    }

    fn visit_ty_item_syn_node(&self, syn_node_defn: TypeItemSynNodeDefn) {
        // todo!()
    }

    fn visit_trai_item_node(&self, syn_node_defn: TraitItemSynNodeDefn) {
        // todo!()
    }

    fn visit_trai_for_ty_item_syn_node(&self, syn_node_defn: TraitForTypeItemSynNodeDefn) {
        // todo!()
    }
}

struct DeclTokenInfoEngine<'a, 'b> {
    db: &'a dyn TokenInfoDb,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    expr_region_data: &'a SynExprRegionData,
    sema_expr_region: &'a SemaExprRegion,
    sheet: &'b mut TokenInfoSheet,
    syn_expr_region: ExprRegionLeash,
    regional_token_idx_base: RegionalTokenIdxBase,
}

impl<'a, 'b> DeclTokenInfoEngine<'a, 'b> {
    fn new(
        engine: &'b mut TokenInfoEngine<'a>,
        syn_expr_region: SynExprRegion,
    ) -> DeclTokenInfoEngine<'a, 'b> {
        let expr_region_data = syn_expr_region.data(engine.db);
        let db = engine.db;
        DeclTokenInfoEngine {
            db,
            token_sheet_data: engine.token_sheet_data,
            ast_sheet: engine.ast_sheet,
            sheet: &mut engine.sheet,
            expr_region_data,
            sema_expr_region: db.sema_expr_region(syn_expr_region),
            syn_expr_region: syn_expr_region.into(),
            regional_token_idx_base: match expr_region_data.path() {
                RegionPath::Snippet(_) => todo!(),
                RegionPath::Decl(path) => path.decl_regional_token_idx_base(db),
                RegionPath::Defn(path) => path.defn_regional_token_idx_base(db).expect("todo"),
            },
        }
    }

    fn add(&mut self, regional_token_idx: RegionalTokenIdx, token_info: TokenInfo) {
        let base = self.regional_token_idx_base;
        self.sheet
            .add(regional_token_idx.token_idx(base), token_info)
    }

    fn override_add(&mut self, regional_token_idx: RegionalTokenIdx, token_info: TokenInfo) {
        let base = self.regional_token_idx_base;
        self.sheet
            .override_add(regional_token_idx.token_idx(base), token_info)
    }

    fn visit_all(mut self) {
        for (expr_idx, expr) in self.expr_region_data.expr_arena().indexed_iter() {
            self.visit_expr(expr_idx, expr)
        }
        for item_path_expr in self
            .expr_region_data
            .principal_item_path_expr_arena()
            .iter()
        {
            self.visit_item_path_expr(item_path_expr)
        }
        for (current_symbol_idx, current_symbol) in self
            .expr_region_data
            .symbol_region()
            .current_symbol_indexed_iter()
        {
            self.visit_current_symbol(current_symbol_idx, current_symbol)
        }
    }

    fn visit_expr(&mut self, expr_idx: SynExprIdx, expr: &SynExprData) {
        match expr {
            SynExprData::CurrentSymbol {
                regional_token_idx,
                current_symbol_idx,
                current_symbol_kind,
                ..
            }
            | SynExprData::FrameVarDecl {
                regional_token_idx,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
                ..
            } => self.add(
                *regional_token_idx,
                TokenInfo::CurrentSymbol {
                    current_symbol_idx: *current_symbol_idx,
                    current_symbol_kind: *current_symbol_kind,
                    syn_expr_region: self.syn_expr_region,
                },
            ),
            SynExprData::InheritedSymbol {
                regional_token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
                ..
            } => self.add(
                *regional_token_idx,
                TokenInfo::InheritedSymbol {
                    inherited_symbol_idx: *inherited_symbol_idx,
                    syn_expr_region: self.syn_expr_region,
                    inherited_symbol_kind: *inherited_symbol_kind,
                },
            ),
            SynExprData::SelfType(regional_token_idx) => {
                self.add(*regional_token_idx, TokenInfo::SelfType)
            }
            SynExprData::SelfValue(regional_token_idx) => {
                self.add(*regional_token_idx, TokenInfo::SelfValue)
            }
            SynExprData::Field { ident_token, .. } => {
                self.add(ident_token.regional_token_idx(), TokenInfo::Field)
            }
            SynExprData::MethodApplicationOrCall { ident_token, .. } => {
                self.add(ident_token.regional_token_idx(), TokenInfo::Method)
            }
            SynExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => {
                if let Some(_) = place_label_regional_token {
                    todo!()
                }
                // ad hoc
                // self.add(*at_regional_token_idx, TokenInfo::Method)
            }
            SynExprData::Literal(_, _)
            | SynExprData::PrincipalEntityPath { .. }
            | SynExprData::AssociatedItem { .. }
            | SynExprData::Binary { .. }
            | SynExprData::Prefix { .. }
            | SynExprData::Suffix { .. }
            | SynExprData::TemplateInstantiation { .. }
            | SynExprData::NewTuple { .. }
            | SynExprData::List { .. }
            | SynExprData::Bracketed { .. }
            | SynExprData::Err(_)
            | SynExprData::Block { .. }
            | SynExprData::Be { .. } => (),
            SynExprData::BoxColonList { .. } => (),
            SynExprData::FunctionApplicationOrCall { function, .. }
            | SynExprData::ExplicitApplication {
                function_expr_idx: function,
                ..
            } => match self.expr_region_data[*function] {
                SynExprData::List {
                    lbox_regional_token_idx,
                    items: _,
                    rbox_regional_token_idx,
                } => {
                    self.add(lbox_regional_token_idx, TokenInfo::BoxPrefix);
                    self.add(rbox_regional_token_idx, TokenInfo::BoxPrefix)
                }
                SynExprData::BoxColonList {
                    lbox_regional_token_idx,
                    colon_regional_token_idx,
                    rbox_regional_token_idx,
                    ..
                } => {
                    self.add(lbox_regional_token_idx, TokenInfo::BoxColon);
                    self.add(colon_regional_token_idx, TokenInfo::BoxColon);
                    self.add(rbox_regional_token_idx, TokenInfo::BoxColon)
                }
                _ => (),
            },
            SynExprData::IndexOrCompositionWithList {
                owner: _,
                lbox_regional_token_idx: _,
                items: _indices,
                rbox_regional_token_idx: _,
            } => {
                ()
                // ad hoc
                // this should always be some
                // todo!()
                // match self.sema_expr_region.expr_disambiguation(expr_idx) {
                //     Some(Ok(disambiguation)) => match disambiguation {
                //         SynExprDisambiguation::IndexOrComposeWithList(disambiguation) => {
                //             match disambiguation {
                //                 IndexOrComposeWithListExprDisambiguation::Index(_) => (),
                //                 IndexOrComposeWithListExprDisambiguation::ComposeWithList => {
                //                     todo!()
                //                 }
                //             }
                //         }
                //         _ => unreachable!(),
                //     },
                //     None | Some(Err(_)) => (),
                // }
            }
            SynExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => {
                self.add(*lpar_regional_token_idx, TokenInfo::UnitLeftParenthesis);
                self.add(*rpar_regional_token_idx, TokenInfo::UnitRightParenthesis);
            }
            SynExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => {
                self.add(
                    function_ident.regional_token_idx(),
                    TokenInfo::HtmlFunctionIdent,
                );
                for argument in arguments.iter() {
                    match argument {
                        SynHtmlArgumentExpr::Expanded { property_ident, .. }
                        | SynHtmlArgumentExpr::Shortened { property_ident, .. } => self.add(
                            property_ident.regional_token_idx(),
                            TokenInfo::HtmlPropertyIdent,
                        ),
                    }
                }
            }
            SynExprData::FunctionCall { .. } => (),
            SynExprData::Ritchie { .. } => (),
            SynExprData::Sorry { regional_token_idx } => todo!(),
            SynExprData::Todo { regional_token_idx } => {
                self.add(*regional_token_idx, TokenInfo::Todo)
            }
            SynExprData::Unreachable { regional_token_idx } => {
                self.add(*regional_token_idx, TokenInfo::Unreachable)
            }
        }
    }

    fn visit_item_path_expr(&mut self, item_path_expr: &SynPrincipalEntityPathExpr) {
        match item_path_expr {
            SynPrincipalEntityPathExpr::Root {
                principal_entity_path,
                path_name_token,
                ..
            } => self.add(
                path_name_token.regional_token_idx(),
                TokenInfo::Entity((*principal_entity_path).into()),
            ),
            SynPrincipalEntityPathExpr::Subitem {
                path: Ok(path),
                ident_token: Ok(ident_token),
                ..
            } => self.add(
                ident_token.regional_token_idx(),
                TokenInfo::Entity((*path).into()),
            ),
            SynPrincipalEntityPathExpr::Subitem { .. } => (),
        }
    }

    fn visit_current_symbol(
        &mut self,
        current_symbol_idx: SynCurrentSymbolIdx,
        current_symbol: &SynCurrentSymbol,
    ) {
        let current_symbol_kind = current_symbol.kind();
        match current_symbol_kind {
            SynCurrentSymbolKind::LetVariable {
                pattern_symbol_idx: pattern_symbol,
            }
            | SynCurrentSymbolKind::BeVariable {
                pattern_symbol_idx: pattern_symbol,
            }
            | SynCurrentSymbolKind::CaseVariable {
                pattern_symbol_idx: pattern_symbol,
            }
            | SynCurrentSymbolKind::ExplicitRegularParameter {
                pattern_symbol_idx: pattern_symbol,
            } => match self.expr_region_data[pattern_symbol] {
                SynPatternSymbol::Atom(pattern_expr_idx) => {
                    match self.expr_region_data[pattern_expr_idx] {
                        SynPatternExpr::Ident {
                            ident_token,
                            symbol_modifier_tokens: _,
                        } => self.override_add(
                            ident_token.regional_token_idx(),
                            TokenInfo::CurrentSymbol {
                                current_symbol_idx,
                                syn_expr_region: self.syn_expr_region,
                                current_symbol_kind,
                            },
                        ),
                        _ => unreachable!(),
                    }
                }
            },
            SynCurrentSymbolKind::FrameVariable(_) => (),
            SynCurrentSymbolKind::ImplicitParameter {
                template_parameter_kind,
            } => match template_parameter_kind {
                CurrentImplicitParameterSynSymbolKind::Type { ident_token } => self.add(
                    ident_token.regional_token_idx(),
                    TokenInfo::CurrentSymbol {
                        current_symbol_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_symbol_kind,
                    },
                ),
                CurrentImplicitParameterSynSymbolKind::Lifetime { label_token } => self.add(
                    label_token.regional_token_idx(),
                    TokenInfo::CurrentSymbol {
                        current_symbol_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_symbol_kind,
                    },
                ),
                CurrentImplicitParameterSynSymbolKind::Place { label_token } => self.add(
                    label_token.regional_token_idx(),
                    TokenInfo::CurrentSymbol {
                        current_symbol_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_symbol_kind,
                    },
                ),
                CurrentImplicitParameterSynSymbolKind::Constant { ident_token } => self.add(
                    ident_token.regional_token_idx(),
                    TokenInfo::CurrentSymbol {
                        current_symbol_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_symbol_kind,
                    },
                ),
            },
            SynCurrentSymbolKind::ExplicitVariadicParameter { ident_token } => self.add(
                ident_token.regional_token_idx(),
                TokenInfo::CurrentSymbol {
                    current_symbol_idx,
                    syn_expr_region: self.syn_expr_region,
                    current_symbol_kind,
                },
            ),
            SynCurrentSymbolKind::FieldVariable { ident_token } => self.add(
                ident_token.regional_token_idx(),
                TokenInfo::CurrentSymbol {
                    current_symbol_idx,
                    syn_expr_region: self.syn_expr_region,
                    current_symbol_kind,
                },
            ),
        }
    }
}
