use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_defn::*;
use husky_entity_path::EntityPath;
use husky_expr::*;

pub(crate) struct InferEngine<'a> {
    db: &'a dyn TokenInfoDb,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    defn_sheet: &'a DefnSheet,
    module_symbol_context: ModuleSymbolContext<'a>,
    sheet: TokenInfoSheet,
}

impl<'a> InferEngine<'a> {
    pub(crate) fn new(db: &'a dyn TokenInfoDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let token_sheet_data = &db.token_sheet_data(module_path)?;
        Ok(Self {
            db,
            token_sheet_data,
            defn_sheet: db.defn_sheet(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            sheet: TokenInfoSheet::new(token_sheet_data),
            module_symbol_context: db.module_symbol_context(module_path)?,
        })
    }

    pub(crate) fn visit_all(mut self) -> TokenInfoSheet {
        for defn in self.defn_sheet.defns() {
            let decl = defn.decl(self.db);
            self.visit_expr_sheet(decl.expr_sheet(self.db).into());
            defn.expr_sheet(self.db)
                .map(|expr_sheet| self.visit_expr_sheet(expr_sheet.into()));
            let ast_idx = defn.ast_idx(self.db);
            match self.ast_sheet[ast_idx] {
                Ast::Defn {
                    token_group_idx,
                    ref body,
                    accessibility,
                    entity_kind,
                    entity_path,
                    ident_token,
                    is_generic,
                    body_kind,
                    saved_stream_state,
                } => {
                    self.sheet
                        .add(ident_token.token_idx(), TokenInfo::Entity(entity_kind));
                    if is_generic {
                        for implicit_parameter in defn.implicit_parameters(self.db) {
                            self.sheet.add(
                                implicit_parameter.ident().token_idx(),
                                TokenInfo::ImplicitParameter,
                            )
                        }
                    }
                }
                _ => unreachable!(),
            }
            match defn {
                Defn::Type(defn) => self.visit_ty(defn),
                Defn::Trait(defn) => self.visit_trai(defn),
                Defn::Form(defn) => self.visit_form(defn),
                Defn::TypeItem(_) => todo!(),
                Defn::TraitItem(_) => todo!(),
                Defn::Variant(_) => todo!(),
            }
        }
        self.sheet
    }

    fn visit_expr_sheet(&mut self, expr_sheet: ExprSheet) {
        AuxInferEngine {
            db: self.db,
            token_sheet_data: self.token_sheet_data,
            ast_sheet: self.ast_sheet,
            sheet: &mut self.sheet,
            symbol_context: ExprContext::new(self.db, self.module_symbol_context, expr_sheet),
            expr_sheet,
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
            TypeDefn::PropsStruct(defn) => self.visit_props_struct_ty(defn),
            TypeDefn::Structure(defn) => self.visit_structure_ty(defn),
            TypeDefn::Foreign(defn) => self.visit_alias_ty(defn),
        }
    }

    fn visit_enum_ty(&mut self, defn: EnumTypeDefn) {
        // todo!()
    }

    fn visit_inductive_ty(&mut self, defn: InductiveTypeDefn) {
        // todo!()
    }

    fn visit_record_ty(&mut self, defn: RecordTypeDefn) {
        // todo!()
    }

    fn visit_unit_struct_ty(&mut self, defn: UnitStructTypeDefn) {
        // todo!()
    }

    fn visit_tuple_struct_ty(&mut self, defn: TupleStructTypeDefn) {
        // todo!()
    }

    fn visit_props_struct_ty(&mut self, defn: PropsStructTypeDefn) {
        // todo!()
    }

    fn visit_structure_ty(&mut self, defn: StructureTypeDefn) {
        // todo!()
    }

    fn visit_alias_ty(&mut self, defn: AlienTypeDefn) {
        // todo!()
    }

    fn visit_trai(&mut self, defn: TraitDefn) {
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

    fn visit_function(&mut self, defn: FunctionDefn) {}

    fn visit_feature(&mut self, defn: FeatureDefn) {}

    fn visit_morphism(&mut self, defn: MorphismDefn) {
        let decl = defn.decl(self.db);
        // todo!()
    }

    fn visit_value(&mut self, defn: ValueDefn) {
        let decl = defn.decl(self.db);
        // todo!()
    }
}

struct AuxInferEngine<'a> {
    db: &'a dyn TokenInfoDb,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    symbol_context: ExprContext<'a>,
    sheet: &'a mut TokenInfoSheet,
    expr_sheet: ExprSheet,
}

impl<'a> AuxInferEngine<'a> {
    fn visit_all(mut self) {
        for expr in self.symbol_context.exprs() {
            self.visit_expr(expr)
        }
        for (local_symbol_idx, local_symbol) in self.symbol_context.indexed_local_symbol_iter() {
            self.visit_local_symbol(local_symbol_idx, local_symbol)
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::LocalSymbol {
                token_idx,
                local_symbol_idx,
                local_symbol_kind,
                ..
            } => self.sheet.add(
                *token_idx,
                TokenInfo::LocalSymbol {
                    local_symbol_idx: *local_symbol_idx,
                    expr_sheet: self.expr_sheet,
                    local_symbol_kind: *local_symbol_kind,
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
                    expr_sheet: self.expr_sheet,
                    inherited_symbol_kind: *inherited_symbol_kind,
                },
            ),
            Expr::Field { ident_token, .. } => {
                self.sheet.add(ident_token.token_idx(), TokenInfo::Field)
            }
            Expr::MethodCall { ident_token, .. } => {
                self.sheet.add(ident_token.token_idx(), TokenInfo::Method)
            }
            Expr::Literal(_)
            | Expr::EntityPath { .. }
            | Expr::Uncertain(_)
            | Expr::Unrecognized(_)
            | Expr::BinaryOpn { .. }
            | Expr::PrefixOpn { .. }
            | Expr::SuffixOpn { .. }
            | Expr::TemplateInstantiation { .. }
            | Expr::NewTuple { .. }
            | Expr::NewBoxList { .. }
            | Expr::Bracketed(_)
            | Expr::Err(_)
            | Expr::Block { .. }
            | Expr::FunctionCall { .. }
            | Expr::Be { .. } => (),
            Expr::BoxColon { .. } => (),
            Expr::Application { function, .. } => match self.symbol_context[*function] {
                Expr::NewBoxList {
                    caller: None,
                    lbox_token_idx,
                    items,
                    rbox_token_idx,
                } => {
                    self.sheet.add(lbox_token_idx, TokenInfo::BoxPrefix);
                    self.sheet.add(rbox_token_idx, TokenInfo::BoxPrefix)
                }
                Expr::BoxColon {
                    caller: None,
                    lbox_token_idx,
                    colon_token_idx,
                    rbox_token,
                } => {
                    self.sheet.add(lbox_token_idx, TokenInfo::BoxColon);
                    self.sheet.add(colon_token_idx, TokenInfo::BoxColon);
                    self.sheet.add(rbox_token.token_idx(), TokenInfo::BoxColon)
                }
                _ => (),
            },
        }
    }

