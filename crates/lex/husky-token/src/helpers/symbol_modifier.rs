use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EphemSymbolModifierTokenVerse {
    Mut(MutToken),
    RefMut(RefToken, Option<LifetimeToken>, MutToken),
    Ambersand(AmbersandToken, Option<LifetimeToken>),
    AmbersandMut(AmbersandToken, Option<LifetimeToken>, MutToken),
    Le(LeToken),
    Tilde(TildeToken),
}

impl Into<SvarModifier> for EphemSymbolModifierTokenVerse {
    #[inline(always)]
    fn into(self) -> SvarModifier {
        match self {
            EphemSymbolModifierTokenVerse::Mut(_) => SvarModifier::Mut,
            EphemSymbolModifierTokenVerse::RefMut(..) => SvarModifier::RefMut,
            EphemSymbolModifierTokenVerse::Ambersand(_, lifetime_token) => {
                SvarModifier::Ambersand(lifetime_token.map(|t| t.label()))
            }
            EphemSymbolModifierTokenVerse::AmbersandMut(_, lifetime_token, _) => {
                SvarModifier::AmbersandMut(lifetime_token.map(|t| t.label()))
            }
            EphemSymbolModifierTokenVerse::Le(..) => SvarModifier::Le,
            EphemSymbolModifierTokenVerse::Tilde(..) => SvarModifier::Tilde,
        }
    }
}

impl Into<Contract> for EphemSymbolModifierTokenVerse {
    #[inline(always)]
    fn into(self) -> Contract {
        match self {
            EphemSymbolModifierTokenVerse::Mut(_) => Contract::Move,
            EphemSymbolModifierTokenVerse::RefMut(..) => Contract::BorrowMut,
            EphemSymbolModifierTokenVerse::Ambersand(_, _) => Contract::Borrow,
            EphemSymbolModifierTokenVerse::AmbersandMut(_, _, _) => Contract::BorrowMut,
            EphemSymbolModifierTokenVerse::Le(_) => Contract::Leash,
            EphemSymbolModifierTokenVerse::Tilde(_) => Contract::Leash,
        }
    }
}

// todo: change this to TryParse
impl<'a, SP> parsec::TryParseOptionFromStream<SP> for EphemSymbolModifierTokenVerse
where
    SP: TokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SP,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream: &mut TokenStream<'a> = &mut sp.borrow_mut();
        let Some((token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None);
        };
        match token {
            TokenData::Keyword(Keyword::Modifier(kw)) => match kw {
                ModifierKeyword::Mut => Ok(Some(EphemSymbolModifierTokenVerse::Mut(MutToken {
                    token_idx,
                }))),
                ModifierKeyword::Covariant
                | ModifierKeyword::Contravariant
                | ModifierKeyword::Invariant => Ok(None),
                ModifierKeyword::Ref => todo!(),
                ModifierKeyword::Le => todo!(),
            },
            TokenData::Punctuation(Punctuation::AMBERSAND) => {
                let lifetime_token = token_stream.try_parse_option::<LifetimeToken>()?;
                if let Some(mut_token) = token_stream.try_parse_option::<MutToken>()? {
                    Ok(Some(EphemSymbolModifierTokenVerse::AmbersandMut(
                        AmbersandToken(token_idx),
                        lifetime_token,
                        mut_token,
                    )))
                } else {
                    Ok(Some(EphemSymbolModifierTokenVerse::Ambersand(
                        AmbersandToken(token_idx),
                        lifetime_token,
                    )))
                }
            }
            TokenData::Punctuation(Punctuation::TILDE) => Ok(Some(
                EphemSymbolModifierTokenVerse::Tilde(TildeToken(token_idx)),
            )),
            TokenData::Error(error) => Err(error)?,
            _ => Ok(None),
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Modifier(ModifierKeyword::Mut)) => {
                    Ok(Some(MutToken { token_idx }))
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
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RefToken {
    token_idx: TokenIdx,
}

impl RefToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

/// `le`
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LeToken {
    token_idx: TokenIdx,
}

impl LeToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}
