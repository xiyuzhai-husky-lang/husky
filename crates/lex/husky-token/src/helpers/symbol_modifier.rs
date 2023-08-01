use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EphemSymbolModifierTokenGroup {
    Mut(MutToken),
    RefMut(RefToken, Option<LifetimeToken>, MutToken),
    Ambersand(AmbersandToken, Option<LifetimeToken>),
    AmbersandMut(AmbersandToken, Option<LifetimeToken>, MutToken),
    Le(LeToken),
    Tilde(TildeToken),
}

impl Into<EphemSymbolModifier> for EphemSymbolModifierTokenGroup {
    #[inline(always)]
    fn into(self) -> EphemSymbolModifier {
        match self {
            EphemSymbolModifierTokenGroup::Mut(_) => EphemSymbolModifier::Mut,
            EphemSymbolModifierTokenGroup::RefMut(..) => EphemSymbolModifier::RefMut,
            EphemSymbolModifierTokenGroup::Ambersand(_, lifetime_token) => {
                EphemSymbolModifier::Ambersand(lifetime_token.map(|t| t.label()))
            }
            EphemSymbolModifierTokenGroup::AmbersandMut(_, lifetime_token, _) => {
                EphemSymbolModifier::AmbersandMut(lifetime_token.map(|t| t.label()))
            }
            EphemSymbolModifierTokenGroup::Le(..) => EphemSymbolModifier::Le,
            EphemSymbolModifierTokenGroup::Tilde(..) => EphemSymbolModifier::Tilde,
        }
    }
}

impl Into<Contract> for EphemSymbolModifierTokenGroup {
    #[inline(always)]
    fn into(self) -> Contract {
        match self {
            EphemSymbolModifierTokenGroup::Mut(_) => Contract::Move,
            EphemSymbolModifierTokenGroup::RefMut(..) => Contract::BorrowMut,
            EphemSymbolModifierTokenGroup::Ambersand(_, _) => Contract::Borrow,
            EphemSymbolModifierTokenGroup::AmbersandMut(_, _, _) => Contract::BorrowMut,
            EphemSymbolModifierTokenGroup::Le(_) => todo!(),
            EphemSymbolModifierTokenGroup::Tilde(_) => Contract::Leash,
        }
    }
}

// todo: change this to TryParse
impl<'a, SP> parsec::TryParseOptionFromStream<SP> for EphemSymbolModifierTokenGroup
where
    SP: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SP,
    ) -> TokenResult<Option<Self>> {
        let token_stream: &mut TokenStream<'a> = &mut sp.borrow_mut();
        let Some((token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None);
        };
        match token {
            Token::Keyword(Keyword::Modifier(kw)) => match kw {
                ModifierKeyword::Mut => Ok(Some(EphemSymbolModifierTokenGroup::Mut(MutToken {
                    token_idx,
                }))),
                ModifierKeyword::Covariant
                | ModifierKeyword::Contravariant
                | ModifierKeyword::Invariant => Ok(None),
                ModifierKeyword::Ref => todo!(),
                ModifierKeyword::Le => todo!(),
            },
            Token::Punctuation(Punctuation::AMBERSAND) => {
                let lifetime_token = token_stream.try_parse_option::<LifetimeToken>()?;
                if let Some(mut_token) = token_stream.try_parse_option::<MutToken>()? {
                    Ok(Some(EphemSymbolModifierTokenGroup::AmbersandMut(
                        AmbersandToken(token_idx),
                        lifetime_token,
                        mut_token,
                    )))
                } else {
                    Ok(Some(EphemSymbolModifierTokenGroup::Ambersand(
                        AmbersandToken(token_idx),
                        lifetime_token,
                    )))
                }
            }
            Token::Punctuation(Punctuation::TILDE) => Ok(Some(
                EphemSymbolModifierTokenGroup::Tilde(TildeToken(token_idx)),
            )),
            Token::Error(error) => Err(error)?,
            _ => Ok(None),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct MutToken {
    token_idx: TokenIdx,
}

impl MutToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for MutToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Modifier(ModifierKeyword::Mut)) => {
                    Ok(Some(MutToken { token_idx }))
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
pub struct RefToken {
    token_idx: TokenIdx,
}

impl RefToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

/// `le`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct LeToken {
    token_idx: TokenIdx,
}

impl LeToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}
