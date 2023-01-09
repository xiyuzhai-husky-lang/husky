use crate::*;
use husky_token::{
    ImplToken, LeftAngleBracketToken, Token, TokenError, TokenGroupIdx, TokenStream,
};
use parsec::ParseContext;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct ImplBlock {
    kind: ImplBlockKind,
    ast_idx: AstIdx,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImplBlockKind {
    Type(TypePath),
    Todo,
    Err(ImplBlockError),
}

pub type ImplBlockArena = Arena<ImplBlock>;
pub type ImplBlockIdx = ArenaIdx<ImplBlock>;
pub type ImplBlockIdxRange = ArenaIdxRange<ImplBlock>;

impl ImplBlock {
    pub(crate) fn parse_from_token_group<'a>(
        crate_prelude: CratePrelude<'a>,
        ast_idx: AstIdx,
        mut token_stream: TokenStream<'a>,
        princiapl_entity_path_expr_arena: &mut PrincipalEntityPathExprArena,
    ) -> Self {
        token_stream.parse::<ImplToken>().unwrap().unwrap();
        if let Some(_) = token_stream.try_parse::<LeftAngleBracketToken>() {
            match ignore_implicit_parameters(&mut token_stream) {
                Ok(_) => (),
                Err(e) => todo!(),
            }
        }
        // let Ok((expr, path)) = PrincipalEntityPathExpr::parse_from_token_stream(
        //     &mut token_stream,
        //     princiapl_entity_path_expr_arena
        // ) else {
        //     todo!()
        // };
        // match path {
        //     EntityPath::ModuleItem(_) => todo!(),
        //     _ => todo!(),
        // }
        Self {
            kind: ImplBlockKind::Todo,
            ast_idx,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ImplBlockError {
    #[error("unmatched angle bras")]
    UnmatchedAngleBras,
    #[error("token error")]
    Token(#[from] TokenError),
}

pub type ImplBlockResult<T> = Result<T, ImplBlockError>;

fn ignore_implicit_parameters<'a>(token_stream: &mut TokenStream<'a>) -> ImplBlockResult<()> {
    let mut layer = 1;
    while let Some(token) = token_stream.next() {
        match token {
            Token::Punctuation(_) => todo!(),
            Token::Err(e) => return Err(e.clone().into()),
            _ => (),
        }
    }
    match layer {
        0 => Ok(()),
        _ => Err(ImplBlockError::UnmatchedAngleBras),
    }
}
