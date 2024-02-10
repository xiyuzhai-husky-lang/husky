use crate::*;
use husky_eth_term::term::EthTerm;
use husky_syn_expr::{SynPatternExprData, SynPatternExprIdx, SynPatternExprRoot};
use husky_term_prelude::literal::{
    int::{
        TermI128Literal, TermI64Literal, TermISizeLiteral, TermR128Literal, TermR64Literal,
        TermRSizeLiteral, TermU128Literal, TermU64Literal, TermUSizeLiteral,
    },
    Literal,
};
use husky_token_data::{IntegerLikeLiteralTokenData, LiteralTokenData};

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirEagerPatternExpr {
    /// example: `1`
    Literal(Literal),
    /// example: `a`
    Ident {
        symbol_modifier: Option<SvarModifier>,
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
    Some,
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
            SynPatternExprData::Literal { literal, .. } => {
                HirEagerPatternExpr::Literal(match literal {
                    LiteralTokenData::Unit => Literal::Unit(()),
                    LiteralTokenData::Char(_) => todo!(),
                    LiteralTokenData::String(_) => todo!(),
                    LiteralTokenData::Integer(literal) => match literal {
                        IntegerLikeLiteralTokenData::UnspecifiedRegular(value) => {
                            let EthTerm::EntityPath(ItemPathTerm::TypeOntology(path)) =
                                self.syn_pattern_expr_ty(syn_pattern_expr_idx)
                            else {
                                unreachable!()
                            };
                            let Some(PreludeTypePath::Num(PreludeNumTypePath::Int(path))) =
                                path.prelude_ty_path(db)
                            else {
                                todo!()
                            };
                            Literal::from_unspecified_int(path, value, db)
                        }
                        IntegerLikeLiteralTokenData::UnspecifiedLarge() => todo!(),
                        IntegerLikeLiteralTokenData::I8(val) => Literal::I8(val),
                        IntegerLikeLiteralTokenData::I16(val) => Literal::I16(val),
                        IntegerLikeLiteralTokenData::I32(val) => Literal::I32(val),
                        IntegerLikeLiteralTokenData::I64(val) => {
                            Literal::I64(TermI64Literal::new(db, val))
                        }
                        IntegerLikeLiteralTokenData::I128(val) => {
                            Literal::I128(TermI128Literal::new(db, val))
                        }
                        IntegerLikeLiteralTokenData::ISize(val) => {
                            Literal::ISize(TermISizeLiteral::new(db, val as i64))
                        }
                        IntegerLikeLiteralTokenData::R8(val) => Literal::R8(val),
                        IntegerLikeLiteralTokenData::R16(val) => Literal::R16(val),
                        IntegerLikeLiteralTokenData::R32(val) => Literal::R32(val),
                        IntegerLikeLiteralTokenData::R64(val) => {
                            Literal::R64(TermR64Literal::new(db, val as u64))
                        }
                        IntegerLikeLiteralTokenData::R128(val) => {
                            Literal::R128(TermR128Literal::new(db, val as u128))
                        }
                        IntegerLikeLiteralTokenData::RSize(val) => {
                            Literal::RSize(TermRSizeLiteral::new(db, val as u64))
                        }
                        IntegerLikeLiteralTokenData::U8(val) => Literal::U8(val),
                        IntegerLikeLiteralTokenData::U16(val) => Literal::U16(val),
                        IntegerLikeLiteralTokenData::U32(val) => Literal::U32(val),
                        IntegerLikeLiteralTokenData::U64(val) => {
                            Literal::U64(TermU64Literal::new(db, val as u64))
                        }
                        IntegerLikeLiteralTokenData::U128(val) => {
                            Literal::U128(TermU128Literal::new(db, val as u128))
                        }
                        IntegerLikeLiteralTokenData::USize(val) => {
                            Literal::USize(TermUSizeLiteral::new(db, val as u64))
                        }
                    },
                    LiteralTokenData::Float(_) => todo!(),
                    LiteralTokenData::Bool(_) => todo!(),
                })
            }
            SynPatternExprData::Ident {
                symbol_modifier_tokens,
                ident_token,
            } => HirEagerPatternExpr::Ident {
                ident: ident_token.ident(),
                symbol_modifier: symbol_modifier_tokens.map(Into::into),
            },
            SynPatternExprData::UnitTypeVariant { path, .. } => {
                HirEagerPatternExpr::Unit(path.into())
            }
            SynPatternExprData::Tuple { .. } => todo!(),
            SynPatternExprData::TupleStruct { .. } => todo!(),
            SynPatternExprData::TupleTypeVariant { path, .. } => {
                // ad hoc
                if path.ident(db).data(db) == "Some" {
                    HirEagerPatternExpr::Some
                } else {
                    todo!()
                }
            }
            SynPatternExprData::Props { name: _, fields: _ } => todo!(),
            SynPatternExprData::OneOf { ref options } => {
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
            SynPatternExprData::Binding {
                ident_token: _,
                asperand_token: _,
                src: _,
            } => todo!(),
            SynPatternExprData::Range {
                start: _,
                dot_dot_token: _,
                end: _,
            } => todo!(),
        }
    }
}
