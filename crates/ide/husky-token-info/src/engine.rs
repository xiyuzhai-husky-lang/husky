use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_defn::*;

use husky_entity_taxonomy::EntityKind;
use husky_entity_tree::ParentUseExpr;
use husky_expr_ty::{ExprDisambiguation, ExprTypeRegion, IndexOrComposeWithListExprDisambiguation};
use husky_syn_expr::*;

pub(crate) struct InferEngine<'a> {
    db: &'a dyn TokenInfoDb,
    module_path: ModulePath,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    entity_tree_presheet: &'a EntityTreePresheet,
    entity_tree_sheet: &'a EntityTreeSheet,
    module_symbol_context: ModuleSymbolContext<'a>,
    sheet: TokenInfoSheet,
}

impl<'a> InferEngine<'a> {
    pub(crate) fn new(db: &'a dyn TokenInfoDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let token_sheet_data = &db.token_sheet_data(module_path)?;
        Ok(Self {
            db,
            module_path,
            token_sheet_data,
            ast_sheet: db.ast_sheet(module_path)?,
            entity_tree_presheet: db.entity_tree_presheet(module_path)?,
            entity_tree_sheet: db.entity_tree_sheet(module_path)?,
            sheet: TokenInfoSheet::new(token_sheet_data),
            module_symbol_context: db.module_symbol_context(module_path)?,
        })
    }

    pub(crate) fn visit_all(mut self) -> EntityTreeResult<TokenInfoSheet> {
        self.visit_nodes()?;
        self.visit_once_use_rules();
        Ok(self.sheet)
    }

    fn visit_nodes(&mut self) -> EntityTreeResult<()> {
        for node_defn in self.module_path.node_defns(self.db)?.iter().copied() {
            self.visit_node(node_defn)
        }
        Ok(())
    }

    fn visit_once_use_rules(&mut self) {
        for (rule_idx, rule) in self.entity_tree_sheet.once_use_rule_indexed_iter() {
            self.visit_once_use_rule(rule, rule_idx);
        }
    }

