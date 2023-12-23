use crate::*;
use husky_ethereal_term::EtherealTerm;
use husky_syn_expr::{SynPatternExpr, SynPatternExprIdx, SynPatternExprRoot};
use husky_token_data::{IntegerLikeLiteralData, LiteralData};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirEagerPatternExpr {
    /// example: `1`
    Literal(TermLiteral),
    /// example: `a`
    Ident {
        symbol_modifier: Option<SymbolModifier>,
        ident: Ident,
    },
    /// example: `A::B`
    Unit(PatternPath),
    /// example: `(a, b)`
    Tuple {
        path: Option<PatternPath>,
        fields: HirEagerPatternExprIdxRange,
    },
    /// example: `C { .. }`
    Props {
        path: Option<PatternPath>,
        // todo: change to punctuated
        fields: HirEagerPatternExprIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf {
        options: HirEagerPatternExprIdxRange,
    },
    /// example: `x @ 1..9`
    Binding {
        ident: Ident,
        /// example: `1..9`
        src: HirEagerPatternExprIdx,
    },
    /// example: `1..9`
    Range {
        start: HirEagerPatternExprIdx,
        end: HirEagerPatternExprIdx,
    },
}

pub type HirEagerPatternExprArena = Arena<HirEagerPatternExpr>;
pub type HirEagerPatternExprIdx = ArenaIdx<HirEagerPatternExpr>;
pub type HirEagerPatternExprIdxRange = ArenaIdxRange<HirEagerPatternExpr>;
pub type HirEagerPatternExprMap<V> = ArenaMap<HirEagerPatternExpr, V>;
pub type HirEagerPatternExprOrderedMap<V> = ArenaOrderedMap<HirEagerPatternExpr, V>;

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_pattern_expr(
        &mut self,
        syn_pattern_root: impl Into<SynPatternExprRoot>,
    ) -> HirEagerPatternExprIdx {
        let syn_pattern_expr_idx = syn_pattern_root.into().syn_pattern_expr_idx();
        let pattern_expr = self.new_pattern_expr_aux(syn_pattern_expr_idx);
        self.alloc_pattern_expr(pattern_expr, syn_pattern_expr_idx)
    }

    fn new_pattern_expr_aux(
        &mut self,
        syn_pattern_expr_idx: SynPatternExprIdx,
    ) -> HirEagerPatternExpr {
        let db = self.db();
        match self.syn_expr_region_data()[syn_pattern_expr_idx] {
            SynPatternExpr::Literal { literal, .. } => {
                HirEagerPatternExpr::Literal(match literal {
                    LiteralData::Unit => TermLiteral::Unit(()),
                    LiteralData::Char(_) => todo!(),
                    LiteralData::String(_) => todo!(),
                    LiteralData::Integer(literal) => match literal {
                        IntegerLikeLiteralData::UnspecifiedRegular(value) => {
                            let EtherealTerm::EntityPath(TermEntityPath::TypeOntology(path)) =
                                self.syn_pattern_expr_ty(syn_pattern_expr_idx)
                            else {
                                unreachable!()
                            };
                            let Some(PreludeTypePath::Num(PreludeNumTypePath::Int(path))) =
                                path.prelude_ty_path(db)
                            else {
                                todo!()
                            };
                            match path {
                                PreludeIntTypePath::I8 => TermLiteral::I8(value as i8),
                                PreludeIntTypePath::I16 => TermLiteral::I16(value as i16),
                                PreludeIntTypePath::I32 => TermLiteral::I32(value as i32),
                                PreludeIntTypePath::I64 => {
                                    TermLiteral::I64(TermI64Literal::new(db, value as i64))
                                }
                                PreludeIntTypePath::I128 => todo!(),
                                PreludeIntTypePath::ISize => todo!(),
                                PreludeIntTypePath::U8 => TermLiteral::U8(value as u8),
                                PreludeIntTypePath::U16 => TermLiteral::U16(value as u16),
                                PreludeIntTypePath::U32 => TermLiteral::U32(value as u32),
                                PreludeIntTypePath::U64 => {
                                    TermLiteral::U64(TermU64Literal::new(db, value as u64))
                                }
                                PreludeIntTypePath::U128 => todo!(),
                                PreludeIntTypePath::USize => {
                                    TermLiteral::USize(TermUSizeLiteral::new(db, value as u64))
                                }
                                PreludeIntTypePath::R8 => TermLiteral::R8(value as u8),
                                PreludeIntTypePath::R16 => TermLiteral::R16(value as u16),
                                PreludeIntTypePath::R32 => TermLiteral::R32(value as u32),
                                PreludeIntTypePath::R64 => {
                                    TermLiteral::R64(TermR64Literal::new(db, value as u64))
                                }
                                PreludeIntTypePath::R128 => todo!(),
                                PreludeIntTypePath::RSize => todo!(),
                            }
                        }
                        IntegerLikeLiteralData::UnspecifiedLarge() => todo!(),
                        IntegerLikeLiteralData::I8(_) => todo!(),
                        IntegerLikeLiteralData::I16(_) => todo!(),
                        IntegerLikeLiteralData::I32(_) => todo!(),
                        IntegerLikeLiteralData::I64(_) => todo!(),
                        IntegerLikeLiteralData::I128(_) => todo!(),
                        IntegerLikeLiteralData::ISize(_) => todo!(),
                        IntegerLikeLiteralData::R8(_) => todo!(),
                        IntegerLikeLiteralData::R16(_) => todo!(),
                        IntegerLikeLiteralData::R32(_) => todo!(),
                        IntegerLikeLiteralData::R64(_) => todo!(),
                        IntegerLikeLiteralData::R128(_) => todo!(),
                        IntegerLikeLiteralData::RSize(_) => todo!(),
                        IntegerLikeLiteralData::U8(_) => todo!(),
                        IntegerLikeLiteralData::U16(_) => todo!(),
                        IntegerLikeLiteralData::U32(_) => todo!(),
                        IntegerLikeLiteralData::U64(_) => todo!(),
                        IntegerLikeLiteralData::U128(_) => todo!(),
                        IntegerLikeLiteralData::USize(_) => todo!(),
                    },
                    LiteralData::Float(_) => todo!(),
                    LiteralData::TupleIndex(_) => todo!(),
                    LiteralData::Bool(_) => todo!(),
                })
            }
            SynPatternExpr::Ident {
                symbol_modifier_tokens,
                ident_token,
            } => HirEagerPatternExpr::Ident {
                ident: ident_token.ident(),
                symbol_modifier: symbol_modifier_tokens.map(Into::into),
            },
            SynPatternExpr::TypeVariantUnit { path, .. } => HirEagerPatternExpr::Unit(path.into()),
            SynPatternExpr::Tuple { name: _, fields: _ } => todo!(),
            SynPatternExpr::Props { name: _, fields: _ } => todo!(),
            SynPatternExpr::OneOf { ref options } => {
                let hir_eager_options = options
                    .elements()
                    .iter()
                    .map(|option| self.new_pattern_expr_aux(option.syn_pattern_expr_idx()))
                    .collect();
                HirEagerPatternExpr::OneOf {
                    options: self.alloc_pattern_exprs(
                        hir_eager_options,
                        options
                            .elements()
                            .iter()
                            .map(|option| option.syn_pattern_expr_idx()),
                    ),
                }
            }
            SynPatternExpr::Binding {
                ident_token: _,
                asperand_token: _,
                src: _,
            } => todo!(),
            SynPatternExpr::Range {
                start: _,
                dot_dot_token: _,
                end: _,
            } => todo!(),
        }
    }
}