    fn visit_local_symbol(&mut self, local_symbol_idx: LocalSymbolIdx, local_symbol: &LocalSymbol) {
        let local_symbol_kind = local_symbol.kind();
        match local_symbol_kind {
            LocalSymbolKind::LetVariable { pattern_symbol }
            | LocalSymbolKind::Parameter { pattern_symbol } => {
                match self.symbol_context[pattern_symbol] {
                    PatternSymbol::Atom(pattern_expr_idx) => {
                        match self.symbol_context[pattern_expr_idx] {
                            PatternExpr::Identifier {
                                ident_token,
                                liason,
                            } => self.sheet.add(
                                ident_token.token_idx(),
                                TokenInfo::LocalSymbol {
                                    local_symbol_idx,
                                    expr_sheet: self.expr_sheet,
                                    local_symbol_kind,
                                },
                            ),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
    }

    // fn visit_pattern_expr(&mut self, pattern_expr_idx: PatternExprIdx, pattern_expr: &PatternExpr) {
    //     match pattern_expr {
    //         PatternExpr::Literal(_) => todo!(),
    //         PatternExpr::Identifier { ident_token, .. } => {
    //             let env = self.pattern_expr_sheet.pattern_info(pattern_expr_idx);
    //             let info = match env {
    //                 PatternInfo::Parameter => TokenInfo::Parameter,
    //                 PatternInfo::Let => TokenInfo::Variable {
    //                     // ad hoc
    //                     variable_idx: None,
    //                     expr_sheet: self.expr_sheet,
    //                 },
    //                 PatternInfo::Match => TokenInfo::Variable {
    //                     // ad hoc
    //                     variable_idx: None,
    //                     expr_sheet: self.expr_sheet,
    //                 },
    //                 PatternInfo::Be => TokenInfo::Variable {
    //                     // ad hoc
    //                     variable_idx: None,
    //                     expr_sheet: self.expr_sheet,
    //                 },
    //             };
    //             self.sheet.add(ident_token.token_idx(), info)
    //         }
    //         // TokenInfo::Parameter),
    //         // PatternExpr::Identifier { ident_token } => {
    //         //     self.sheet.add(ident_token.token_idx(), TokenInfo::Variable)
    //         // }
    //         PatternExpr::Entity(_) => todo!(),
    //         PatternExpr::Tuple { name, fields } => todo!(),
    //         PatternExpr::Struct { name, fields } => todo!(),
    //         PatternExpr::OneOf { options } => todo!(),
    //         PatternExpr::Binding {
    //             ident_token,
    //             asperand_token,
    //             src,
    //         } => todo!(),
    //         PatternExpr::Range {
    //             start,
    //             dot_dot_token,
    //             end,
    //         } => todo!(),
    //     }
    // }
}