    fn visit_once_use_rule(&mut self, rule: &OnceUseRule, rule_idx: OnceUseRuleIdx) {
        let use_expr_idx = rule.use_expr_idx();
        let use_expr = &self.entity_tree_presheet[use_expr_idx];
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

    fn visit_node(&mut self, node_defn: NodeDefn) {
        let node_decl = node_defn.node_decl(self.db);
        if let Some(expr_region) = node_decl.expr_region(self.db) {
            self.visit_expr_region(expr_region)
        }
        if let Some(expr_region) = node_defn.expr_region(self.db) {
            self.visit_expr_region(expr_region)
        }
        let ast_idx = node_defn.ast_idx(self.db);
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                ident_token,
                entity_kind,
                ..
            } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfo::EntityNode(node_decl.node_path(self.db), entity_kind),
            ),
            Ast::ImplBlock { .. } => (),
            _ => unreachable!(),
        }
        match node_defn {
            NodeDefn::ModuleItem(defn) => self.visit_module_item_node(defn),
            NodeDefn::AssociatedItem(defn) => self.visit_associated_item(defn),
            NodeDefn::TypeVariant(_) => todo!(),
            NodeDefn::ImplBlock(_) => (),
            NodeDefn::Submodule(_) => (),
        }
    }

    fn visit_expr_region(&mut self, expr_region: ExprRegion) {
        InferContext {
            db: self.db,
            token_sheet_data: self.token_sheet_data,
            ast_sheet: self.ast_sheet,
            sheet: &mut self.sheet,
            expr_region_data: expr_region.data(self.db),
            expr_ty_region: self.db.expr_ty_region(expr_region),
            expr_region: expr_region.into(),
        }
        .visit_all()
    }

    fn visit_module_item_node(&mut self, defn: ModuleItemNodeDefn) {
        match defn {
            ModuleItemNodeDefn::Type(defn) => self.visit_ty(defn),
            ModuleItemNodeDefn::Trait(defn) => self.visit_trai(defn),
            ModuleItemNodeDefn::Fugitive(defn) => self.visit_fugitive_node(defn),
        }
    }

    fn visit_ty(&mut self, defn: TypeNodeDefn) {
        match defn {
            TypeNodeDefn::Enum(defn) => self.visit_enum_ty(defn),
            TypeNodeDefn::Inductive(defn) => self.visit_inductive_ty(defn),
            TypeNodeDefn::Record(defn) => self.visit_record_ty(defn),
            TypeNodeDefn::UnitStruct(defn) => self.visit_unit_struct_ty(defn),
            TypeNodeDefn::TupleStruct(defn) => self.visit_tuple_struct_ty(defn),
            TypeNodeDefn::PropsStruct(defn) => self.visit_props_struct_ty(defn),
            TypeNodeDefn::Structure(defn) => self.visit_structure_ty(defn),
            TypeNodeDefn::Extern(defn) => self.visit_alias_ty(defn),
            TypeNodeDefn::Union(_) => todo!(),
        }
    }

    fn visit_enum_ty(&mut self, _defn: EnumTypeNodeDefn) {
        // todo!()
    }

    fn visit_inductive_ty(&mut self, _defn: InductiveTypeNodeDefn) {
        // todo!()
    }

    fn visit_record_ty(&mut self, _defn: RecordTypeNodeDefn) {
        // todo!()
    }

    fn visit_unit_struct_ty(&mut self, _defn: UnitStructTypeNodeDefn) {
        // todo!()
    }

    fn visit_tuple_struct_ty(&mut self, _defn: TupleStructTypeNodeDefn) {
        // todo!()
    }

    fn visit_props_struct_ty(&mut self, _defn: PropsStructTypeNodeDefn) {
        // todo!()
    }

    fn visit_structure_ty(&mut self, _defn: StructureTypeNodeDefn) {
        // todo!()
    }

    fn visit_alias_ty(&mut self, _defn: ExternTypeNodeDefn) {
        // todo!()
    }

    fn visit_trai(&mut self, _defn: TraitNodeDefn) {
        //todo!()
    }

    fn visit_fugitive_node(&mut self, defn: FugitiveNodeDefn) {
        match defn {
            FugitiveNodeDefn::Fn(defn) => self.visit_fn_node(defn),
            FugitiveNodeDefn::Val(defn) => self.visit_val_node(defn),
            FugitiveNodeDefn::Gn(defn) => self.visit_gn_node(defn),
        }
    }

    fn visit_fn_node(&mut self, node_defn: FnNodeDefn) {}

    fn visit_val_node(&mut self, node_defn: ValNodeDefn) {}

    fn visit_gn_node(&mut self, node_defn: GnNodeDefn) {
        let node_decl = node_defn.node_decl(self.db);
        // todo!()
    }

    fn visit_value(&mut self, node_defn: ValNodeDefn) {
        let node_decl = node_defn.node_decl(self.db);
        // todo!()
    }

    fn visit_associated_item(&mut self, node_defn: AssociatedItemNodeDefn) {
        match node_defn {
            AssociatedItemNodeDefn::TypeItem(node_defn) => self.visit_ty_item_node(node_defn),
            AssociatedItemNodeDefn::TraitItem(node_defn) => self.visit_trai_item_node(node_defn),
            AssociatedItemNodeDefn::TraitForTypeItem(node_defn) => {
                self.visit_trai_for_ty_item_node(node_defn)
            }
        }
    }

    fn visit_ty_item_node(&self, node_defn: TypeItemNodeDefn) {
        // todo!()
    }

    fn visit_trai_item_node(&self, node_defn: TraitItemNodeDefn) {
        // todo!()
    }

    fn visit_trai_for_ty_item_node(&self, node_defn: TraitForTypeItemNodeDefn) {
        // todo!()
    }
}

struct InferContext<'a> {
    db: &'a dyn TokenInfoDb,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    expr_region_data: &'a ExprRegionData,
    expr_ty_region: &'a ExprTypeRegion,
    sheet: &'a mut TokenInfoSheet,
    expr_region: ExprRegionLeash,
}

