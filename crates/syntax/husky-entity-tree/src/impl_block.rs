use crate::*;
use husky_token::{
    ImplToken, LeftAngleBracketToken, Token, TokenError, TokenGroupIdx, TokenStream,
};
use husky_word::IdentPairMap;
use parsec::ParseContext;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct ImplBlock {
    kind: ImplBlockVariant,
    ast_idx: AstIdx,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImplBlockVariant {
    Type(TypePath),
    Todo,
    Err(ImplBlockError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplBlockKind {
    Type(TypePath),
    Todo,
    Err,
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
            kind: ImplBlockVariant::Todo,
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

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_impl_blocks(
    db: &dyn EntityTreeDb,
    ty: TypePath,
) -> EntityTreeCrateBundleResult<Vec<ImplBlockIdx>> {
    let crate_path = ty.module_path(db).crate_path(db);
    let entity_tree_crate_bundle = db.entity_tree_crate_bundle(crate_path)?;
    Ok(entity_tree_crate_bundle
        .impl_block_indexed_iter()
        .filter_map(|(idx, impl_block)| {
            (impl_block.kind == ImplBlockVariant::Type(ty)).then_some(idx)
        })
        .collect())
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_associated_items(
    db: &dyn EntityTreeDb,
    ty: TypePath,
) -> EntityTreeCrateBundleResult<IdentPairMap<AssociatedItemIdx>> {
    let crate_path = ty.module_path(db).crate_path(db);
    let entity_tree_crate_bundle = db.entity_tree_crate_bundle(crate_path)?;
    Ok(entity_tree_crate_bundle
        .associated_item_indexed_iter()
        .filter_map(|(idx, associated_item)| {
            (associated_item.impl_block_kind() == ImplBlockKind::Type(ty))
                .then_some((associated_item.ident(), idx))
        })
        .collect())
}
