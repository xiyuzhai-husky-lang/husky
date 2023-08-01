use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolModifierTokenGroup {
    Mut(MutToken),
    RefMut(RefToken, Option<LifetimeToken>, MutToken),
    Ambersand(AmbersandToken, Option<LifetimeToken>),
    AmbersandMut(AmbersandToken, Option<LifetimeToken>, MutToken),
    Le(LeToken),
    Tilde(TildeToken),
}

impl Into<SymbolModifier> for SymbolModifierTokenGroup {
    #[inline(always)]
    fn into(self) -> SymbolModifier {
        match self {
            SymbolModifierTokenGroup::Mut(_) => SymbolModifier::Mut,
            SymbolModifierTokenGroup::RefMut(..) => SymbolModifier::RefMut,
            SymbolModifierTokenGroup::Ambersand(_, lifetime_token) => {
                SymbolModifier::Ambersand(lifetime_token.map(|t| t.label()))
            }
            SymbolModifierTokenGroup::AmbersandMut(_, lifetime_token, _) => {
                SymbolModifier::AmbersandMut(lifetime_token.map(|t| t.label()))
            }
            SymbolModifierTokenGroup::Le(..) => SymbolModifier::Le,
            SymbolModifierTokenGroup::Tilde(..) => SymbolModifier::Tilde,
        }
    }
}

impl Into<Contract> for SymbolModifierTokenGroup {
    #[inline(always)]
    fn into(self) -> Contract {
        match self {
            SymbolModifierTokenGroup::Mut(_) => Contract::Move,
            SymbolModifierTokenGroup::RefMut(..) => Contract::BorrowMut,
            _ => todo!(),
        }
    }
}

// todo: change this to TryParse
impl<'a, SP> parsec::TryParseOptionFromStream<SP> for SymbolModifierTokenGroup
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
                ModifierKeyword::Mut => {
                    Ok(Some(SymbolModifierTokenGroup::Mut(MutToken { token_idx })))
                }
                ModifierKeyword::Covariant
                | ModifierKeyword::Contravariant
                | ModifierKeyword::Invariant => Ok(None),
                ModifierKeyword::Ref => todo!(),
                ModifierKeyword::Le => todo!(),
            },
            Token::Punctuation(Punctuation::AMBERSAND) => {
                Ok(Some(SymbolModifierTokenGroup::Ambersand(todo!(), todo!())))
            }
            Token::Punctuation(Punctuation::TILDE) => todo!(),
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
