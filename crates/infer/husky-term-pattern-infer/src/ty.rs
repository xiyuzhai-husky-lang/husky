use crate::*;

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn infer_ty(&self) -> TermPatternInferResult<TermPatternItd> {
        match self.expr().variant {
            RawExprVariant::Atom(ref atom) => self.infer_atom_ty_term_pattern(atom),
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.infer_opn_ty_term_pattern(opn_variant, opds),
        }
    }

    fn infer_atom_ty_term_pattern(
        &self,
        atom: &RawAtomExpr,
    ) -> TermPatternInferResult<TermPatternItd> {
        match atom {
            RawAtomExpr::Literal(literal) => self.infer_literal(literal),
            RawAtomExpr::Symbol(symbol) => match symbol.kind {
                SymbolKind::EntityPath(_) => todo!(),
                SymbolKind::LocalVariable { init_range } => todo!(),
                SymbolKind::FrameVariable { .. } => Ok(self.term_menu().i32().term().into()),
                SymbolKind::Unrecognized => {
                    self.err_original(OriginalTermPatternInferError::IdentUnrecognized {
                        ident: symbol.ident,
                    })?
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
    ) -> TermPatternInferResult<TermPatternItd> {
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

    fn infer_literal(&self, literal: &RawLiteralData) -> TermPatternInferResult<TermPatternItd> {
        let term_menu = self.term_menu();
        match literal {
            RawLiteralData::Unit => todo!(),
            RawLiteralData::Integer(_) => todo!(),
            RawLiteralData::I32(_) => todo!(),
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
}
