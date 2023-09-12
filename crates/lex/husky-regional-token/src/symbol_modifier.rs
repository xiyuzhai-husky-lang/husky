use crate::*;
use husky_term_prelude::{Contract, SymbolModifier};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EphemSymbolModifierRegionalTokenGroup {
    Mut(MutRegionalToken),
    RefMut(
        RefRegionalToken,
        Option<LifetimeRegionalToken>,
        MutRegionalToken,
    ),
    Ambersand(AmbersandRegionalToken, Option<LifetimeRegionalToken>),
    AmbersandMut(
        AmbersandRegionalToken,
        Option<LifetimeRegionalToken>,
        MutRegionalToken,
    ),
    Le(LeRegionalToken),
    Tilde(TildeRegionalToken),
}

impl Into<SymbolModifier> for EphemSymbolModifierRegionalTokenGroup {
    #[inline(always)]
    fn into(self) -> SymbolModifier {
        match self {
            EphemSymbolModifierRegionalTokenGroup::Mut(_) => SymbolModifier::Mut,
            EphemSymbolModifierRegionalTokenGroup::RefMut(..) => SymbolModifier::RefMut,
            EphemSymbolModifierRegionalTokenGroup::Ambersand(_, lifetime_token) => {
                SymbolModifier::Ambersand(lifetime_token.map(|t| t.label()))
            }
            EphemSymbolModifierRegionalTokenGroup::AmbersandMut(_, lifetime_token, _) => {
                SymbolModifier::AmbersandMut(lifetime_token.map(|t| t.label()))
            }
            EphemSymbolModifierRegionalTokenGroup::Le(..) => SymbolModifier::Le,
            EphemSymbolModifierRegionalTokenGroup::Tilde(..) => SymbolModifier::Tilde,
        }
    }
}

impl Into<Contract> for EphemSymbolModifierRegionalTokenGroup {
    #[inline(always)]
    fn into(self) -> Contract {
        match self {
            EphemSymbolModifierRegionalTokenGroup::Mut(_) => Contract::Move,
            EphemSymbolModifierRegionalTokenGroup::RefMut(..) => Contract::BorrowMut,
            EphemSymbolModifierRegionalTokenGroup::Ambersand(_, _) => Contract::Borrow,
            EphemSymbolModifierRegionalTokenGroup::AmbersandMut(_, _, _) => Contract::BorrowMut,
            EphemSymbolModifierRegionalTokenGroup::Le(_) => todo!(),
            EphemSymbolModifierRegionalTokenGroup::Tilde(_) => Contract::Leash,
        }
    }
}

// todo: change this to TryParse
impl<'a, SP> parsec::TryParseOptionFromStream<SP> for EphemSymbolModifierRegionalTokenGroup
where
    SP: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SP,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream: &mut RegionalTokenStream<'a> = &mut sp.borrow_mut();
        let Some((token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None);
        };
        match token {
            Token::Keyword(Keyword::Modifier(kw)) => match kw {
                ModifierKeyword::Mut => Ok(Some(EphemSymbolModifierRegionalTokenGroup::Mut(
                    MutRegionalToken { token_idx },
                ))),
                ModifierKeyword::Covariant
                | ModifierKeyword::Contravariant
                | ModifierKeyword::Invariant => Ok(None),
                ModifierKeyword::Ref => todo!(),
                ModifierKeyword::Le => todo!(),
            },
            Token::Punctuation(Punctuation::AMBERSAND) => {
                let lifetime_token = token_stream.try_parse_option::<LifetimeRegionalToken>()?;
                if let Some(mut_token) = token_stream.try_parse_option::<MutRegionalToken>()? {
                    Ok(Some(EphemSymbolModifierRegionalTokenGroup::AmbersandMut(
                        AmbersandRegionalToken(token_idx),
                        lifetime_token,
                        mut_token,
                    )))
                } else {
                    Ok(Some(EphemSymbolModifierRegionalTokenGroup::Ambersand(
                        AmbersandRegionalToken(token_idx),
                        lifetime_token,
                    )))
                }
            }
            Token::Punctuation(Punctuation::TILDE) => Ok(Some(
                EphemSymbolModifierRegionalTokenGroup::Tilde(TildeRegionalToken(token_idx)),
            )),
            Token::Error(error) => Err(error)?,
            _ => Ok(None),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct MutRegionalToken {
    token_idx: RegionalTokenIdx,
}

impl MutRegionalToken {
    pub fn token_idx(&self) -> RegionalTokenIdx {
        self.token_idx
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
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Modifier(ModifierKeyword::Mut)) => {
                    Ok(Some(MutRegionalToken { token_idx }))
                }
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

/// `ref`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct RefRegionalToken {
    token_idx: RegionalTokenIdx,
}

impl RefRegionalToken {
    pub fn token_idx(&self) -> RegionalTokenIdx {
        self.token_idx
    }
}

/// `le`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct LeRegionalToken {
    token_idx: RegionalTokenIdx,
}

impl LeRegionalToken {
    pub fn token_idx(&self) -> RegionalTokenIdx {
        self.token_idx
    }
}
