use crate::*;
use husky_entity_path::path::{
    major_item::ty::{PreludeNumTypePath, PreludeTypePath},
    PatternPath,
};
use husky_eth_term::term::EthTerm;
use husky_hir_ty::ritchie::HirContract;
use husky_place::place::EthPlace;
use husky_syn_expr::{
    context::SynPatternRoot,
    pattern::{SynPatternData, SynPatternIdx},
};
use husky_term_prelude::literal::{
    int::{
        I128Literal, I64Literal, ISizeLiteral, R128Literal, R64Literal, RSizeLiteral, U128Literal,
        U64Literal, USizeLiteral,
    },
    Literal,
};
use husky_token_data::{IntegerLikeLiteralTokenData, LiteralTokenData};
use variable::runtime::HirEagerRuntimeVariableIdx;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirEagerPatternData {
    /// example: `1`
    Literal(Literal),
    /// example: `a`
    Ident {
        symbol_modifier: Option<VariableModifier>,
        ident: Ident,
        variable_idx: HirEagerRuntimeVariableIdx,
    },
    /// example: `A::B`
    UnitPath(PatternPath),
    /// example: `(a, b)`
    Tuple {
        path: Option<PatternPath>,
        fields: HirEagerPatternIdxRange,
    },
    /// example: `C { .. }`
    Props {
        path: Option<PatternPath>,
        // todo: change to punctuated
        fields: HirEagerPatternIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf {
        options: HirEagerPatternIdxRange,
    },
    /// example: `x @ 1..9`
    Binding {
        ident: Ident,
        /// example: `1..9`
        src: HirEagerPatternIdx,
    },
    /// example: `1..9`
    Range {
        start: HirEagerPatternIdx,
        end: HirEagerPatternIdx,
    },
    Some,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerPatternEntry {
    data: HirEagerPatternData,
    contract: HirContract,
}

/// # constructor
impl HirEagerPatternEntry {
    pub fn new(data: HirEagerPatternData, contract: HirContract) -> Self {
        Self { data, contract }
    }
}

/// # getters
impl HirEagerPatternEntry {
    pub fn data(&self) -> &HirEagerPatternData {
        &self.data
    }

    pub fn contract(&self) -> HirContract {
        self.contract
    }

    pub fn is_destructive(&self) -> bool {
        self.contract.is_destructive()
    }
}

pub type HirEagerPatternArena = Arena<HirEagerPatternEntry>;
pub type HirEagerPatternIdx = ArenaIdx<HirEagerPatternEntry>;
pub type HirEagerPatternIdxRange = ArenaIdxRange<HirEagerPatternEntry>;
pub type HirEagerPatternMap<V> = ArenaMap<HirEagerPatternEntry, V>;
pub type HirEagerPatternOrderedMap<V> = ArenaOrderedMap<HirEagerPatternEntry, V>;

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_pattern(
        &mut self,
        syn_pattern_root: impl Into<SynPatternRoot>,
    ) -> HirEagerPatternIdx {
        let syn_pattern = syn_pattern_root.into().syn_pattern_idx();
        let pattern_data = self.new_pattern_aux(syn_pattern);
        self.alloc_pattern(pattern_data, syn_pattern)
    }

    fn new_pattern_aux(&mut self, syn_pattern: SynPatternIdx) -> HirEagerPatternData {
        let db = self.db();
        match self.syn_expr_region_data()[syn_pattern] {
            SynPatternData::Literal { literal, .. } => {
                HirEagerPatternData::Literal(match literal {
                    LiteralTokenData::Unit => Literal::Unit(()),
                    LiteralTokenData::Char(_) => todo!(),
                    LiteralTokenData::String(_) => todo!(),
                    LiteralTokenData::Integer(literal) => match literal {
                        IntegerLikeLiteralTokenData::UnspecifiedRegular(value) => {
                            let EthTerm::ItemPath(ItemPathTerm::TypeOntology(path)) =
                                self.syn_pattern_ty(syn_pattern)
                            else {
                                unreachable!()
                            };
                            let Some(PreludeTypePath::Num(PreludeNumTypePath::Int(path))) =
                                path.prelude(db)
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
                            Literal::I64(I64Literal::new(db, val))
                        }
                        IntegerLikeLiteralTokenData::I128(val) => {
                            Literal::I128(I128Literal::new(db, val))
                        }
                        IntegerLikeLiteralTokenData::ISize(val) => {
                            Literal::ISize(ISizeLiteral::new(db, val as i64))
                        }
                        IntegerLikeLiteralTokenData::R8(val) => Literal::R8(val),
                        IntegerLikeLiteralTokenData::R16(val) => Literal::R16(val),
                        IntegerLikeLiteralTokenData::R32(val) => Literal::R32(val),
                        IntegerLikeLiteralTokenData::R64(val) => {
                            Literal::R64(R64Literal::new(db, val as u64))
                        }
                        IntegerLikeLiteralTokenData::R128(val) => {
                            Literal::R128(R128Literal::new(db, val as u128))
                        }
                        IntegerLikeLiteralTokenData::RSize(val) => {
                            Literal::RSize(RSizeLiteral::new(db, val as u64))
                        }
                        IntegerLikeLiteralTokenData::U8(val) => Literal::U8(val),
                        IntegerLikeLiteralTokenData::U16(val) => Literal::U16(val),
                        IntegerLikeLiteralTokenData::U32(val) => Literal::U32(val),
                        IntegerLikeLiteralTokenData::U64(val) => {
                            Literal::U64(U64Literal::new(db, val as u64))
                        }
                        IntegerLikeLiteralTokenData::U128(val) => {
                            Literal::U128(U128Literal::new(db, val as u128))
                        }
                        IntegerLikeLiteralTokenData::USize(val) => {
                            Literal::USize(USizeLiteral::new(db, val as u64))
                        }
                    },
                    LiteralTokenData::Float(_) => todo!(),
                    LiteralTokenData::Bool(_) => todo!(),
                })
            }
            SynPatternData::Ident {
                symbol_modifier_tokens,
                ident_token,
            } => {
                let map = self
                    .syn_expr_region_data()
                    .syn_pattern_current_variables_mapped(syn_pattern, |current_variable_idx| {
                        self.current_variable_to_hir_eager_runtime_variable(current_variable_idx)
                    });
                debug_assert_eq!(map.len(), 1);
                debug_assert_eq!(map.data()[0].0, ident_token.ident());
                let variable_idx = map.data()[0].1.unwrap();
                HirEagerPatternData::Ident {
                    ident: ident_token.ident(),
                    symbol_modifier: symbol_modifier_tokens.map(Into::into),
                    variable_idx,
                }
            }
            SynPatternData::UnitTypeVariant { path, .. } => {
                HirEagerPatternData::UnitPath(path.into())
            }
            SynPatternData::Tuple { .. } => todo!(),
            SynPatternData::TupleStruct { .. } => todo!(),
            SynPatternData::TupleTypeVariant { path, .. } => {
                // ad hoc
                if path.ident(db).data(db) == "Some" {
                    HirEagerPatternData::Some
                } else {
                    todo!()
                }
            }
            SynPatternData::Props { name: _, fields: _ } => todo!(),
            SynPatternData::OneOf { ref options } => {
                let hir_eager_options = options
                    .elements()
                    .iter()
                    .map(|option| {
                        let syn_pattern = option.syn_pattern();
                        self.new_pattern_aux(syn_pattern)
                    })
                    .collect();
                HirEagerPatternData::OneOf {
                    options: self.alloc_patterns(
                        hir_eager_options,
                        options.elements().iter().map(|option| option.syn_pattern()),
                    ),
                }
            }
            SynPatternData::Binding {
                ident_token: _,
                asperand_token: _,
                src: _,
            } => todo!(),
            SynPatternData::Range {
                start: _,
                dot_dot_token: _,
                end: _,
            } => todo!(),
        }
    }
}