impl<'a> InferContext<'a> {
    fn visit_all(mut self) {
        for (expr_idx, expr) in self.expr_region_data.expr_arena().indexed_iter() {
            self.visit_expr(expr_idx, expr)
        }
        for entity_path_expr in self
            .expr_region_data
            .principal_entity_path_expr_arena()
            .iter()
        {
            self.visit_entity_path_expr(entity_path_expr)
        }
        for (current_symbol_idx, current_symbol) in self
            .expr_region_data
            .symbol_region()
            .current_symbol_indexed_iter()
        {
            self.visit_current_symbol(current_symbol_idx, current_symbol)
        }
    }

    fn visit_expr(&mut self, expr_idx: ExprIdx, expr: &SynExpr) {
        match expr {
            SynExpr::CurrentSymbol {
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
                ..
            }
            | SynExpr::FrameVarDecl {
                token_idx,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
                ..
            } => self.sheet.add(
                *token_idx,
                TokenInfo::CurrentSymbol {
                    current_symbol_idx: *current_symbol_idx,
                    current_symbol_kind: *current_symbol_kind,
                    expr_region: self.expr_region,
                },
            ),
            SynExpr::InheritedSymbol {
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
                ..
            } => self.sheet.add(
                *token_idx,
                TokenInfo::InheritedSymbol {
                    inherited_symbol_idx: *inherited_symbol_idx,
                    expr_region: self.expr_region,
                    inherited_symbol_kind: *inherited_symbol_kind,
                },
            ),
            SynExpr::SelfType(token_idx) => self.sheet.add(*token_idx, TokenInfo::SelfType),
            SynExpr::SelfValue(token_idx) => self.sheet.add(*token_idx, TokenInfo::SelfValue),
            SynExpr::Field { ident_token, .. } => {
                self.sheet.add(ident_token.token_idx(), TokenInfo::Field)
            }
            SynExpr::MethodApplicationOrCall { ident_token, .. } => {
                self.sheet.add(ident_token.token_idx(), TokenInfo::Method)
            }
            SynExpr::Literal(_, _)
            | SynExpr::PrincipalEntityPath { .. }
            | SynExpr::ScopeResolution { .. }
            | SynExpr::Binary { .. }
            | SynExpr::Prefix { .. }
            | SynExpr::Suffix { .. }
            | SynExpr::TemplateInstantiation { .. }
            | SynExpr::NewTuple { .. }
            | SynExpr::List { .. }
            | SynExpr::Bracketed { .. }
            | SynExpr::Err(_)
            | SynExpr::Block { .. }
            | SynExpr::Be { .. } => (),
            SynExpr::BoxColonList { .. } => (),
            SynExpr::FunctionApplicationOrCall { function, .. }
            | SynExpr::ExplicitApplication {
                function_expr_idx: function,
                ..
            } => match self.expr_region_data[*function] {
                SynExpr::List {
                    lbox_token_idx,
                    items: _,
                    rbox_token_idx,
                } => {
                    self.sheet.add(lbox_token_idx, TokenInfo::BoxPrefix);
                    self.sheet.add(rbox_token_idx, TokenInfo::BoxPrefix)
                }
                SynExpr::BoxColonList {
                    lbox_token_idx,
                    colon_token_idx,
                    rbox_token_idx,
                    ..
                } => {
                    self.sheet.add(lbox_token_idx, TokenInfo::BoxColon);
                    self.sheet.add(colon_token_idx, TokenInfo::BoxColon);
                    self.sheet.add(rbox_token_idx, TokenInfo::BoxColon)
                }
                _ => (),
            },
            SynExpr::IndexOrCompositionWithList {
                owner: _,
                lbox_token_idx: _,
                items: _indices,
                rbox_token_idx: _,
            } => {
                // ad hoc
                // this should always be some
                match self.expr_ty_region.expr_ty_info_variant(expr_idx) {
                    Some(Ok(disambiguation)) => match disambiguation {
                        ExprDisambiguation::IndexOrComposeWithList(disambiguation) => {
                            match disambiguation {
                                IndexOrComposeWithListExprDisambiguation::Index(_) => (),
                                IndexOrComposeWithListExprDisambiguation::ComposeWithList => {
                                    todo!()
                                }
                            }
                        }
                        _ => unreachable!(),
                    },
                    None | Some(Err(_)) => (),
                }
            }
            SynExpr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => {
                self.sheet
                    .add(*lpar_token_idx, TokenInfo::UnitLeftParenthesis);
                self.sheet
                    .add(*rpar_token_idx, TokenInfo::UnitRightParenthesis);
            }
            SynExpr::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => {
                self.sheet
                    .add(function_ident.token_idx(), TokenInfo::HtmlFunctionIdent);
                for argument in arguments.iter() {
                    match argument {
                        HtmlArgumentExpr::Expanded { property_ident, .. }
                        | HtmlArgumentExpr::Shortened { property_ident, .. } => self
                            .sheet
                            .add(property_ident.token_idx(), TokenInfo::HtmlPropertyIdent),
                    }
                }
            }
            SynExpr::FunctionCall { .. } => (),
            SynExpr::Ritchie { .. } => (),
        }
    }

