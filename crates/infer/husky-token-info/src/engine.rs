use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_defn::*;
use husky_expr::*;

pub(crate) struct TokenInfoInferEngine<'a> {
    db: &'a dyn TokenInfoDb,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    defn_sheet: &'a DefnSheet,
    sheet: TokenInfoSheet,
}

impl<'a> TokenInfoInferEngine<'a> {
    pub(crate) fn new(db: &'a dyn TokenInfoDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let token_sheet = &db.token_sheet(module_path)?;
        Ok(Self {
            db,
            token_sheet,
            defn_sheet: db.defn_sheet(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            sheet: TokenInfoSheet::new(token_sheet),
        })
    }

    pub(crate) fn visit_all(mut self) -> TokenInfoSheet {
        for defn in self.defn_sheet.defns() {
            let decl = defn.decl(self.db);
            self.visit_expr_sheet(decl.expr_sheet(self.db));
            self.visit_expr_sheet(defn.expr_sheet(self.db));
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
        ExprSheetTokenInfoInferEngine {
            db: self.db,
            token_sheet: self.token_sheet,
            ast_sheet: self.ast_sheet,
            sheet: &mut self.sheet,
            expr_arena: expr_sheet.expr_arena(self.db),
            pattern_expr_arena: expr_sheet.pattern_expr_arena(self.db),
            entity_path_expr_arena: expr_sheet.entity_path_expr_arena(self.db),
            stmt_arena: expr_sheet.stmt_arena(self.db),
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

    fn visit_function(&mut self, defn: FunctionDefn) {
        // todo!()
    }

    fn visit_feature(&mut self, defn: FeatureDefn) {
        let decl = defn.decl(self.db);
        // todo!()
    }

    fn visit_morphism(&mut self, defn: MorphismDefn) {
        let decl = defn.decl(self.db);
        // todo!()
    }

    fn visit_value(&mut self, defn: ValueDefn) {
        let decl = defn.decl(self.db);
        // todo!()
    }
}

struct ExprSheetTokenInfoInferEngine<'a> {
    db: &'a dyn TokenInfoDb,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    expr_arena: &'a ExprArena,
    pattern_expr_arena: &'a PatternExprArena,
    entity_path_expr_arena: &'a EntityPathExprArena,
    stmt_arena: &'a StmtArena,
    sheet: &'a mut TokenInfoSheet,
}

impl<'a> ExprSheetTokenInfoInferEngine<'a> {
    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath(_) => todo!(),
            Expr::Variable {
                token_idx,
                variable_idx,
            } => todo!(),
            Expr::Uncertain(_) => todo!(),
            Expr::Unrecognized(_) => (),
            Expr::Field { ident_token, .. } => todo!(),
            Expr::MethodCall { ident_token, .. } => todo!(),
            Expr::BinaryOpn { .. }
            | Expr::PrefixOpn { .. }
            | Expr::SuffixOpn { .. }
            | Expr::TemplateInstantiation { .. }
            | Expr::Application { .. }
            | Expr::NewTuple { .. }
            | Expr::NewList { .. }
            | Expr::Bracketed(_)
            | Expr::Err(_) => (),
            Expr::Block { stmts } => {
                for stmt in &self.stmt_arena[stmts] {
                    self.visit_stmt(stmt)
                }
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let {} => todo!(),
        }
    }

    fn visit_pattern_expr(&mut self, pattern_expr: &PatternExpr) {
        match pattern_expr {
            PatternExpr::Literal(_) => todo!(),
            PatternExpr::ParameterIdentifier { ident_token } => self
                .sheet
                .add(ident_token.token_idx(), TokenInfo::Parameter),
            PatternExpr::Entity(_) => todo!(),
            PatternExpr::Tuple { name, fields } => todo!(),
            PatternExpr::Struct { name, fields } => todo!(),
            PatternExpr::OneOf { options } => todo!(),
            PatternExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            PatternExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    fn visit_all(mut self) {
        for expr in self.expr_arena.data() {
            self.visit_expr(expr)
        }
        for pattern_expr in self.pattern_expr_arena.data() {
            self.visit_pattern_expr(pattern_expr)
        }
    }
}
