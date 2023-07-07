use super::*;
use husky_entity_taxonomy::{EntityKind, FugitiveKind, ModuleItemKind};
use husky_print_utils::p;
use husky_token::Punctuation;
use salsa::DebugWithDb;
use std::ops::ControlFlow;

pub type TokenDisambiguationResult<T> = ControlFlow<(), T>;

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn disambiguate_token(
        &mut self,
        token_idx: TokenIdx,
        token: Token,
    ) -> TokenDisambiguationResult<DisambiguatedToken> {
        TokenDisambiguationResult::Continue(match token {
            Token::Keyword(keyword) => match keyword {
                Keyword::Connection(keyword) => match keyword {
                    ConnectionKeyword::For
                    | ConnectionKeyword::Where
                    | ConnectionKeyword::Extends => return TokenDisambiguationResult::Break(()),
                },
                Keyword::Pronoun(pronoun) => match pronoun {
                    PronounKeyword::Crate => {
                        let crate_root_path = self.parser.crate_root_path;
                        DisambiguatedToken::AtomicExpr(self.parse_principal_entity_path_expr(
                            CrateToken::new(token_idx).into(),
                            crate_root_path.into(),
                        ))
                    }
                    PronounKeyword::SelfType => match self.allow_self_ty() {
                        AllowSelfType::True => {
                            DisambiguatedToken::AtomicExpr(Expr::SelfType(token_idx))
                        }
                        AllowSelfType::False => DisambiguatedToken::AtomicExpr(Expr::Err(
                            OriginalExprError::SelfTypeNotAllowed(token_idx).into(),
                        )),
                    },
                    PronounKeyword::SelfValue => match self.peek() {
                        Some(Token::Punctuation(Punctuation::COLON_COLON)) => {
                            todo!()
                        }
                        _ => match self.allow_self_value() {
                            AllowSelfValue::True => {
                                DisambiguatedToken::AtomicExpr(Expr::SelfValue(token_idx))
                            }
                            AllowSelfValue::False => DisambiguatedToken::AtomicExpr(Expr::Err(
                                OriginalExprError::SelfValueNotAllowed(token_idx).into(),
                            )),
                        },
                    },
                    PronounKeyword::Super => todo!(),
                },
                Keyword::Fugitive(FugitiveKeyword::Fn) => {
                    DisambiguatedToken::Ritchie(token_idx, RitchieKind::FnType)
                }
                _ => DisambiguatedToken::AtomicExpr(Expr::Err(
                    OriginalExprError::UnexpectedKeyword(token_idx).into(),
                )),
            },
            Token::Ident(ident) => match self.top_expr() {
                TopExprRef::Incomplete(
                    IncompleteExpr::CommaList {
                        opr:
                            IncompleteCommaListOpr::FunctionApplicationOrCall { .. }
                            | IncompleteCommaListOpr::MethodApplicationOrCall { .. },
                        ..
                    }
                    | IncompleteExpr::CallList { .. },
                ) => match self.try_parse_err_as_none::<EqToken>() {
                    Some(eq_token) => DisambiguatedToken::IncompleteKeywordArgument {
                        token_idx,
                        ident,
                        eq_token,
                    },
                    None => self.resolve_ident(token_idx, ident),
                },
                _ => self.resolve_ident(token_idx, ident),
            },
            Token::Label(_) => todo!(),
            Token::Punctuation(punct) => match punct.mapped() {
                PunctuationMapped::Binary(binary) => {
                    DisambiguatedToken::BinaryOpr(token_idx, binary)
                }
                PunctuationMapped::Bra(bra) => DisambiguatedToken::Bra(token_idx, bra),
                PunctuationMapped::Ket(ket) => match self.last_bra() {
                    Some(bra) => {
                        if bra != ket {
                            todo!()
                        }
                        DisambiguatedToken::Ket(token_idx, ket)
                    }
                    None => return TokenDisambiguationResult::Break(()),
                },
                PunctuationMapped::Suffix(suffix) => {
                    DisambiguatedToken::SuffixOpr(token_idx, suffix)
                }
                PunctuationMapped::LaOrLt => match self.top_expr() {
                    TopExprRef::Incomplete(_) => todo!(),
                    TopExprRef::Finished(expr) => {
                        match expr.base_entity_path(self.db(), &self.parser.expr_arena) {
                            BaseEntityPath::Uncertain {
                                inclination: BaseEntityPathInclination::TypeOrVariant,
                            } => DisambiguatedToken::Bra(token_idx, Bracket::TemplateAngle),
                            BaseEntityPath::Some(entity_path) => {
                                match entity_path.entity_kind(self.db()) {
                                    EntityKind::Module => todo!(),
                                    EntityKind::ModuleItem {
                                        module_item_kind,
                                        connection,
                                    } => match module_item_kind {
                                        ModuleItemKind::Fugitive(FugitiveKind::Val) => {
                                            DisambiguatedToken::BinaryOpr(
                                                token_idx,
                                                BinaryComparisonOpr::Less.into(),
                                            )
                                        }
                                        ModuleItemKind::Type(_)
                                        | ModuleItemKind::Fugitive(_)
                                        | ModuleItemKind::Trait => DisambiguatedToken::Bra(
                                            token_idx,
                                            Bracket::TemplateAngle,
                                        ),
                                    },
                                    EntityKind::AssociatedItem {
                                        associated_item_kind,
                                    } => todo!(),
                                    EntityKind::TypeVariant => todo!(),
                                    EntityKind::Trait => todo!(),
                                    EntityKind::ImplBlock => todo!(),
                                }
                            }
                            _ => DisambiguatedToken::BinaryOpr(
                                token_idx,
                                BinaryOpr::Comparison(BinaryComparisonOpr::Less),
                            ),
                        }
                    }
                    TopExprRef::None => DisambiguatedToken::Bra(token_idx, Bracket::HtmlAngle),
                },
                PunctuationMapped::ColonColonLa => todo!(),
                PunctuationMapped::RaOrGt => match (self.last_bra(), self.env_bra()) {
                    (Some(Bracket::TemplateAngle), _) => {
                        DisambiguatedToken::Ket(token_idx, Bracket::TemplateAngle)
                    }
                    (None, Some(Bracket::TemplateAngle)) => {
                        return TokenDisambiguationResult::Break(())
                    }
                    _ => DisambiguatedToken::BinaryOpr(
                        token_idx,
                        BinaryComparisonOpr::Greater.into(),
                    ),
                },
                PunctuationMapped::Sheba => DisambiguatedToken::AtomicExpr(Expr::Err(
                    OriginalExprError::UnexpectedSheba(token_idx).into(),
                )),
                PunctuationMapped::Shr => {
                    DisambiguatedToken::BinaryOpr(token_idx, BinaryOpr::Shift(BinaryShiftOpr::Shr))
                }
                PunctuationMapped::DeriveAssign => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Minus => {
                    DisambiguatedToken::PrefixOpr(token_idx, PrefixOpr::Minus)
                }
                PunctuationMapped::DoubleVertical => todo!(),
                PunctuationMapped::Tilde => {
                    DisambiguatedToken::PrefixOpr(token_idx, PrefixOpr::Tilde)
                }
                PunctuationMapped::Dot => DisambiguatedToken::Dot(token_idx),
                PunctuationMapped::Colon => match self.last_incomplete_expr() {
                    Some(IncompleteExpr::CommaList {
                        opr: IncompleteCommaListOpr::BoxList { .. },
                        items,
                        ..
                    }) => {
                        if items.len() == 0 && self.complete_expr().is_none() {
                            DisambiguatedToken::ColonRightAfterLBox(token_idx)
                        } else {
                            match self.token_stream.is_empty() {
                                true => return TokenDisambiguationResult::Break(()),
                                false => DisambiguatedToken::BinaryOpr(token_idx, BinaryOpr::Ins),
                            }
                        }
                    }
                    _ => match self.peek() {
                        // not end of token group
                        Some(_) => DisambiguatedToken::BinaryOpr(token_idx, BinaryOpr::Ins),
                        // end of token group
                        None => return TokenDisambiguationResult::Break(()),
                    },
                },
                PunctuationMapped::Comma => {
                    self.reduce(Precedence::ListItem);
                    match self.last_incomplete_expr() {
                        Some(expr) => match expr {
                            IncompleteExpr::CommaList { .. } | IncompleteExpr::CallList { .. } => {
                                DisambiguatedToken::Comma(token_idx)
                            }
                            _ => return TokenDisambiguationResult::Break(()),
                        },
                        None => return TokenDisambiguationResult::Break(()),
                    }
                }
                PunctuationMapped::Vertical => match self.last_incomplete_expr() {
                    Some(IncompleteExpr::CommaList {
                        bra: Bracket::Lambda,
                        ..
                    }) => DisambiguatedToken::Ket(token_idx, Bracket::Lambda),
                    _ => match self.complete_expr().is_some() {
                        true => DisambiguatedToken::BinaryOpr(
                            token_idx,
                            BinaryOpr::Closed(BinaryClosedOpr::BitOr),
                        ),
                        false => DisambiguatedToken::Bra(token_idx, Bracket::Lambda),
                    },
                },
                PunctuationMapped::DoubleExclamation => todo!(),
                PunctuationMapped::Semicolon => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::EmptyHtmlKet => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::At => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::AtEq => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Exclamation => self.resolve_prefix_or_other(
                    token_idx,
                    PrefixOpr::Not,
                    DisambiguatedToken::SuffixOpr(token_idx, SuffixOpr::UnwrapOrComposeWithNot),
                ),
                PunctuationMapped::Question => self.resolve_prefix_or_other(
                    token_idx,
                    PrefixOpr::Option,
                    DisambiguatedToken::SuffixOpr(token_idx, SuffixOpr::UnveilOrComposeWithOption),
                ),
                PunctuationMapped::Pound => return TokenDisambiguationResult::Break(()),
                PunctuationMapped::Ambersand => self.resolve_prefix_or_other(
                    token_idx,
                    PrefixOpr::Ref,
                    DisambiguatedToken::BinaryOpr(
                        token_idx,
                        BinaryOpr::Closed(BinaryClosedOpr::BitOr),
                    ),
                ),
                PunctuationMapped::DotDot => todo!(),
                PunctuationMapped::DotDotDot => todo!(),
                PunctuationMapped::Star => DisambiguatedToken::BinaryOpr(
                    token_idx,
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
                    None => DisambiguatedToken::BinaryOpr(token_idx, BinaryOpr::Assign),
                },
                PunctuationMapped::ForAll => todo!(),
                PunctuationMapped::Exists => todo!(),
                PunctuationMapped::HeavyArrow => todo!(),
            },
            Token::WordOpr(opr) => match opr {
                WordOpr::And => DisambiguatedToken::BinaryOpr(
                    token_idx,
                    BinaryOpr::ShortCircuitLogic(BinaryShortcuitLogicOpr::And),
                ),
                WordOpr::Or => DisambiguatedToken::BinaryOpr(
                    token_idx,
                    BinaryOpr::ShortCircuitLogic(BinaryShortcuitLogicOpr::Or),
                ),
                WordOpr::As => DisambiguatedToken::BinaryOpr(token_idx, BinaryOpr::As),
                WordOpr::Be => DisambiguatedToken::Be(token_idx),
            },
            Token::Literal(literal) => {
                DisambiguatedToken::AtomicExpr(Expr::Literal(token_idx, literal))
            }
            Token::Error(error) => {
                DisambiguatedToken::AtomicExpr(Expr::Err(DerivedExprError::Token(error).into()))
            }
        })
    }

    fn resolve_prefix_or_other(
        &mut self,
        token_idx: TokenIdx,
        prefix_opr: PrefixOpr,
        other: DisambiguatedToken,
    ) -> DisambiguatedToken {
        match self.complete_expr() {
            Some(Expr::List { .. }) | Some(Expr::BoxColonList { .. }) | None => {
                DisambiguatedToken::PrefixOpr(token_idx, prefix_opr)
            }
            Some(_) => other,
        }
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    fn resolve_ident(&mut self, token_idx: TokenIdx, ident: Ident) -> DisambiguatedToken {
        if let Some(opn) = self.last_incomplete_expr() {
            match opn {
                IncompleteExpr::Binary {
                    punctuation: BinaryOpr::ScopeResolution,
                    lopd,
                    ..
                } => match lopd.base_entity_path(self.db(), &self.parser.expr_arena) {
                    BaseEntityPath::None => todo!(),
                    BaseEntityPath::Some(_) => {
                        p!(
                            lopd,
                            ident.debug(self.parser.db),
                            self.parser.path.debug(self.parser.db)
                        );
                        todo!()
                    }
                    BaseEntityPath::Uncertain { .. } => {
                        return DisambiguatedToken::AtomicExpr(Expr::Err(
                            OriginalExprError::UnresolvedSubentity { token_idx, ident }.into(),
                        ))
                    }
                    BaseEntityPath::Err => todo!(),
                },
                _ => (),
            }
        }
        DisambiguatedToken::AtomicExpr(
            match self.parser.symbol_context.resolve_ident(
                self.db(),
                self.parser.module_path,
                token_idx,
                ident,
            ) {
                Some(symbol) => match symbol {
                    // Symbol::Variable(variable_idx) => Expr::Variable {
                    //     token_idx,
                    //     symbol_idx: variable_idx,
                    // },
                    Symbol::PrincipalEntity(entity_path) => self.parse_principal_entity_path_expr(
                        IdentToken::new(ident, token_idx).into(),
                        entity_path,
                    ),
                    Symbol::Inherited(inherited_symbol_idx, inherited_symbol_kind) => {
                        Expr::InheritedSymbol {
                            ident,
                            token_idx,
                            inherited_symbol_idx,
                            inherited_symbol_kind,
                        }
                    }
                    Symbol::Local(current_symbol_idx, current_symbol_kind) => Expr::CurrentSymbol {
                        ident,
                        token_idx,
                        current_symbol_idx,
                        current_symbol_kind,
                    },
                    //  Expr::EntityPath(entity_path),
                },
                None => Expr::Err(OriginalExprError::UnrecognizedIdent { token_idx, ident }.into()),
            },
        )
    }
}

#[derive(Debug)]
pub(crate) enum DisambiguatedToken {
    AtomicExpr(Expr),
    BinaryOpr(TokenIdx, BinaryOpr),
    PrefixOpr(TokenIdx, PrefixOpr),
    SuffixOpr(TokenIdx, SuffixOpr),
    Bra(TokenIdx, Bracket),
    Ket(TokenIdx, Bracket),
    Dot(TokenIdx),
    Comma(TokenIdx),
    Be(TokenIdx),
    ColonRightAfterLBox(TokenIdx),
    Ritchie(TokenIdx, RitchieKind),
    IncompleteKeywordArgument {
        token_idx: TokenIdx,
        ident: Ident,
        eq_token: EqToken,
    },
}
