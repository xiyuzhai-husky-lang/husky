use crate::*;
use husky_term_prelude::{SymbolModifier, TermContract};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EphemSymbolModifierRegionalTokens {
    Mut(MutRegionalToken),
    Ref(RefRegionalToken),
    RefMut(RefRegionalToken, MutRegionalToken),
    Ambersand(AmbersandRegionalToken, Option<LifetimeLabelRegionalToken>),
    AmbersandMut(
        AmbersandRegionalToken,
        Option<LifetimeLabelRegionalToken>,
        MutRegionalToken,
    ),
    Le(LeRegionalToken),
    Tilde(TildeRegionalToken),
    At(AtRegionalToken, Option<PlaceLabelRegionalToken>),
}

impl Into<SymbolModifier> for EphemSymbolModifierRegionalTokens {
    #[inline(always)]
    fn into(self) -> SymbolModifier {
        match self {
            EphemSymbolModifierRegionalTokens::Mut(_) => SymbolModifier::Mut,
            EphemSymbolModifierRegionalTokens::Ref(_) => SymbolModifier::Ref,
            EphemSymbolModifierRegionalTokens::RefMut(..) => SymbolModifier::RefMut,
            EphemSymbolModifierRegionalTokens::Ambersand(_, lifetime_token) => {
                SymbolModifier::Ambersand(lifetime_token.map(|t| t.label()))
            }
            EphemSymbolModifierRegionalTokens::AmbersandMut(_, lifetime_token, _) => {
                SymbolModifier::AmbersandMut(lifetime_token.map(|t| t.label()))
            }
            EphemSymbolModifierRegionalTokens::Le(..) => SymbolModifier::Le,
            EphemSymbolModifierRegionalTokens::Tilde(..) => SymbolModifier::Tilde,
            EphemSymbolModifierRegionalTokens::At(_, _) => todo!(),
        }
    }
}

// we try to keep as similar to `Rust` as possible
impl Into<TermContract> for EphemSymbolModifierRegionalTokens {
    #[inline(always)]
    fn into(self) -> TermContract {
        match self {
            EphemSymbolModifierRegionalTokens::Mut(_) => TermContract::Move,
            EphemSymbolModifierRegionalTokens::Ref(_) => TermContract::Borrow,
            EphemSymbolModifierRegionalTokens::RefMut(..) => TermContract::BorrowMut,
            EphemSymbolModifierRegionalTokens::Ambersand(_, _) => TermContract::Borrow,
            EphemSymbolModifierRegionalTokens::AmbersandMut(_, _, _) => TermContract::BorrowMut,
            EphemSymbolModifierRegionalTokens::Le(_) => todo!(),
            EphemSymbolModifierRegionalTokens::Tilde(_) => TermContract::Leash,
            EphemSymbolModifierRegionalTokens::At(_, _) => TermContract::At,
        }
    }
}

// todo: change this to TryParse
impl<'a, SP> parsec::TryParseOptionFromStream<SP> for EphemSymbolModifierRegionalTokens
where
    SP: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SP,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream: &mut RegionalTokenStream<'a> = &mut sp.borrow_mut();
        let Some((regional_token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None);
        };
        match token {
            TokenData::Keyword(Keyword::Modifier(kw)) => match kw {
                ModifierKeyword::Mut => Ok(Some(EphemSymbolModifierRegionalTokens::Mut(
                    MutRegionalToken { regional_token_idx },
                ))),
                ModifierKeyword::Covariant
                | ModifierKeyword::Contravariant
                | ModifierKeyword::Invariant => Ok(None),
                ModifierKeyword::Ref => {
                    let ref_regional_token = RefRegionalToken { regional_token_idx };
                    if let Some(_) = token_stream.try_parse_option::<MutRegionalToken>()? {
                        Ok(Some(EphemSymbolModifierRegionalTokens::Ref(
                            ref_regional_token,
                        )))
                    } else {
                        Ok(Some(EphemSymbolModifierRegionalTokens::Ref(
                            ref_regional_token,
                        )))
                    }
                }
                ModifierKeyword::Le => todo!(),
            },
            TokenData::Punctuation(Punctuation::AMBERSAND) => {
                let lifetime_token =
                    token_stream.try_parse_option::<LifetimeLabelRegionalToken>()?;
                if let Some(mut_token) = token_stream.try_parse_option::<MutRegionalToken>()? {
                    Ok(Some(EphemSymbolModifierRegionalTokens::AmbersandMut(
                        AmbersandRegionalToken(regional_token_idx),
                        lifetime_token,
                        mut_token,
                    )))
                } else {
                    Ok(Some(EphemSymbolModifierRegionalTokens::Ambersand(
                        AmbersandRegionalToken(regional_token_idx),
                        lifetime_token,
                    )))
                }
            }
            TokenData::Punctuation(Punctuation::TILDE) => Ok(Some(
                EphemSymbolModifierRegionalTokens::Tilde(TildeRegionalToken(regional_token_idx)),
            )),
            TokenData::Punctuation(Punctuation::AT) => {
                Ok(Some(EphemSymbolModifierRegionalTokens::At(
                    AtRegionalToken(regional_token_idx),
                    token_stream.try_parse_option()?,
                )))
            }
            TokenData::Error(error) => Err(error)?,
            _ => Ok(None),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct MutRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl MutRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for MutRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Modifier(ModifierKeyword::Mut)) => {
                    Ok(Some(MutRegionalToken { regional_token_idx }))
                }
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::Ident(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

/// `ref`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct RefRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl RefRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

/// `le`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct LeRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl LeRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}
