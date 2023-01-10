use crate::*;
use husky_token::{
    ImplToken, LeftAngleBracketToken, Token, TokenError, TokenGroupIdx, TokenStream,
};
use husky_word::IdentPairMap;
use parsec::ParseContext;
use thiserror::Error;

#[salsa::tracked(jar = EntityTreeJar)]
pub struct ImplBlock {
    #[id]
    pub id: ImplBlockId,
    pub ast_idx: AstIdx,
    pub body: AstIdxRange,
    #[return_ref]
    pub variant: ImplBlockVariant,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImplBlockVariant {
    Type { ty: TypePath },
    TypeAsTrait { ty: TypePath, trai: TraitPath },
    Err(ImplBlockError),
}
impl ImplBlockVariant {
    pub fn kind(&self) -> ImplBlockKind {
        match self {
            ImplBlockVariant::Type { ty } => ImplBlockKind::Type { ty: *ty },
            ImplBlockVariant::TypeAsTrait { ty, trai } => todo!(),
            ImplBlockVariant::Err(_) => ImplBlockKind::Err,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ImplBlockId {
    module_path: ModulePath,
    impl_block_kind: ImplBlockKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ImplBlockKind {
    Type { ty: TypePath },
    TypeAsTrait { ty: TypePath, trai: TraitPath },
    Err,
}

impl ImplBlock {
    pub(crate) fn parse_from_token_group<'a>(
        db: &dyn EntityTreeDb,
        module_symbol_context: ModuleSymbolContext<'a>,
        module_path: ModulePath,
        ast_idx: AstIdx,
        body: AstIdxRange,
        mut token_stream: TokenStream<'a>,
        princiapl_entity_path_expr_arena: &mut PrincipalPathExprArena,
    ) -> Self {
        let mut parser = PrincipalPathExprParser::new(
            token_stream,
            princiapl_entity_path_expr_arena,
            module_symbol_context,
        );
        parser.parse::<ImplToken>().unwrap().unwrap();
        if let Some(_) = parser.try_parse::<LeftAngleBracketToken>() {
            match ignore_implicit_parameters(&mut parser) {
                Ok(_) => (),
                Err(e) => todo!(),
            }
        }
        let (expr, path) = match parser.parse_principal_entity_path_expr() {
            Ok((expr, path)) => (expr, path),
            Err(e) => {
                return new_impl_block(
                    db,
                    module_path,
                    ast_idx,
                    body,
                    ImplBlockVariant::Err(e.into()),
                );
            }
        };
        match path {
            ModuleItemPath::Type(ty) => new_impl_block(
                db,
                module_path,
                ast_idx,
                body,
                ImplBlockVariant::Type { ty },
            ),
            ModuleItemPath::Trait(_) => {
                todo!();

                new_impl_block(
                    db,
                    module_path,
                    ast_idx,
                    body,
                    ImplBlockVariant::TypeAsTrait {
                        ty: todo!(),
                        trai: todo!(),
                    },
                )
            }
            ModuleItemPath::Form(_) => todo!(),
        }
    }

    pub fn kind(&self, db: &dyn EntityTreeDb) -> ImplBlockKind {
        self.variant(db).kind()
    }

    pub fn module_path(&self, db: &dyn EntityTreeDb) -> ModulePath {
        self.id(db).module_path
    }
}

fn new_impl_block(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
    ast_idx: ArenaIdx<Ast>,
    body: ArenaIdxRange<Ast>,
    variant: ImplBlockVariant,
) -> ImplBlock {
    ImplBlock::new(
        db,
        ImplBlockId {
            module_path,
            impl_block_kind: variant.kind(),
        },
        ast_idx,
        body,
        variant,
    )
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ImplBlockError {
    #[error("unmatched angle bras")]
    UnmatchedAngleBras,
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("principal path expr error")]
    PrincipalPath(#[from] PrincipalPathExprError),
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
) -> EntityTreeCrateBundleResult<Vec<ImplBlock>> {
    let crate_path = ty.module_path(db).crate_path(db);
    let entity_tree_crate_bundle = db.entity_tree_crate_bundle(crate_path)?;
    Ok(entity_tree_crate_bundle
        .impl_block_iter()
        .filter_map(|impl_block| {
            (impl_block.variant(db) == &ImplBlockVariant::Type { ty }).then_some(impl_block)
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
            (associated_item.impl_block().kind(db) == ImplBlockKind::Type { ty })
                .then_some((associated_item.ident(), idx))
        })
        .collect())
}
