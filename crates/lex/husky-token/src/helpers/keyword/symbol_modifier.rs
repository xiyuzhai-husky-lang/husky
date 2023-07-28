use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolModifierKeywordGroup {
    Mut(MutToken),
    RefMut(RefToken, MutToken),
}

impl Into<SymbolModifier> for SymbolModifierKeywordGroup {
    #[inline(always)]
    fn into(self) -> SymbolModifier {
        match self {
            SymbolModifierKeywordGroup::Mut(_) => SymbolModifier::Mut,
            SymbolModifierKeywordGroup::RefMut(_, _) => SymbolModifier::RefMut,
        }
    }
}

impl Into<Contract> for SymbolModifierKeywordGroup {
    #[inline(always)]
    fn into(self) -> Contract {
        match self {
            SymbolModifierKeywordGroup::Mut(_) => Contract::Move,
            SymbolModifierKeywordGroup::RefMut(_, _) => Contract::BorrowMut,
        }
    }
}

// todo: change this to TryParse
impl<'a, SP> parsec::TryParseOptionFromStream<SP> for SymbolModifierKeywordGroup
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
        let kw = match token {
            Token::Keyword(Keyword::Modifier(kw)) => kw,
            Token::Error(error) => Err(error)?,
            _ => return Ok(Default::default()),
        };
        match kw {
            ModifierKeyword::Mut => Ok(Some(SymbolModifierKeywordGroup::Mut(MutToken {
                token_idx,
            }))),
            ModifierKeyword::Covariant
            | ModifierKeyword::Contravariant
            | ModifierKeyword::Invariant => Ok(None),
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
