mod contract;
mod region;
mod symbol;

use std::ops::ControlFlow;

pub use self::region::*;
pub use self::symbol::*;

use super::*;
use husky_coword::Ident;
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{ItemPath, TypeVariantPath};
use idx_arena::{map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};
use parsec::{IsStreamParser, PunctuatedSmallList, TryParseOptionFromStream};

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynPatternExpr {
    /// example: `1`
    Literal {
        regional_token_idx: RegionalTokenIdx,
        literal: LiteralData,
    },
    /// example: `a`
    Ident {
        symbol_modifier_tokens: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
    },
    /// example: `A::B`
    TypeVariantUnit {
        path_expr_idx: SynPrincipalEntityPathExprIdx,
        path: TypeVariantPath,
    },
    /// example: `(a, b)`
    Tuple {
        name: Option<ItemPath>,
        fields: SynPatternExprIdxRange,
    },
    /// example: `C { .. }`
    Props {
        name: Option<ItemPath>,
        // todo: change to punctuated
        fields: SynPatternExprIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf {
        options:
            PunctuatedSmallList<SynPatternComponent, VerticalRegionalToken, SynExprError, false, 3>,
    },
    /// example: `x @ 1..9`
    Binding {
        ident_token: IdentRegionalToken,
        asperand_token: AtRegionalToken,
        /// example: `1..9`
        src: SynPatternExprIdx,
    },
    /// example: `1..9`
    Range {
        start: SynPatternExprIdx,
        dot_dot_token: DotDotRegionalToken,
        end: SynPatternExprIdx,
    },
}

pub type SynPatternExprArena = Arena<SynPatternExpr>;
pub type SynPatternExprIdx = ArenaIdx<SynPatternExpr>;
pub type SynPatternExprIdxRange = ArenaIdxRange<SynPatternExpr>;
pub type SynPatternExprMap<V> = ArenaMap<SynPatternExpr, V>;
pub type SynPatternExprOrderedMap<V> = ArenaOrderedMap<SynPatternExpr, V>;

/// irreducible against `|`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynPatternComponent(SynPatternExprIdx);

impl SynPatternComponent {
    pub fn syn_pattern_expr_idx(self) -> SynPatternExprIdx {
        self.0
    }
}

impl<'a, C> TryParseOptionFromStream<SynExprParser<'a, C>> for SynPatternComponent
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        parser: &mut SynExprParser<'a, C>,
    ) -> Result<Option<Self>, Self::Error> {
        let symbol_modifier_tokens: Option<EphemSymbolModifierRegionalTokens> =
            parser.try_parse_option()?;
        let Some((regional_token_idx, token_data)) = parser.next_indexed() else {
            return Ok(None);
        };
        let db = parser.db();
        let expr = match parser.disambiguate_token(regional_token_idx, token_data) {
            ControlFlow::Continue(resolved_token) => match resolved_token {
                DisambiguatedTokenData::Literal(regional_token_idx, literal) => {
                    Some(SynPatternExpr::Literal {
                        regional_token_idx,
                        literal,
                    })
                }
                DisambiguatedTokenData::IdentifiableEntityPath(syn_expr) => match syn_expr {
                    IdentifiableEntityPathExpr::Principal {
                        path_expr_idx,
                        opt_path,
                    } => match opt_path {
                        Some(path) => match path {
                            // modules and major items will be overriden by pattern symbol
                            PrincipalEntityPath::Module(_) => parse_overriding_ident_pattern(
                                parser,
                                path_expr_idx,
                                symbol_modifier_tokens,
                            ),
                            PrincipalEntityPath::MajorItem(path) => match path {
                                MajorItemPath::Type(_) => todo!(),
                                MajorItemPath::Trait(_) => todo!(),
                                MajorItemPath::Fugitive(path) => match path.fugitive_kind(db) {
                                    FugitiveKind::FunctionFn
                                    | FugitiveKind::FunctionGn
                                    | FugitiveKind::Val => parse_overriding_ident_pattern(
                                        parser,
                                        path_expr_idx,
                                        symbol_modifier_tokens,
                                    ),
                                    FugitiveKind::AliasType => todo!(),
                                },
                            },
                            PrincipalEntityPath::TypeVariant(path) => {
                                if symbol_modifier_tokens.is_some() {
                                    todo!()
                                }
                                Some(SynPatternExpr::TypeVariantUnit {
                                    path_expr_idx,
                                    path,
                                })
                            }
                        },
                        None => todo!(),
                    },
                    IdentifiableEntityPathExpr::AssociatedItem { .. } => todo!(),
                },
                DisambiguatedTokenData::InheritedSynSymbol {
                    regional_token_idx,
                    ident,
                    ..
                }
                | DisambiguatedTokenData::CurrentSynSymbol {
                    regional_token_idx,
                    ident,
                    ..
                }
                | DisambiguatedTokenData::UnrecognizedIdent {
                    regional_token_idx,
                    ident,
                } => Some(SynPatternExpr::Ident {
                    symbol_modifier_tokens,
                    ident_token: IdentRegionalToken::new(ident, regional_token_idx),
                }),
                DisambiguatedTokenData::SelfType(_) => todo!(),
                DisambiguatedTokenData::SelfValue(_) => todo!(),
                DisambiguatedTokenData::Bra(_, _) => todo!(),
                _ => None,
            },
            ControlFlow::Break(_) => None,
        };
        let Some(expr) = expr else {
            if let Some(_) = symbol_modifier_tokens {
                todo!()
            } else {
                return Ok(None);
            }
        };
        Ok(Some(SynPatternComponent(
            parser.context.borrow_mut().alloc_pattern_expr(expr),
        )))
    }
}

