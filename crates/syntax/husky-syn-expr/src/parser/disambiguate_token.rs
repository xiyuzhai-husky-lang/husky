use super::*;
use husky_entity_kind::{ritchie::RitchieItemKind, EntityKind, MajorFugitiveKind, MajorItemKind};
use husky_opr::*;
use husky_term_prelude::ritchie::RitchieKind;
use husky_token_data::delimiter::Delimiter;

pub type TokenDisambiguationResult<T> = ControlFlow<(), T>;

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn disambiguate_token(
        &mut self,
        regional_token_idx: RegionalTokenIdx,
        token: TokenData,
    ) -> TokenDisambiguationResult<DisambiguatedTokenData> {
        TokenDisambiguationResult::Continue(match token {
            TokenData::Keyword(keyword) => match keyword {
                Keyword::Connection(keyword) => match keyword {
                    ConnectionKeyword::For
                    | ConnectionKeyword::Where
                    | ConnectionKeyword::Extends => return TokenDisambiguationResult::Break(()),
                },
                Keyword::End(_) => return TokenDisambiguationResult::Break(()),
                Keyword::Pronoun(pronoun) => {
                    let crate_root_path = self.context().crate_root_path();
                    match pronoun {
                        PronounKeyword::Crate => DisambiguatedTokenData::IdentifiableEntityPath(
                            self.parse_identifiable_item_path_expr(
                                CrateRegionalToken::new(regional_token_idx).into(),
                                crate_root_path.into(),
                            ),
                        ),
                        PronounKeyword::SelfType => match self.allow_self_ty() {
                            AllowSelfType::True => {
                                DisambiguatedTokenData::SelfType(regional_token_idx)
                            }
                            AllowSelfType::False => DisambiguatedTokenData::Err(
                                OriginalSynExprError::SelfTypeNotAllowed(regional_token_idx).into(),
                            ),
                        },
                        PronounKeyword::SelfValue => match self.peek() {
                            Some(TokenData::Punctuation(Punctuation::COLON_COLON)) => {
                                todo!()
                            }
                            _ => match self.allow_self_value() {
                                AllowSelfValue::True => {
                                    DisambiguatedTokenData::SelfValue(regional_token_idx)
                                }
                                AllowSelfValue::False => DisambiguatedTokenData::Err(
                                    OriginalSynExprError::SelfValueNotAllowed(regional_token_idx)
                                        .into(),
                                ),
                            },
                        },
                        PronounKeyword::Super => todo!(),
                    }
                }
                Keyword::Fugitive(FugitiveKeyword::Fn) => {
                    DisambiguatedTokenData::Ritchie(regional_token_idx, RitchieItemKind::Fn.into())
                }
                Keyword::Sorry => DisambiguatedTokenData::Sorry { regional_token_idx },
                Keyword::Todo => DisambiguatedTokenData::Todo { regional_token_idx },
                Keyword::Unreachable => DisambiguatedTokenData::Unreachable { regional_token_idx },
                _ => DisambiguatedTokenData::Err(
                    OriginalSynExprError::UnexpectedKeyword(regional_token_idx).into(),
                ),
            },
            TokenData::Ident(ident) => match self.top_expr() {
                TopExprRef::Incomplete(
                    IncompleteSynExprData::CommaList {
                        opr:
                            IncompleteCommaListOpr::FunctionApplicationOrCall { .. }
                            | IncompleteCommaListOpr::MethodApplicationOrCall { .. },
                        ..
                    }
                    | IncompleteSynExprData::CallList { .. },
                ) => match self.try_parse_err_as_none::<EqRegionalToken>() {
                    Some(eq_token) => DisambiguatedTokenData::IncompleteKeywordArgument {
                        regional_token_idx,
                        ident,
                        eq_token,
                    },
                    None => self.resolve_ident(regional_token_idx, ident),
                },
                _ => self.resolve_ident(regional_token_idx, ident),
            },
            TokenData::Label(_) => todo!(),
            TokenData::Punctuation(punct) => match punct.mapped() {
                PunctuationMapped::Binary(binary) => {
                    DisambiguatedTokenData::SynBinaryOpr(regional_token_idx, binary)
                }
                PunctuationMapped::LeftDelimiter(bra) => {
                    DisambiguatedTokenData::LeftDelimiter(regional_token_idx, bra)
                }
                PunctuationMapped::RightDelimiter(ket) => match self.last_bra() {
                    Some(bra) => {
                        if bra != ket {
                            todo!()
                        }
                        DisambiguatedTokenData::RightDelimiter(regional_token_idx, ket)
                    }
                    None => return TokenDisambiguationResult::Break(()),
                },
                PunctuationMapped::Suffix(suffix) => {
                    DisambiguatedTokenData::SynSuffixOpr(regional_token_idx, suffix)
                }
                PunctuationMapped::LaOrLt => match self.top_expr() {
                    TopExprRef::Incomplete(_) => todo!(),
                    TopExprRef::Finished(expr) => {
                        match expr.base_item_path(self.db(), &self.context().syn_expr_arena()) {
                            BaseEntityPath::UncertainDueToError {
                                inclination: BaseEntityPathInclination::TypeOrVariant,
                            } => DisambiguatedTokenData::LeftDelimiter(
                                regional_token_idx,
                                Delimiter::TurboFish,
                            ),
                            BaseEntityPath::Some(item_path) => {
                                match item_path.item_kind(self.db()) {
                                    EntityKind::Module => todo!(),
                                    EntityKind::MajorItem {
                                        module_item_kind, ..
                                    } => match module_item_kind {
                                        MajorItemKind::Fugitive(
                                            MajorFugitiveKind::Val | MajorFugitiveKind::Const,
                                        ) => DisambiguatedTokenData::SynBinaryOpr(
                                            regional_token_idx,
                                            BinaryComparisonOpr::Less.into(),
                                        ),
                                        MajorItemKind::Type(_)
                                        | MajorItemKind::Fugitive(_)
                                        | MajorItemKind::Trait => {
                                            DisambiguatedTokenData::LeftDelimiter(
                                                regional_token_idx,
                                                Delimiter::TurboFish,
                                            )
                                        }
                                    },
                                    EntityKind::AssocItem { .. } => todo!(),
                                    EntityKind::TypeVariant => todo!(),
                                    EntityKind::ImplBlock => todo!(),
                                    EntityKind::Attr => todo!(),
                                }
                            }
                            _ => DisambiguatedTokenData::SynBinaryOpr(
                                regional_token_idx,
                                SynBinaryOpr::Comparison(BinaryComparisonOpr::Less),
                            ),
                        }
                    }
                    TopExprRef::None => DisambiguatedTokenData::LeftDelimiter(
                        regional_token_idx,
                        Delimiter::HtmlAngle,
                    ),
                },
                PunctuationMapped::ColonColonLa => todo!(),
                PunctuationMapped::RaOrGt => match (self.last_bra(), self.env_bra()) {
                    (Some(Delimiter::TurboFish), _) => DisambiguatedTokenData::RightDelimiter(
                        regional_token_idx,
                        Delimiter::TurboFish,
                    ),
                    (None, Some(Delimiter::TurboFish)) => {
                        return TokenDisambiguationResult::Break(())
                    }
                    _ => DisambiguatedTokenData::SynBinaryOpr(
                        regional_token_idx,
                        BinaryComparisonOpr::Greater.into(),
                    ),
                },
                PunctuationMapped::Sheba => DisambiguatedTokenData::Err(
                    OriginalSynExprError::UnexpectedSheba(regional_token_idx).into(),
                ),
                PunctuationMapped::Shr => DisambiguatedTokenData::SynBinaryOpr(
                    regional_token_idx,
                    SynBinaryOpr::Shift(BinaryShiftOpr::Shr),
                ),
                PunctuationMapped::DeriveAssign => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Minus => {
                    DisambiguatedTokenData::SynPrefixOpr(regional_token_idx, SynPrefixOpr::Minus)
                }
                PunctuationMapped::VertVert => todo!(),
                PunctuationMapped::Tilde => {
                    DisambiguatedTokenData::SynPrefixOpr(regional_token_idx, SynPrefixOpr::Tilde)
                }
                PunctuationMapped::Dot => DisambiguatedTokenData::Dot(regional_token_idx),
                PunctuationMapped::Colon => match self.last_incomplete_expr() {
                    Some(IncompleteSynExprData::CommaList {
                        opr: IncompleteCommaListOpr::BoxList { .. },
                        items,
                        ..
                    }) => {
                        if items.len() == 0 && self.complete_expr().is_none() {
                            DisambiguatedTokenData::ColonRightAfterLbox(regional_token_idx)
                        } else {
                            match self.token_stream.is_empty() {
                                true => return TokenDisambiguationResult::Break(()),
                                false => DisambiguatedTokenData::SynBinaryOpr(
                                    regional_token_idx,
                                    SynBinaryOpr::Ins,
                                ),
                            }
                        }
                    }
                    _ => match self.peek() {
                        // not end of token group
                        Some(_) => DisambiguatedTokenData::SynBinaryOpr(
                            regional_token_idx,
                            SynBinaryOpr::Ins,
                        ),
                        // end of token group
                        None => return TokenDisambiguationResult::Break(()),
                    },
                },
                PunctuationMapped::Comma => {
                    self.reduce(Precedence::ListItem);
                    match self.last_incomplete_expr() {
                        Some(expr) => match expr {
                            IncompleteSynExprData::CommaList { .. }
                            | IncompleteSynExprData::CallList { .. } => {
                                DisambiguatedTokenData::Comma(regional_token_idx)
                            }
                            _ => return TokenDisambiguationResult::Break(()),
                        },
                        None => return TokenDisambiguationResult::Break(()),
                    }
                }
                PunctuationMapped::Vert => match (self.env(), self.last_bra()) {
                    (
                        Some(ExprEnvironment::WithinDelimiteredParameterList(Delimiter::Vert)),
                        None,
                    ) => return TokenDisambiguationResult::Break(()),
                    _ => match self.complete_expr().is_some() {
                        true => DisambiguatedTokenData::SynBinaryOpr(
                            regional_token_idx,
                            SynBinaryOpr::Closed(BinaryClosedOpr::BitOr),
                        ),
                        false => DisambiguatedTokenData::LeftDelimiter(
                            regional_token_idx,
                            Delimiter::Vert,
                        ),
                    },
                },
                PunctuationMapped::DoubleExclamation => todo!(),
                PunctuationMapped::Semicolon => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::EmptyHtmlKet => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::At => DisambiguatedTokenData::At(regional_token_idx),
                PunctuationMapped::AtEq => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Exclamation => self.resolve_prefix_or_other(
                    regional_token_idx,
                    SynPrefixOpr::Not,
                    DisambiguatedTokenData::SynSuffixOpr(
                        regional_token_idx,
                        SynSuffixOpr::UnwrapOrComposeWithNot,
                    ),
                ),
                PunctuationMapped::Question => self.resolve_prefix_or_other(
                    regional_token_idx,
                    SynPrefixOpr::Option,
                    DisambiguatedTokenData::SynSuffixOpr(
                        regional_token_idx,
                        SynSuffixOpr::UnveilOrComposeWithOption,
                    ),
                ),
                PunctuationMapped::Pound => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Ambersand => self.resolve_prefix_or_other(
                    regional_token_idx,
                    SynPrefixOpr::Ref,
                    DisambiguatedTokenData::SynBinaryOpr(
                        regional_token_idx,
                        SynBinaryOpr::Closed(BinaryClosedOpr::BitAnd),
                    ),
                ),
                PunctuationMapped::DotDot => todo!(),
                PunctuationMapped::DotDotDot => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Star => DisambiguatedTokenData::SynBinaryOpr(
                    regional_token_idx,
                    SynBinaryOpr::Closed(BinaryClosedOpr::Mul),
                ),
                PunctuationMapped::Eq => match self.env() {
                    Some(env) => match env {
                        ExprEnvironment::TypeBeforeEq
                        | ExprEnvironment::WithinDelimiteredParameterList(_) => {
                            match self.last_bra() {
                                Some(_) => todo!(),
                                None => return TokenDisambiguationResult::Break(()),
                            }
                        }
                        ExprEnvironment::Condition(_) => todo!(),
                    },
                    None => DisambiguatedTokenData::SynBinaryOpr(
                        regional_token_idx,
                        SynBinaryOpr::AssignOrDefEq,
                    ),
                },
                PunctuationMapped::ForAll => todo!(),
                PunctuationMapped::Exists => todo!(),
                PunctuationMapped::HeavyArrow => return TokenDisambiguationResult::Break(()),
            },
            TokenData::WordOpr(opr) => match opr {
                WordOpr::And => DisambiguatedTokenData::SynBinaryOpr(
                    regional_token_idx,
                    SynBinaryOpr::ShortCircuitLogic(BinaryShortcuitLogicOpr::And),
                ),
                WordOpr::Or => DisambiguatedTokenData::SynBinaryOpr(
                    regional_token_idx,
                    SynBinaryOpr::ShortCircuitLogic(BinaryShortcuitLogicOpr::Or),
                ),
                WordOpr::As => {
                    DisambiguatedTokenData::SynBinaryOpr(regional_token_idx, SynBinaryOpr::As)
                }
                WordOpr::Be => DisambiguatedTokenData::Be(regional_token_idx),
            },
            TokenData::Literal(literal) => {
                DisambiguatedTokenData::Literal(regional_token_idx, literal)
            }
            TokenData::Error(error) => {
                DisambiguatedTokenData::Err(DerivedSynExprError::TokenData(error).into())
            }
        })
    }

    fn resolve_prefix_or_other(
        &mut self,
        regional_token_idx: RegionalTokenIdx,
        prefix_opr: SynPrefixOpr,
        other: DisambiguatedTokenData,
    ) -> DisambiguatedTokenData {
        match self.complete_expr() {
            Some(SynExprData::List { .. }) | Some(SynExprData::BoxColonList { .. }) | None => {
                DisambiguatedTokenData::SynPrefixOpr(regional_token_idx, prefix_opr)
            }
            Some(_) => other,
        }
    }
}

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    fn resolve_ident(
        &mut self,
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
    ) -> DisambiguatedTokenData {
        if let Some(opn) = self.last_incomplete_expr() {
            match opn {
                IncompleteSynExprData::Binary {
                    punctuation: SynBinaryOpr::ScopeResolution,
                    ..
                } => {
                    return DisambiguatedTokenData::AssocItem {
                        ident,
                        regional_token_idx,
                    }
                }
                _ => (),
            }
        }
        match self.context().syn_symbol_context().resolve_ident(
            self.db(),
            self.context().module_path(),
            regional_token_idx,
            ident,
        ) {
            Some(symbol) => match symbol {
                Symbol::PrincipalEntity(item_path) => {
                    DisambiguatedTokenData::IdentifiableEntityPath(
                        self.parse_identifiable_item_path_expr(
                            IdentRegionalToken::new(ident, regional_token_idx).into(),
                            item_path,
                        ),
                    )
                }
                Symbol::Inherited(inherited_syn_symbol_idx, inherited_syn_symbol_kind) => {
                    DisambiguatedTokenData::InheritedSynSymbol {
                        ident,
                        regional_token_idx,
                        inherited_syn_symbol_idx,
                        inherited_syn_symbol_kind,
                    }
                }
                Symbol::Current(current_variable_idx, current_variable_kind) => {
                    DisambiguatedTokenData::CurrentSynSymbol {
                        ident,
                        regional_token_idx,
                        current_variable_idx,
                        current_variable_kind,
                    }
                }
            },
            None => DisambiguatedTokenData::UnrecognizedIdent {
                regional_token_idx,
                ident,
            },
        }
    }
}

