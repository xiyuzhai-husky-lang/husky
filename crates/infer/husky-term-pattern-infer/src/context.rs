use crate::*;
use husky_entity_path::EntityPathItd;
use husky_expr_syntax::{RawExpr, RawExprArena, RawExprIdx, RawExprVariant};
use husky_term::{Term, TermAtom, TermContext, TermItd, TermMenu, Ty};
use husky_term_pattern::TermPatternItd;
use husky_word::InternWord;

pub(crate) struct TermPatternInferContext<'a> {
    db: &'a dyn TermPatternInferQueryGroup,
    expr_arena: &'a RawExprArena,
    expr_idx: RawExprIdx,
    term_menu: &'a TermMenu,
}

impl<'a> InternWord for TermPatternInferContext<'a> {
    fn word_itr(&self) -> &husky_word::WordInterner {
        self.db.word_itr()
    }
}

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn new(
        db: &'a dyn TermPatternInferQueryGroup,
        expr_arena: &'a RawExprArena,
        expr_idx: RawExprIdx,
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
        let result = self.infer(sheet.term_itr_mut());
        sheet.insert_result(self.expr_idx, result)
    }

    pub(crate) fn infer(&self, term_itr: &mut TermPatternInterner) -> ExprTermPatternInferResult {
        match self.expr().variant {
            RawExprVariant::Atom(ref atom) => self.infer_atom_ty_term_pattern(atom, term_itr),
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.infer_opn_ty_term_pattern(opn_variant, opds),
        }
    }

    fn infer_atom_ty_term_pattern(
        &self,
        atom: &RawAtomExpr,
        term_itr: &mut TermPatternInterner,
    ) -> ExprTermPatternInferResult {
        match atom {
            RawAtomExpr::Literal(literal) => self.infer_literal(*literal, term_itr),
            RawAtomExpr::Symbol(symbol) => match symbol.kind {
                SymbolKind::EntityPath(_) => todo!(),
                SymbolKind::LocalVariable { init_range } => todo!(),
                SymbolKind::FrameVariable { .. } => ExprTermPatternInferResult {
                    const_expr: Ok(None),
                    ty: Ok(self.term_menu().i32().term().into()),
                },
                SymbolKind::Unrecognized => {
                    let error =
                        self.error_original(OriginalTermPatternInferError::IdentUnrecognized {
                            ident: symbol.ident,
                        });
                    ExprTermPatternInferResult {
                        const_expr: self.err_derived(
                            DerivedTermPatternInferError::TermPatternInferError(Box::new(
                                error.clone(),
                            )),
                        ),
                        ty: Err(error),
                    }
                }
                SymbolKind::ThisValue => todo!(),
                SymbolKind::ThisMethod => todo!(),
                SymbolKind::ThisField => todo!(),
            },
            RawAtomExpr::Uncertain => todo!(),
        }
    }

    fn infer_opn_ty_term_pattern(
        &self,
        opn_variant: &RawOpnVariant,
        opds: &RawExprRange,
    ) -> ExprTermPatternInferResult {
        match opn_variant {
            RawOpnVariant::Binary(_) => todo!(),
            RawOpnVariant::Prefix(_) => todo!(),
            RawOpnVariant::Suffix(_) => todo!(),
            RawOpnVariant::CurlBracketed => todo!(),
            RawOpnVariant::List(_) => todo!(),
            RawOpnVariant::Field(_) => todo!(),
            RawOpnVariant::Abstraction => todo!(),
        }
        todo!()
    }

    fn infer_literal(
        &self,
        literal: RawLiteralData,
        term_itr: &mut TermPatternInterner,
    ) -> ExprTermPatternInferResult {
        let term_menu = self.term_menu();
        match literal {
            RawLiteralData::Unit => todo!(),
            RawLiteralData::Integer(_) => {
                let term = term_itr.it_unresolved(UnresolvedTerm::IntegerLiteral(self.expr_idx()));
                let ty = term_itr.it_unresolved(UnresolvedTerm::IntegerType(term));
                ExprTermPatternInferResult {
                    const_expr: Ok(Some(ConstExprPattern {
                        term: term.into(),
                        opt_substitution_ctx: None,
                    })),
                    ty: Ok(ty.into()),
                }
            }
            RawLiteralData::I32(i) => ExprTermPatternInferResult {
                const_expr: Ok(Some(ConstExprPattern {
                    term: TermPatternItd::Resolved(i.into()),
                    opt_substitution_ctx: None,
                })),
                ty: Ok(self.term_menu.i32().term().into()),
            },
            RawLiteralData::I64(_) => todo!(),
            RawLiteralData::Float(_) => todo!(),
            RawLiteralData::F32(_) => todo!(),
            RawLiteralData::F64(_) => todo!(),
            RawLiteralData::Bits(_) => todo!(),
            RawLiteralData::B32(_) => todo!(),
            RawLiteralData::B64(_) => todo!(),
            RawLiteralData::Bool(_) => todo!(),
        }
    }

    fn subexpr_context(&self, subexpr: RawExprIdx) -> Self {
        Self {
            db: self.db,
            expr_arena: self.expr_arena,
            expr_idx: subexpr,
            term_menu: self.term_menu,
        }
    }

    fn subexprs(&self) -> Option<RawExprRange> {
        match self.expr().variant {
            RawExprVariant::Atom(_) => None,
            RawExprVariant::Opn { ref opds, .. } => Some(opds.clone()),
        }
    }

    pub(crate) fn expr_idx(&self) -> RawExprIdx {
        self.expr_idx
    }

    pub(crate) fn expr(&self) -> &'a RawExpr {
        &self.expr_arena[self.expr_idx]
    }

    pub(crate) fn term_menu(&self) -> &'a TermMenu {
        self.term_menu
    }

    #[inline(always)]
    fn term_ctx(&self) -> TermContext<'a> {
        TermContext::new(self.db.upcast(), self.term_menu)
    }

    pub(crate) fn entity_path_term(&self, path: EntityPathItd) -> TermPatternInferResult<TermItd> {
        self.map_original(self.term_ctx().entity_path_term(path))
    }
}