    fn visit_entity_path_expr(&mut self, entity_path_expr: &PrincipalEntityPathExpr) {
        match entity_path_expr {
            PrincipalEntityPathExpr::Root {
                principal_entity_path,
                path_name_token,
                ..
            } => self.sheet.add(
                path_name_token.token_idx(),
                TokenInfo::Entity((*principal_entity_path).into()),
            ),
            PrincipalEntityPathExpr::Subentity {
                path: Ok(path),
                ident_token: Ok(ident_token),
                ..
            } => self
                .sheet
                .add(ident_token.token_idx(), TokenInfo::Entity((*path).into())),
            PrincipalEntityPathExpr::Subentity { .. } => (),
        }
    }

    fn visit_current_symbol(
        &mut self,
        current_symbol_idx: CurrentSymbolIdx,
        current_symbol: &CurrentSymbol,
    ) {
        let current_symbol_kind = current_symbol.kind();
        match current_symbol_kind {
            CurrentSymbolKind::LetVariable {
                pattern_symbol_idx: pattern_symbol,
            }
            | CurrentSymbolKind::ExplicitRegularParameter {
                pattern_symbol_idx: pattern_symbol,
            } => match self.expr_region_data[pattern_symbol] {
                PatternSymbol::Atom(pattern_expr_idx) => {
                    match self.expr_region_data[pattern_expr_idx] {
                        PatternExpr::Ident {
                            ident_token,
                            symbol_modifier_keyword_group: _,
                        } => self.sheet.add(
                            ident_token.token_idx(),
                            TokenInfo::CurrentSymbol {
                                current_symbol_idx,
                                expr_region: self.expr_region,
                                current_symbol_kind,
                            },
                        ),
                        _ => unreachable!(),
                    }
                }
            },
            CurrentSymbolKind::FrameVariable(_) => (),
            CurrentSymbolKind::ImplicitParameter {
                implicit_parameter_kind,
            } => match implicit_parameter_kind {
                CurrentImplicitParameterSymbolKind::Type { ident_token } => self.sheet.add(
                    ident_token.token_idx(),
                    TokenInfo::CurrentSymbol {
                        current_symbol_idx,
                        expr_region: self.expr_region,
                        current_symbol_kind,
                    },
                ),
                CurrentImplicitParameterSymbolKind::Lifetime { label_token } => self.sheet.add(
                    label_token.token_idx(),
                    TokenInfo::CurrentSymbol {
                        current_symbol_idx,
                        expr_region: self.expr_region,
                        current_symbol_kind,
                    },
                ),
                CurrentImplicitParameterSymbolKind::Constant { ident_token } => self.sheet.add(
                    ident_token.token_idx(),
                    TokenInfo::CurrentSymbol {
                        current_symbol_idx,
                        expr_region: self.expr_region,
                        current_symbol_kind,
                    },
                ),
            },
            CurrentSymbolKind::ExplicitVariadicParameter { ident_token } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfo::CurrentSymbol {
                    current_symbol_idx,
                    expr_region: self.expr_region,
                    current_symbol_kind,
                },
            ),
        }
    }
}
