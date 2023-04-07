use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PatternSymbolModifierKeywordGroup {
    Mut(MutToken),
    RefMut(RefToken, MutToken),
}

impl<'a, Context> parsec::ParseFromStream<Context> for PatternSymbolModifierKeywordGroup
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        let token_stream: &mut TokenStream<'a> = &mut ctx.borrow_mut();
        let Some((token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None)
        };
        let kw = match token {
            Token::Keyword(Keyword::Modifier(kw)) => kw,
            Token::Error(error) => Err(error)?,
            _ => return Ok(None),
        };
        match kw {
            ModifierKeyword::Mut => Ok(Some(PatternSymbolModifierKeywordGroup::Mut(MutToken {
                token_idx,
            }))),
            ModifierKeyword::Covariant
            | ModifierKeyword::Contravariant
            | ModifierKeyword::Invariant => Ok(None),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct MutToken {
    token_idx: TokenIdx,
}

impl MutToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFromStream<Context> for MutToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct RefToken {
    token_idx: TokenIdx,
}

impl RefToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}
