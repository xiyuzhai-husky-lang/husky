use crate::*;
use husky_entity_path::EntityPath;
use husky_expr::{Expr, ExprArena, ExprIdx, ExprVariant};
use husky_term::{Term, TermContext, TermMenu};
use husky_term_pattern::TermPatternItd;
use husky_word::WordDb;

pub(crate) struct TermPatternInferContext<'a> {
    db: &'a dyn TermPatternInferDb,
    expr_arena: &'a ExprArena,
    expr_idx: ExprIdx,
    term_menu: &'a TermMenu,
}

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn new(
        db: &'a dyn TermPatternInferDb,
        expr_arena: &'a ExprArena,
        expr_idx: ExprIdx,
        term_menu: &'a TermMenu,
    ) -> Self {
        Self {
            db,
            expr_arena,
            expr_idx,
            term_menu,
        }
    }

    pub(crate) fn write_inference(self, sheet: &mut TermPatternInferSheet) {
        if let Some(subexprs) = self.subexprs() {
            for subexpr in subexprs {
                self.subexpr_context(subexpr).write_inference(sheet)
            }
        }
        let result = self.infer(sheet);
        sheet.insert_result(self.expr_idx, result)
    }

    pub(crate) fn infer(
        &self,
        sheet: &mut TermPatternInferSheet,
    ) -> ExprTermPatternInferRawResults {
        match self.expr().variant {
            ExprVariant::Atom(ref atom) => self.infer_atom(atom, sheet),
            ExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.infer_opn_ty(opn_variant, opds, sheet),
        }
    }

    fn infer_atom(
        &self,
        atom: &AtomExpr,
        sheet: &mut TermPatternInferSheet,
    ) -> ExprTermPatternInferRawResults {
        match atom {
            AtomExpr::Literal(literal) => self.infer_literal(literal, sheet),
            AtomExpr::Symbol(symbol) => match symbol.kind {
                SymbolKind::EntityPath(_) => todo!(),
                SymbolKind::LocalVariable { init_range } => ExprTermPatternInferRawResults {
                    const_expr: Ok(None),
                    ty: self.var_ty_result(sheet, symbol.ident, init_range),
                },
                SymbolKind::FrameVariable { .. } => ExprTermPatternInferRawResults {
                    const_expr: Ok(None),
                    ty: Ok(self.term_menu().i32().into()),
                },
                SymbolKind::ThisValue => todo!(),
                SymbolKind::ThisMethod => todo!(),
                SymbolKind::ThisField => todo!(),
            },
            AtomExpr::Unrecognized(ident) => {
                let error = self.error_original(OriginalTermPatternInferError::IdentUnrecognized {
                    ident: self.db.dt_ident(*ident).to_owned(),
                });
                ExprTermPatternInferRawResults {
                    const_expr: Err(error.clone()),
                    ty: self
                        .err_derived(DerivedTermPatternInferError::TermPatternInferError(error)),
                }
            }
            AtomExpr::Uncertain(_) => todo!(),
        }
    }

    fn infer_opn_ty(
        &self,
        opn_variant: &RawOpnVariant,
        opds: &ExprRange,
        sheet: &mut TermPatternInferSheet,
    ) -> ExprTermPatternInferRawResults {
        match opn_variant {
            RawOpnVariant::Binary(opr) => self.infer_binary_opn(*opr, opds, sheet),
            RawOpnVariant::Prefix(_) => todo!(),
            RawOpnVariant::Suffix(_) => todo!(),
            RawOpnVariant::CurlBracketed => todo!(),
            RawOpnVariant::List(_) => todo!(),
            RawOpnVariant::Field(_) => todo!(),
            RawOpnVariant::Abstraction => todo!(),
        }
    }

    fn infer_binary_opn(
        &self,
        opr: BinaryOpr,
        opds: &ExprRange,
        sheet: &mut TermPatternInferSheet,
    ) -> ExprTermPatternInferRawResults {
        match opr {
            BinaryOpr::Assign(_) => todo!(),
            BinaryOpr::PureClosed(_) => ExprTermPatternInferRawResults {
                // todo: if both operands are constant, make this constexpr?
                const_expr: Ok(None),
                ty: self.expr_ty_result(sheet, opds.start()),
            },
            BinaryOpr::Comparison(_) => todo!(),
            BinaryOpr::ShortcuitLogic(_) => todo!(),
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => todo!(),
            BinaryOpr::As => todo!(),
        }
    }

    fn infer_literal(
        &self,
        literal: &RawLiteralData,
        sheet: &mut TermPatternInferSheet,
    ) -> ExprTermPatternInferRawResults {
        let _term_menu = self.term_menu();
        match literal {
            RawLiteralData::Unit => todo!(),
            RawLiteralData::Integer(_) => {
                let term = sheet.it_unresolved(UnresolvedTerm::IntegerLiteral(self.expr_idx()));
                let ty = sheet.it_unresolved(UnresolvedTerm::IntegerType(term));
                ExprTermPatternInferRawResults {
                    const_expr: Ok(Some(ConstExprPattern {
                        term: term.into(),
                        opt_substitution_ctx: None,
                    })),
                    ty: Ok(ty.into()),
                }
            }
            RawLiteralData::I32(i) => ExprTermPatternInferRawResults {
                const_expr: Ok(Some(ConstExprPattern {
                    term: TermPatternItd::Resolved(todo!()),
                    opt_substitution_ctx: None,
                })),
                ty: Ok(self.term_menu.i32().into()),
            },
            RawLiteralData::I64(i) => ExprTermPatternInferRawResults {
                const_expr: Ok(Some(ConstExprPattern {
                    term: TermPatternItd::Resolved(todo!()),
                    opt_substitution_ctx: None,
                })),
                ty: Ok(self.term_menu.i32().into()),
            },
            RawLiteralData::Float(_) => {
                let term = sheet.it_unresolved(UnresolvedTerm::FloatLiteral(self.expr_idx()));
                let ty = sheet.it_unresolved(UnresolvedTerm::FloatType(term));
                ExprTermPatternInferRawResults {
                    const_expr: Ok(Some(ConstExprPattern {
                        term: term.into(),
                        opt_substitution_ctx: None,
                    })),
                    ty: Ok(ty.into()),
                }
            }
            RawLiteralData::F32(_) => todo!(),
            RawLiteralData::F64(_) => todo!(),
            RawLiteralData::Bits(_) => todo!(),
            RawLiteralData::B32(_) => todo!(),
            RawLiteralData::B64(_) => todo!(),
            RawLiteralData::Bool(_) => todo!(),
        }
    }

    fn subexpr_context(&self, subexpr: ExprIdx) -> Self {
        Self {
            db: self.db,
            expr_arena: self.expr_arena,
            expr_idx: subexpr,
            term_menu: self.term_menu,
        }
    }

    fn subexprs(&self) -> Option<ExprRange> {
        match self.expr().variant {
            ExprVariant::Atom(_) => None,
            ExprVariant::Opn { ref opds, .. } => Some(opds.clone()),
        }
    }

    pub(crate) fn expr_idx(&self) -> ExprIdx {
        self.expr_idx
    }

    pub(crate) fn expr(&self) -> &'a Expr {
        &self.expr_arena[self.expr_idx]
    }

    pub(crate) fn term_menu(&self) -> &'a TermMenu {
        self.term_menu
    }

    #[inline(always)]
    fn term_ctx(&self) -> TermContext<'a> {
        TermContext::new(self.db.upcast(), self.term_menu)
    }

    pub(crate) fn entity_path_term(&self, path: EntityPath) -> TermPatternInferResult<Term> {
        self.map_original(self.term_ctx().entity_path_term(path))
    }
}
