use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolModifierKeywordGroup {
    Default,
    Mut(MutToken),
    RefMut(RefToken, MutToken),
}

impl SymbolModifierKeywordGroup {
    pub fn symbol_modifier(self) -> SymbolModifier {
        match self {
            SymbolModifierKeywordGroup::Default => SymbolModifier::Const,
            SymbolModifierKeywordGroup::Mut(_) => todo!(),
            SymbolModifierKeywordGroup::RefMut(_, _) => todo!(),
        }
    }

    pub fn contract(self) -> Contract {
        match self {
            SymbolModifierKeywordGroup::Mut(_) => Contract::Move,
            SymbolModifierKeywordGroup::RefMut(_, _) => Contract::BorrowMut,
            SymbolModifierKeywordGroup::Default => Contract::Pure,
        }
    }
}

impl Default for SymbolModifierKeywordGroup {
    fn default() -> Self {
        SymbolModifierKeywordGroup::Default
    }
}

// todo: change this to TryParse
impl<'a, Context> parsec::TryParseFromStream<Context> for SymbolModifierKeywordGroup
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_from_stream(ctx: &mut Context) -> TokenResult<Self> {
        let token_stream: &mut TokenStream<'a> = &mut ctx.borrow_mut();
        let Some((token_idx, token)) = token_stream.next_indexed() else {
            return Ok(Default::default())
        };
        let kw = match token {
            Token::Keyword(Keyword::Modifier(kw)) => kw,
            Token::Error(error) => Err(error)?,
            _ => return Ok(Default::default()),
        };
        match kw {
            ModifierKeyword::Mut => Ok(SymbolModifierKeywordGroup::Mut(MutToken { token_idx })),
            ModifierKeyword::Covariant
            | ModifierKeyword::Contravariant
            | ModifierKeyword::Invariant => Ok(Default::default()),
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

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for MutToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
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
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct RefToken {
    token_idx: TokenIdx,
}

impl RefToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}