fn parse_overriding_ident_pattern<'a, C>(
    parser: &mut SynExprParser<'a, C>,
    path_expr_idx: ArenaIdx<SynPrincipalEntityPathExpr>,
    symbol_modifier_tokens: Option<EphemSymbolModifierRegionalTokens>,
) -> Option<SynPatternExpr>
where
    C: IsSynExprContext<'a>,
{
    match parser.context().syn_principal_entity_path_expr_arena()[path_expr_idx] {
        SynPrincipalEntityPathExpr::Root {
            path_name_token, ..
        } => match path_name_token {
            PathNameRegionalToken::Ident(ident_token) => Some(SynPatternExpr::Ident {
                symbol_modifier_tokens,
                ident_token,
            }),
            PathNameRegionalToken::CrateRoot(_) => todo!(),
            PathNameRegionalToken::SelfMod(_) => todo!(),
            PathNameRegionalToken::Super(_) => todo!(),
        },
        SynPrincipalEntityPathExpr::Subitem { .. } => todo!(),
    }
}

// pub fn parse_pattern_expr(
//     &mut self,
//     env: SynPatternExprEnvironment,
// ) -> SynExprResult<Option<SynPatternExprIdx>> {
//     todo!("overhaul this");
//     let symbol_modifier_token_group = self.try_parse_option()?;
//     let ident_token = match symbol_modifier_token_group {
//         None => match self.try_parse_option::<IdentRegionalToken>()? {
//             Some(ident_token) => ident_token,
//             None => return Ok(None),
//         },
//         Some(ephem_symbol_modifier_token_group) => {
//             self.try_parse_expected(|token_stream_state| {
//                 OriginalSynExprError::ExpectedIdentAfterModifier(
//                     token_stream_state,
//                     ephem_symbol_modifier_token_group,
//                 )
//             })?
//         }
//     };
//     Ok(Some(self.context_mut().alloc_pattern_expr(
//         SynPatternExpr::Ident {
//             symbol_modifier_keyword_group: symbol_modifier_token_group,
//             ident_token,
//         },
//         env,
//     )))
//     // if let Some(ref_token) = self.parse::<RefToken>()? {
//     //     todo!()
//     // } else if let Some(mut_token) = self.parse::<MutToken>()? {
//     //     let ident_token: RegionalIdentToken =
//     //         self.parse_expected(OriginalExprError::ExpectedIdentAfterMut)?;
//     //     Ok(Some(self.alloc_pattern_expr(
//     //         PatternExpr::Ident {
//     //             ident_token,
//     //             modifier: SymbolModifierKeywordGroup::Mut(mut_token),
//     //         },
//     //         env,
//     //     )))
//     // } else if let Some(ident_token) = self.parse::<RegionalIdentToken>()? {
//     //     Ok(Some(self.alloc_pattern_expr(
//     //         PatternExpr::Ident {
//     //             ident_token,
//     //             modifier: SymbolModifierKeywordGroup::Pure,
//     //         },
//     //         env,
//     //     )))
//     // } else {
//     //     Ok(None)
//     // }
// }

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    fn try_parse_option_syn_pattern_expr_root_from_stream_without_guaranteed_rollback(
        self: &mut SynExprParser<'a, C>,
        root_kind: SynPatternExprRootKind,
    ) -> SynExprResult<Option<SynPatternExprRoot>> {
        let punctuated_patterns = self.try_parse::<PunctuatedSmallList<
            SynPatternComponent,
            VerticalRegionalToken,
            SynExprError,
            false,
            3,
        >>()?;
        match punctuated_patterns.elements().len() {
            0 => Ok(None),
            1 => Ok(Some(SynPatternExprRoot::new(
                root_kind,
                punctuated_patterns.elements()[0].0,
                self.context_mut(),
            ))),
            _ => {
                let expr = SynPatternExpr::OneOf {
                    options: punctuated_patterns,
                };
                Ok(Some(SynPatternExprRoot::new(
                    root_kind,
                    self.context_mut().alloc_pattern_expr(expr),
                    self.context_mut(),
                )))
            }
        }
    }
}

