use super::*;
use husky_entity_taxonomy::{EntityKind, FugitiveKind, MajorItemKind};
use husky_print_utils::p;
use salsa::DebugWithDb;
use std::ops::ControlFlow;

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
                Keyword::Pronoun(pronoun) => match pronoun {
                    PronounKeyword::Crate => {
                        let crate_root_path = self.context().crate_root_path();
                        DisambiguatedTokenData::IdentifiableEntityPath(
                            self.parse_identifiable_item_path_expr(
                                CrateRegionalToken::new(regional_token_idx).into(),
                                crate_root_path.into(),
                            ),
                        )
                    }
                    PronounKeyword::SelfType => match self.allow_self_ty() {
                        AllowSelfType::True => DisambiguatedTokenData::SelfType(regional_token_idx),
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
                },
                Keyword::Fugitive(FugitiveKeyword::Fn) => {
                    DisambiguatedTokenData::Ritchie(regional_token_idx, RitchieKind::FnType)
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
                    IncompleteSynExpr::CommaList {
                        opr:
                            IncompleteCommaListOpr::FunctionApplicationOrCall { .. }
                            | IncompleteCommaListOpr::MethodApplicationOrCall { .. },
                        ..
                    }
                    | IncompleteSynExpr::CallList { .. },
                ) => match self.try_parse_err_as_none::<RegionalEqToken>() {
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
                    DisambiguatedTokenData::BinaryOpr(regional_token_idx, binary)
                }
                PunctuationMapped::Bra(bra) => DisambiguatedTokenData::Bra(regional_token_idx, bra),
                PunctuationMapped::Ket(ket) => match self.last_bra() {
                    Some(bra) => {
                        if bra != ket {
                            todo!()
                        }
                        DisambiguatedTokenData::Ket(regional_token_idx, ket)
                    }
                    None => return TokenDisambiguationResult::Break(()),
                },
                PunctuationMapped::Suffix(suffix) => {
                    DisambiguatedTokenData::SuffixOpr(regional_token_idx, suffix)
                }
                PunctuationMapped::LaOrLt => match self.top_expr() {
                    TopExprRef::Incomplete(_) => todo!(),
                    TopExprRef::Finished(expr) => {
                        match expr.base_item_path(self.db(), &self.context().syn_expr_arena()) {
                            BaseEntityPath::Uncertain {
                                inclination: BaseEntityPathInclination::TypeOrVariant,
                            } => DisambiguatedTokenData::Bra(
                                regional_token_idx,
                                Bracket::TemplateAngle,
                            ),
                            BaseEntityPath::Some(item_path) => {
                                match item_path.item_kind(self.db()) {
                                    EntityKind::Module => todo!(),
                                    EntityKind::MajorItem {
                                        module_item_kind,
                                        connection,
                                    } => match module_item_kind {
                                        MajorItemKind::Fugitive(FugitiveKind::Val) => {
                                            DisambiguatedTokenData::BinaryOpr(
                                                regional_token_idx,
                                                BinaryComparisonOpr::Less.into(),
                                            )
                                        }
                                        MajorItemKind::Type(_)
                                        | MajorItemKind::Fugitive(_)
                                        | MajorItemKind::Trait => DisambiguatedTokenData::Bra(
                                            regional_token_idx,
                                            Bracket::TemplateAngle,
                                        ),
                                    },
                                    EntityKind::AssociatedItem {
                                        associated_item_kind,
                                    } => todo!(),
                                    EntityKind::TypeVariant => todo!(),
                                    EntityKind::Trait => todo!(),
                                    EntityKind::ImplBlock => todo!(),
                                    EntityKind::Attr => todo!(),
                                }
                            }
                            _ => DisambiguatedTokenData::BinaryOpr(
                                regional_token_idx,
                                BinaryOpr::Comparison(BinaryComparisonOpr::Less),
                            ),
                        }
                    }
                    TopExprRef::None => {
                        DisambiguatedTokenData::Bra(regional_token_idx, Bracket::HtmlAngle)
                    }
                },
                PunctuationMapped::ColonColonLa => todo!(),
                PunctuationMapped::RaOrGt => match (self.last_bra(), self.env_bra()) {
                    (Some(Bracket::TemplateAngle), _) => {
                        DisambiguatedTokenData::Ket(regional_token_idx, Bracket::TemplateAngle)
                    }
                    (None, Some(Bracket::TemplateAngle)) => {
                        return TokenDisambiguationResult::Break(())
                    }
                    _ => DisambiguatedTokenData::BinaryOpr(
                        regional_token_idx,
                        BinaryComparisonOpr::Greater.into(),
                    ),
                },
                PunctuationMapped::Sheba => DisambiguatedTokenData::Err(
                    OriginalSynExprError::UnexpectedSheba(regional_token_idx).into(),
                ),
                PunctuationMapped::Shr => DisambiguatedTokenData::BinaryOpr(
                    regional_token_idx,
                    BinaryOpr::Shift(BinaryShiftOpr::Shr),
                ),
                PunctuationMapped::DeriveAssign => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Minus => {
                    DisambiguatedTokenData::PrefixOpr(regional_token_idx, PrefixOpr::Minus)
                }
                PunctuationMapped::DoubleVertical => todo!(),
                PunctuationMapped::Tilde => {
                    DisambiguatedTokenData::PrefixOpr(regional_token_idx, PrefixOpr::Tilde)
                }
                PunctuationMapped::Dot => DisambiguatedTokenData::Dot(regional_token_idx),
                PunctuationMapped::Colon => match self.last_incomplete_expr() {
                    Some(IncompleteSynExpr::CommaList {
                        opr: IncompleteCommaListOpr::BoxList { .. },
                        items,
                        ..
                    }) => {
                        if items.len() == 0 && self.complete_expr().is_none() {
                            DisambiguatedTokenData::ColonRightAfterLbox(regional_token_idx)
                        } else {
                            match self.token_stream.is_empty() {
                                true => return TokenDisambiguationResult::Break(()),
                                false => DisambiguatedTokenData::BinaryOpr(
                                    regional_token_idx,
                                    BinaryOpr::Ins,
                                ),
                            }
                        }
                    }
                    _ => match self.peek() {
                        // not end of token group
                        Some(_) => {
                            DisambiguatedTokenData::BinaryOpr(regional_token_idx, BinaryOpr::Ins)
                        }
                        // end of token group
                        None => return TokenDisambiguationResult::Break(()),
                    },
                },
                PunctuationMapped::Comma => {
                    self.reduce(Precedence::ListItem);
                    match self.last_incomplete_expr() {
                        Some(expr) => match expr {
                            IncompleteSynExpr::CommaList { .. }
                            | IncompleteSynExpr::CallList { .. } => {
                                DisambiguatedTokenData::Comma(regional_token_idx)
                            }
                            _ => return TokenDisambiguationResult::Break(()),
                        },
                        None => return TokenDisambiguationResult::Break(()),
                    }
                }
                PunctuationMapped::Vertical => match self.last_incomplete_expr() {
                    Some(IncompleteSynExpr::CommaList {
                        bra: Bracket::Lambda,
                        ..
                    }) => DisambiguatedTokenData::Ket(regional_token_idx, Bracket::Lambda),
                    _ => match self.complete_expr().is_some() {
                        true => DisambiguatedTokenData::BinaryOpr(
                            regional_token_idx,
                            BinaryOpr::Closed(BinaryClosedOpr::BitOr),
                        ),
                        false => DisambiguatedTokenData::Bra(regional_token_idx, Bracket::Lambda),
                    },
                },
                PunctuationMapped::DoubleExclamation => todo!(),
                PunctuationMapped::Semicolon => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::EmptyHtmlKet => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::At => DisambiguatedTokenData::At(regional_token_idx),
                PunctuationMapped::AtEq => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Exclamation => self.resolve_prefix_or_other(
                    regional_token_idx,
                    PrefixOpr::Not,
                    DisambiguatedTokenData::SuffixOpr(
                        regional_token_idx,
                        SuffixOpr::UnwrapOrComposeWithNot,
                    ),
                ),
                PunctuationMapped::Question => self.resolve_prefix_or_other(
                    regional_token_idx,
                    PrefixOpr::Option,
                    DisambiguatedTokenData::SuffixOpr(
                        regional_token_idx,
                        SuffixOpr::UnveilOrComposeWithOption,
                    ),
                ),
                PunctuationMapped::Pound => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Ambersand => self.resolve_prefix_or_other(
                    regional_token_idx,
                    PrefixOpr::Ref,
                    DisambiguatedTokenData::BinaryOpr(
                        regional_token_idx,
                        BinaryOpr::Closed(BinaryClosedOpr::BitOr),
                    ),
                ),
                PunctuationMapped::DotDot => todo!(),
                PunctuationMapped::DotDotDot => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Star => DisambiguatedTokenData::BinaryOpr(
                    regional_token_idx,
                    BinaryOpr::Closed(BinaryClosedOpr::Mul),
                ),
                PunctuationMapped::Eq => match self.env() {
                    Some(env) => match env {
                        ExprEnvironment::TypeBeforeEq
                        | ExprEnvironment::WithinBracketedParameterList(_) => match self.last_bra()
                        {
                            Some(_) => todo!(),
                            None => return TokenDisambiguationResult::Break(()),
                        },
                        ExprEnvironment::Condition(_) => todo!(),
                    },
                    None => {
                        DisambiguatedTokenData::BinaryOpr(regional_token_idx, BinaryOpr::Assign)
                    }
                },
                PunctuationMapped::ForAll => todo!(),
                PunctuationMapped::Exists => todo!(),
                PunctuationMapped::HeavyArrow => return TokenDisambiguationResult::Break(()),
            },
            TokenData::WordOpr(opr) => match opr {
                WordOpr::And => DisambiguatedTokenData::BinaryOpr(
                    regional_token_idx,
                    BinaryOpr::ShortCircuitLogic(BinaryShortcuitLogicOpr::And),
                ),
                WordOpr::Or => DisambiguatedTokenData::BinaryOpr(
                    regional_token_idx,
                    BinaryOpr::ShortCircuitLogic(BinaryShortcuitLogicOpr::Or),
                ),
                WordOpr::As => DisambiguatedTokenData::BinaryOpr(regional_token_idx, BinaryOpr::As),
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
        prefix_opr: PrefixOpr,
        other: DisambiguatedTokenData,
    ) -> DisambiguatedTokenData {
        match self.complete_expr() {
            Some(SynExpr::List { .. }) | Some(SynExpr::BoxColonList { .. }) | None => {
                DisambiguatedTokenData::PrefixOpr(regional_token_idx, prefix_opr)
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
                IncompleteSynExpr::Binary {
                    punctuation: BinaryOpr::ScopeResolution,
                    lopd,
                    ..
                } => match lopd.base_item_path(self.db(), &self.context().syn_expr_arena()) {
                    BaseEntityPath::None => {
                        match lopd {
                            SynExpr::Literal(_, _) => todo!(),
                            SynExpr::PrincipalEntityPath { .. } => todo!(),
                            SynExpr::AssociatedItem { .. } => todo!(),
                            SynExpr::InheritedSymbol {
                                ident,
                                regional_token_idx,
                                inherited_symbol_idx,
                                inherited_symbol_kind,
                            } => todo!(),
                            SynExpr::CurrentSymbol {
                                ident,
                                regional_token_idx,
                                current_symbol_idx,
                                current_symbol_kind,
                            } => todo!(),
                            SynExpr::FrameVarDecl {
                                regional_token_idx,
                                ident,
                                frame_var_symbol_idx,
                                current_symbol_kind,
                            } => todo!(),
                            SynExpr::SelfType(_) => todo!(),
                            SynExpr::SelfValue(_) => todo!(),
                            SynExpr::Binary {
                                lopd,
                                opr,
                                opr_regional_token_idx,
                                ropd,
                            } => todo!(),
                            SynExpr::Be {
                                src,
                                be_regional_token_idx,
                                target,
                            } => todo!(),
                            SynExpr::Prefix {
                                opr,
                                opr_regional_token_idx,
                                opd,
                            } => todo!(),
                            SynExpr::Suffix {
                                opd,
                                opr,
                                opr_regional_token_idx,
                            } => todo!(),
                            SynExpr::FunctionApplicationOrCall {
                                function,
                                generic_arguments,
                                lpar_regional_token_idx,
                                items,
                                rpar_regional_token_idx,
                            } => todo!(),
                            SynExpr::Ritchie {
                                ritchie_kind_regional_token_idx,
                                ritchie_kind,
                                lpar_token,
                                parameter_ty_items,
                                rpar_regional_token_idx,
                                light_arrow_token,
                                return_ty_expr,
                            } => todo!(),
                            SynExpr::FunctionCall {
                                function,
                                generic_arguments,
                                lpar_regional_token_idx,
                                items,
                                rpar_regional_token_idx,
                            } => todo!(),
                            SynExpr::Field {
                                owner,
                                dot_regional_token_idx,
                                ident_token,
                            } => todo!(),
                            SynExpr::MethodApplicationOrCall {
                                self_argument,
                                dot_regional_token_idx,
                                ident_token,
                                generic_arguments,
                                lpar_regional_token_idx,
                                items,
                                rpar_regional_token_idx,
                            } => todo!(),
                            SynExpr::TemplateInstantiation {
                                template,
                                generic_arguments,
                            } => todo!(),
                            SynExpr::ExplicitApplication {
                                function_expr_idx,
                                argument_expr_idx,
                            } => todo!(),
                            SynExpr::At {
                                at_regional_token_idx,
                                place_label_regional_token,
                            } => todo!(),
                            SynExpr::Unit {
                                lpar_regional_token_idx,
                                rpar_regional_token_idx,
                            } => todo!(),
                            SynExpr::Bracketed {
                                lpar_regional_token_idx,
                                item,
                                rpar_regional_token_idx,
                            } => todo!(),
                            SynExpr::NewTuple {
                                lpar_regional_token_idx,
                                items,
                                rpar_regional_token_idx,
                            } => todo!(),
                            SynExpr::IndexOrCompositionWithList {
                                owner,
                                lbox_regional_token_idx,
                                items,
                                rbox_regional_token_idx,
                            } => todo!(),
                            SynExpr::List {
                                lbox_regional_token_idx,
                                items,
                                rbox_regional_token_idx,
                            } => todo!(),
                            SynExpr::BoxColonList {
                                lbox_regional_token_idx,
                                colon_regional_token_idx,
                                items,
                                rbox_regional_token_idx,
                            } => todo!(),
                            SynExpr::Block { stmts } => todo!(),
                            SynExpr::EmptyHtmlTag {
                                empty_html_bra_idx,
                                function_ident,
                                arguments,
                                empty_html_ket,
                            } => todo!(),
                            SynExpr::Sorry { regional_token_idx } => todo!(),
                            SynExpr::Todo { regional_token_idx } => todo!(),
                            SynExpr::Unreachable { regional_token_idx } => todo!(),
                            SynExpr::Err(_) => todo!(),
                        }
                        todo!()
                    }
                    BaseEntityPath::Some(_) => {
                        // p!(
                        //     regional_token_idx,
                        //     lopd,
                        //     ident.debug(self.context.db),
                        //     self.context.path.debug(self.context.db)
                        // );
                        todo!()
                    }
                    BaseEntityPath::Uncertain { .. } => {
                        return DisambiguatedTokenData::Err(
                            OriginalSynExprError::UnresolvedSubitem {
                                regional_token_idx,
                                ident,
                            }
                            .into(),
                        )
                    }
                    BaseEntityPath::Err => todo!(),
                    BaseEntityPath::SelfType => todo!(),
                    // DisambiguatedToken::BinaryOpr(regional_token_idx, BinaryOpr::Scop),
                },
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
                // Symbol::Variable(variable_idx) => Expr::Variable {
                //     regional_token_idx,
                //     symbol_idx: variable_idx,
                // },
                Symbol::PrincipalEntity(item_path) => {
                    DisambiguatedTokenData::IdentifiableEntityPath(
                        self.parse_identifiable_item_path_expr(
                            IdentRegionalToken::new(ident, regional_token_idx).into(),
                            item_path,
                        ),
                    )
                }
                Symbol::Inherited(inherited_symbol_idx, inherited_symbol_kind) => {
                    DisambiguatedTokenData::InheritedSymbol {
                        ident,
                        regional_token_idx,
                        inherited_symbol_idx,
                        inherited_symbol_kind,
                    }
                }
                Symbol::Current(current_symbol_idx, current_symbol_kind) => {
                    DisambiguatedTokenData::CurrentSymbol {
                        ident,
                        regional_token_idx,
                        current_symbol_idx,
                        current_symbol_kind,
                    }
                } //  Expr::EntityPath(item_path),
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
    Literal(RegionalTokenIdx, Literal),
    IdentifiableEntityPath(IdentifiableEntityPathExpr),
    InheritedSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        inherited_symbol_idx: InheritedSynSymbolIdx,
        inherited_symbol_kind: InheritedSynSymbolKind,
    },
    CurrentSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        current_symbol_idx: CurrentSynSymbolIdx,
        current_symbol_kind: CurrentSynSymbolKind,
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
    Err(SynExprError),
    BinaryOpr(RegionalTokenIdx, BinaryOpr),
    PrefixOpr(RegionalTokenIdx, PrefixOpr),
    SuffixOpr(RegionalTokenIdx, SuffixOpr),
    Bra(RegionalTokenIdx, Bracket),
    Ket(RegionalTokenIdx, Bracket),
    Dot(RegionalTokenIdx),
    Comma(RegionalTokenIdx),
    Be(RegionalTokenIdx),
    ColonRightAfterLbox(RegionalTokenIdx),
    Ritchie(RegionalTokenIdx, RitchieKind),
    IncompleteKeywordArgument {
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
        eq_token: RegionalEqToken,
    },
    At(RegionalTokenIdx),
}
