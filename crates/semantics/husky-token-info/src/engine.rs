use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_defn::*;

use husky_entity_tree::ParentUseExpr;
use husky_expr::*;
use husky_expr_ty::{ExprDisambiguation, ExprTypeRegion};

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
        self.visit_decl_and_defns()?;
        self.visit_use_expr_rules();
        Ok(self.sheet)
    }

    fn visit_decl_and_defns(&mut self) -> EntityTreeResult<()> {
        for (_, defn) in self.db.collect_defns(self.module_path)?.defns() {
            if let Ok(defn) = defn {
                self.visit_decl_and_defn(defn);
            }
        }
        Ok(())
    }

    fn visit_use_expr_rules(&mut self) {
        for (rule_idx, rule) in self.entity_tree_sheet.use_expr_rule_indexed_iter() {
            self.visit_use_expr_rule(rule, rule_idx);
        }
    }

    fn visit_use_expr_rule(&mut self, rule: &UseExprRule, rule_idx: UseExprRuleIdx) {
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
            UseExpr::SelfOne {
                self_value_token: _,
            } => todo!(),
        }
    }

    fn visit_decl_and_defn(&mut self, defn: Defn) {
        let decl = defn.decl(self.db);
        self.visit_expr_region(decl.expr_region(self.db).into());
        defn.expr_region(self.db)
            .map(|expr_region| self.visit_expr_region(expr_region.into()));
        let ast_idx = defn.ast_idx(self.db);
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                ident_token,
                entity_kind,
                ..
            } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfo::Entity(decl.path(self.db), Some(entity_kind)),
            ),
            Ast::ImplBlock { .. } => (),
            _ => unreachable!(),
        }
        match defn {
            Defn::Type(defn) => self.visit_ty(defn),
            Defn::Trait(defn) => self.visit_trai(defn),
            Defn::Form(defn) => self.visit_form(defn),
            Defn::AssociatedItem(defn) => self.visit_associated_item(defn),
            Defn::Variant(_) => todo!(),
            Defn::Impl(_) => (),
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

    fn visit_ty(&mut self, defn: TypeDefn) {
        match defn {
            TypeDefn::Enum(defn) => self.visit_enum_ty(defn),
            TypeDefn::Inductive(defn) => self.visit_inductive_ty(defn),
            TypeDefn::Record(defn) => self.visit_record_ty(defn),
            TypeDefn::UnitStruct(defn) => self.visit_unit_struct_ty(defn),
            TypeDefn::TupleStruct(defn) => self.visit_tuple_struct_ty(defn),
            TypeDefn::RegularStruct(defn) => self.visit_props_struct_ty(defn),
            TypeDefn::Structure(defn) => self.visit_structure_ty(defn),
            TypeDefn::Extern(defn) => self.visit_alias_ty(defn),
            TypeDefn::Union(_) => todo!(),
        }
    }

    fn visit_enum_ty(&mut self, _defn: EnumTypeDefn) {
        // todo!()
    }

    fn visit_inductive_ty(&mut self, _defn: InductiveTypeDefn) {
        // todo!()
    }

    fn visit_record_ty(&mut self, _defn: RecordTypeDefn) {
        // todo!()
    }

    fn visit_unit_struct_ty(&mut self, _defn: UnitStructTypeDefn) {
        // todo!()
    }

    fn visit_tuple_struct_ty(&mut self, _defn: TupleStructTypeDefn) {
        // todo!()
    }

    fn visit_props_struct_ty(&mut self, _defn: RegularStructTypeDefn) {
        // todo!()
    }

    fn visit_structure_ty(&mut self, _defn: StructureTypeDefn) {
        // todo!()
    }

    fn visit_alias_ty(&mut self, _defn: ExternTypeDefn) {
        // todo!()
    }

    fn visit_trai(&mut self, _defn: TraitDefn) {
        //todo!()
    }

    fn visit_form(&mut self, defn: FormDefn) {
        match defn {
            FormDefn::Function(defn) => self.visit_function(defn),
            FormDefn::Feature(defn) => self.visit_feature(defn),
            FormDefn::Morphism(defn) => self.visit_morphism(defn),
            FormDefn::Value(defn) => self.visit_value(defn),
        }
    }

    fn visit_function(&mut self, _defn: FunctionDefn) {}

    fn visit_feature(&mut self, _defn: FeatureDefn) {}

    fn visit_morphism(&mut self, defn: MorphismDefn) {
        let _decl = defn.decl(self.db);
        // todo!()
    }

    fn visit_value(&mut self, defn: ValueDefn) {
        let _decl = defn.decl(self.db);
        // todo!()
    }

    fn visit_associated_item(&mut self, defn: AssociatedItemDefn) {
        match defn {
            AssociatedItemDefn::TypeItem(defn) => self.visit_ty_item(defn),
            AssociatedItemDefn::TraitItem(defn) => self.visit_trai_item(defn),
            AssociatedItemDefn::TraitForTypeItem(defn) => self.visit_trai_for_ty_item(defn),
        }
    }

    fn visit_ty_item(&self, _defn: TypeItemDefn) {
        // todo!()
    }

    fn visit_trai_item(&self, _defn: TraitItemDefn) {
        // todo!()
    }

    fn visit_trai_for_ty_item(&self, _defn: TraitForTypeItemDefn) {
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
        for entity_path_expr in self.expr_region_data.entity_path_expr_arena().iter() {
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

    fn visit_expr(&mut self, expr_idx: ExprIdx, expr: &Expr) {
        match expr {
            Expr::CurrentSymbol {
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
                ..
            }
            | Expr::FrameVarDecl {
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
            Expr::InheritedSymbol {
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
            Expr::SelfType(token_idx) => self.sheet.add(*token_idx, TokenInfo::SelfType),
            Expr::SelfValue(token_idx) => self.sheet.add(*token_idx, TokenInfo::SelfValue),
            Expr::Field { ident_token, .. } => {
                self.sheet.add(ident_token.token_idx(), TokenInfo::Field)
            }
            Expr::MethodCall { ident_token, .. } => {
                self.sheet.add(ident_token.token_idx(), TokenInfo::Method)
            }
            Expr::Literal(_)
            | Expr::EntityPath { .. }
            | Expr::Binary { .. }
            | Expr::Prefix { .. }
            | Expr::Suffix { .. }
            | Expr::TemplateInstantiation { .. }
            | Expr::NewTuple { .. }
            | Expr::List { .. }
            | Expr::Bracketed { .. }
            | Expr::Err(_)
            | Expr::Block { .. }
            | Expr::Be { .. } => (),
            Expr::BoxColonList { .. } => (),
            Expr::ExplicitApplicationOrRitchieCall { function, .. }
            | Expr::ExplicitApplication { function, .. } => {
                match self.expr_region_data[*function] {
                    Expr::List {
                        lbox_token_idx,
                        items: _,
                        rbox_token_idx,
                    } => {
                        self.sheet.add(lbox_token_idx, TokenInfo::BoxPrefix);
                        self.sheet.add(rbox_token_idx, TokenInfo::BoxPrefix)
                    }
                    Expr::BoxColonList {
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
                }
            }
            Expr::IndexOrCompositionWithList {
                owner: _,
                lbox_token_idx: _,
                items: _indices,
                rbox_token_idx: _,
            } => {
                // ad hoc
                // this should always be some
                match self.expr_ty_region.expr_disambiguation(expr_idx) {
                    Some(Ok(disambiguation)) => match disambiguation {
                        ExprDisambiguation::IndexOrComposeWithList(_disambiguation) => todo!(),
                        _ => unreachable!(),
                    },
                    None | Some(Err(_)) => (),
                }
            }
            Expr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => todo!(),
        }
    }

    fn visit_entity_path_expr(&mut self, entity_path_expr: &EntityPathExpr) {
        match entity_path_expr {
            EntityPathExpr::Root {
                entity_path,
                token_idx,
                ..
            } => self
                .sheet
                .add(*token_idx, TokenInfo::Entity(Some(*entity_path), None)),
            EntityPathExpr::Subentity {
                path: Ok(entity_path),
                ident_token: Ok(ident_token),
                ..
            } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfo::Entity(Some(*entity_path), None),
            ),
            EntityPathExpr::Subentity { .. } => (),
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
            | CurrentSymbolKind::Parameter {
                pattern_symbol_idx: pattern_symbol,
            } => match self.expr_region_data[pattern_symbol] {
                PatternSymbol::Atom(pattern_expr_idx) => {
                    match self.expr_region_data[pattern_expr_idx] {
                        PatternExpr::Ident {
                            ident_token,
                            modifier_keyword_group: _,
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
            },
        }
    }
}