// parenate

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParenateSynPatternExprRoot {
    syn_pattern_expr_idx: SynPatternExprIdx,
}

impl ParenateSynPatternExprRoot {
    pub fn syn_pattern_expr_idx(&self) -> SynPatternExprIdx {
        self.syn_pattern_expr_idx
    }
}

impl<'a, C> TryParseOptionFromStream<SynExprParser<'a, C>> for ParenateSynPatternExprRoot
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SynExprParser<'a, C>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(root) = sp
            .try_parse_option_syn_pattern_expr_root_from_stream_without_guaranteed_rollback(
                SynPatternExprRootKind::Parenate,
            )?
        else {
            return Ok(None);
        };
        Ok(Some(ParenateSynPatternExprRoot {
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }))
    }
}

// let

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LetSynPatternExprRoot {
    syn_pattern_expr_idx: SynPatternExprIdx,
}

impl LetSynPatternExprRoot {
    pub fn syn_pattern_expr_idx(&self) -> SynPatternExprIdx {
        self.syn_pattern_expr_idx
    }
}

impl<'a, C> TryParseOptionFromStream<SynExprParser<'a, C>> for LetSynPatternExprRoot
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SynExprParser<'a, C>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(root) = sp
            .try_parse_option_syn_pattern_expr_root_from_stream_without_guaranteed_rollback(
                SynPatternExprRootKind::Let,
            )?
        else {
            return Ok(None);
        };
        Ok(Some(LetSynPatternExprRoot {
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }))
    }
}

// case

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CaseSynPatternExprRoot {
    syn_pattern_expr_idx: SynPatternExprIdx,
}

impl CaseSynPatternExprRoot {
    pub fn syn_pattern_expr_idx(&self) -> SynPatternExprIdx {
        self.syn_pattern_expr_idx
    }
}

impl<'a, C> TryParseOptionFromStream<SynExprParser<'a, C>> for CaseSynPatternExprRoot
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SynExprParser<'a, C>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(root) = sp
            .try_parse_option_syn_pattern_expr_root_from_stream_without_guaranteed_rollback(
                SynPatternExprRootKind::Case,
            )?
        else {
            return Ok(None);
        };
        Ok(Some(CaseSynPatternExprRoot {
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }))
    }
}

// be

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BeSynPatternExprRoot {
    syn_pattern_expr_idx: SynPatternExprIdx,
}

impl BeSynPatternExprRoot {
    pub fn syn_pattern_expr_idx(&self) -> SynPatternExprIdx {
        self.syn_pattern_expr_idx
    }
}

impl<'a, C> TryParseOptionFromStream<SynExprParser<'a, C>> for BeSynPatternExprRoot
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SynExprParser<'a, C>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(root) = sp
            .try_parse_option_syn_pattern_expr_root_from_stream_without_guaranteed_rollback(
                SynPatternExprRootKind::Be,
            )?
        else {
            return Ok(None);
        };
        Ok(Some(BeSynPatternExprRoot {
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }))
    }
}
