use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
#[enum_class::from_variants]
pub enum PathNameToken {
    Ident(IdentToken),
    CrateRoot(CrateToken),
    SelfMod(SelfModToken),
    Super(SuperToken),
}

// crate

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct CrateToken {
    pub(super) token_idx: TokenIdx,
}

impl CrateToken {
    pub fn new(token_idx: TokenIdx) -> Self {
        Self { token_idx }
    }

    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

// self mod

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct SelfModToken {
    token_idx: TokenIdx,
}

/// `super` super token
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct SuperToken {
    token_idx: TokenIdx,
}

impl SuperToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl SelfModToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFromStream<Context> for PathNameToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, Self::Error> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                Token::Ident(ident) => {
                    Ok(Some(PathNameToken::Ident(IdentToken { ident, token_idx })))
                }
                Token::Keyword(Keyword::Pronoun(pronoun)) => match pronoun {
                    PronounKeyword::Crate => {
                        Ok(Some(PathNameToken::CrateRoot(CrateToken { token_idx })))
                    }
                    PronounKeyword::SelfType => Ok(None),
                    PronounKeyword::SelfValue => {
                        Ok(Some(PathNameToken::SelfMod(SelfModToken { token_idx })))
                    }
                    PronounKeyword::Super => {
                        Ok(Some(PathNameToken::Super(SuperToken { token_idx })))
                    }
                },
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl PathNameToken {
    pub fn token_idx(self) -> TokenIdx {
        match self {
            PathNameToken::Ident(token) => token.token_idx(),
            PathNameToken::CrateRoot(token) => token.token_idx(),
            PathNameToken::SelfMod(token) => token.token_idx(),
            PathNameToken::Super(token) => token.token_idx(),
        }
    }

    pub fn ident_token(self) -> Option<IdentToken> {
        match self {
            PathNameToken::Ident(ident_token) => Some(ident_token),
            PathNameToken::CrateRoot(_) | PathNameToken::SelfMod(_) | PathNameToken::Super(_) => {
                None
            }
        }
    }
    pub fn ident(self) -> Option<Ident> {
        match self {
            PathNameToken::Ident(ident_token) => Some(ident_token.ident()),
            PathNameToken::CrateRoot(_) | PathNameToken::SelfMod(_) | PathNameToken::Super(_) => {
                None
            }
        }
    }
}