#[derive(Debug)]
pub(crate) enum DisambiguatedTokenData {
    Literal(RegionalTokenIdx, LiteralTokenData),
    IdentifiableEntityPath(ItemPathExpr),
    InheritedSynSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        inherited_syn_symbol_idx: InheritedSymbolicVariableIdx,
        inherited_syn_symbol_kind: InheritedVariableKind,
    },
    CurrentSynSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        current_variable_idx: CurrentVariableIdx,
        current_variable_kind: CurrentVariableKind,
    },
    AssocItem {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
    },
    SelfType(RegionalTokenIdx),
    SelfValue(RegionalTokenIdx),
    /// sorry is for comptime (say proof) terms
    Sorry {
        regional_token_idx: RegionalTokenIdx,
    },
    /// todo is for runtime terms
    Todo {
        regional_token_idx: RegionalTokenIdx,
    },
    Unreachable {
        regional_token_idx: RegionalTokenIdx,
    },
    UnrecognizedIdent {
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
    },
    SynBinaryOpr(RegionalTokenIdx, SynBinaryOpr),
    SynPrefixOpr(RegionalTokenIdx, SynPrefixOpr),
    SynSuffixOpr(RegionalTokenIdx, SynSuffixOpr),
    LeftDelimiter(RegionalTokenIdx, Delimiter),
    RightDelimiter(RegionalTokenIdx, Delimiter),
    Dot(RegionalTokenIdx),
    Comma(RegionalTokenIdx),
    Be(RegionalTokenIdx),
    ColonRightAfterLbox(RegionalTokenIdx),
    Ritchie(RegionalTokenIdx, RitchieKind),
    IncompleteKeywordArgument {
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
        eq_token: EqRegionalToken,
    },
    At(RegionalTokenIdx),
    Err(SynExprError),
}
