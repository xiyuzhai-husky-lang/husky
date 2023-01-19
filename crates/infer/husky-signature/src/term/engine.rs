use super::*;
use husky_expr::*;
use husky_opn_syntax::PrefixOpr;
use husky_print_utils::p;
use outcome::{Success, *};

pub(crate) struct SignatureTermEngine<'a> {
    db: &'a dyn SignatureDb,
    expr_arena: &'a ExprArena,
    entity_path_expr_arena: &'a EntityPathExprArena,
    term_menu: &'a TermMenu,
    symbol_page: &'a SymbolPage,
    expr_terms: ExprMap<SignatureTermOutcome<Term>>,
    term_symbol_page: TermSymbolPage,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TermSymbolPage {
    registry: TermSymbolRegistry,
    inherited_symbol_terms: Vec<Option<TermSymbol>>,
    current_symbol_terms: Vec<TermSymbol>,
}

impl TermSymbolPage {
    fn new(parent: Option<&TermSymbolPage>, symbol_page: &SymbolPage) -> TermSymbolPage {
        let mut registry = parent.map_or(Default::default(), |parent| parent.registry.clone());
        let inherited_symbol_terms = symbol_page
            .indexed_inherited_symbol_iter()
            .map(|(idx, symbol)| todo!())
            .collect();
        let current_symbol_terms = symbol_page
            .indexed_current_symbol_iter()
            .map(|(idx, symbol)| match symbol.variant() {
                CurrentSymbolVariant::ImplicitParameter {
                    implicit_parameter_variant,
                } => match implicit_parameter_variant {
                    ImplicitParameterVariant::Type { .. } => registry.new_ty0().into(),
                },
                CurrentSymbolVariant::Parameter { .. } => registry.new_parameter().into(),
                CurrentSymbolVariant::LetVariable { pattern_symbol } => todo!(),
                CurrentSymbolVariant::FrameVariable(_) => todo!(),
            })
            .collect();
        TermSymbolPage {
            registry,
            inherited_symbol_terms,
            current_symbol_terms,
        }
    }

    fn current_symbol_term(&self, current_symbol_idx: CurrentSymbolIdx) -> TermSymbol {
        self.current_symbol_terms[current_symbol_idx.raw()]
    }
}

impl<'a> SignatureTermEngine<'a> {
    pub(crate) fn new(
        db: &'a dyn SignatureDb,
        expr_page: ExprPage,
        parent_term_symbol_page: Option<&'a TermSymbolPage>,
    ) -> Self {
        let expr_arena = &expr_page.expr_arena(db);
        let toolchain = expr_page.toolchain(db);
        let symbol_page = expr_page.symbol_page(db);
        // ad hoc
        let term_menu = &db.term_menu(toolchain).as_ref().unwrap();
        Self {
            db,
            expr_arena,
            entity_path_expr_arena: expr_page.entity_path_expr_arena(db),
            symbol_page,
            term_menu,
            expr_terms: ExprMap::new(expr_arena),
            term_symbol_page: TermSymbolPage::new(parent_term_symbol_page, symbol_page),
        }
    }

    // ask about the term for expr, assuming it hasn't been computed before
    pub(crate) fn query_new(&mut self, expr_idx: ExprIdx) -> SignatureTermOutcome<Term> {
        let outcome = self.calc(expr_idx);
        let term = match outcome {
            Success(term) => Success(term),
            Failure(_) => todo!(),
            Abort(_) => Abort(SignatureTermAbortion::TermAbortion),
        };
        self.save(expr_idx, outcome);
        term
    }

    pub(crate) fn finish(self) -> SignatureTermSheet {
        SignatureTermSheet::new(self.term_symbol_page)
    }

    fn save(&mut self, expr_idx: ExprIdx, outcome: SignatureTermOutcome<Term>) {
        self.expr_terms.insert_new(expr_idx, outcome)
    }

    fn calc(&mut self, expr_idx: ExprIdx) -> SignatureTermOutcome<Term> {
        match self.expr_arena[expr_idx] {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => Success(Term::Entity(entity_path)),
                None => Abort(SignatureTermAbortion::InvalidEntityPath),
            },
            Expr::InheritedSymbol {
                ident,
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            Expr::CurrentSymbol {
                current_symbol_idx, ..
            } => Success(
                self.term_symbol_page
                    .current_symbol_term(current_symbol_idx)
                    .into(),
            ),
            Expr::FrameVarDecl {
                token_idx,
                ident,
                current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => todo!(),
            Expr::BinaryOpn {
                lopd,
                opr,
                opr_token_idx,
                ropd,
            } => todo!(),
            Expr::Be { .. } => todo!(),
            Expr::PrefixOpn {
                opr,
                opr_token_idx,
                opd,
            } => {
                let Success(opd) = self.query_new(opd) else {
                    return Abort(SignatureTermAbortion::CannotInferOperandTermInPrefix);
                };
                let tmpl = match opr {
                    PrefixOpr::Minus => todo!(),
                    PrefixOpr::Not => todo!(),
                    PrefixOpr::BitNot => todo!(),
                    PrefixOpr::Ref => todo!(),
                    PrefixOpr::Vector => todo!(),
                    PrefixOpr::Slice => todo!(),
                    PrefixOpr::CyclicSlice => todo!(),
                    PrefixOpr::Array(_) => todo!(),
                    PrefixOpr::Option => self.term_menu.option_ty(),
                };
                Success(TermApplication::new(self.db, tmpl, opd).into())
            }
            Expr::SuffixOpn {
                opd,
                punctuation,
                punctuation_token_idx,
            } => todo!(),
            Expr::FunctionCall { .. } => todo!(),
            Expr::Field {
                this_expr,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodCall { .. } => todo!(),
            Expr::TemplateInstantiation { .. } => todo!(),
            Expr::Application { function, argument } => {
                let Success(argument) = self.query_new(argument) else {
                        return Abort(SignatureTermAbortion::CannotInferArgumentTermInApplication)
                    };
                match self.expr_arena[function] {
                    Expr::BoxColon {
                        caller: None,
                        lbox_token_idx,
                        colon_token_idx,
                        rbox_token,
                    } => Success({
                        let db = self.db;
                        let function = self.term_menu.slice_ty();
                        TermApplication::new(db, function, argument).into()
                    }),
                    Expr::NewBoxList {
                        caller: None,
                        lbox_token_idx,
                        items,
                        rbox_token_idx,
                    } => {
                        todo!()
                    }
                    _ => {
                        let Success(function) = self.query_new(function) else {
                            return Abort(SignatureTermAbortion::CannotInferFunctionTermInApplication)
                        };
                        todo!()
                    }
                }
            }
            Expr::NewTuple {
                lpar_token_idx,
                items,
                rpar_token_idx,
            } => todo!(),
            Expr::NewBoxList {
                caller,
                lbox_token_idx,
                items,
                rbox_token_idx,
            } => todo!(),
            Expr::BoxColon {
                caller,
                lbox_token_idx,
                colon_token_idx,
                rbox_token,
            } => todo!(),
            Expr::Bracketed(_) => todo!(),
            Expr::Block { stmts } => todo!(),
            Expr::Err(_) => Abort(SignatureTermAbortion::ExprError),
        }
    }
}
