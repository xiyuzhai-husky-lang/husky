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

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for ImplBlockId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        f.debug_struct("ImplBlockId")
            .field(
                "module_path",
                &self.module_path.debug_with(db, include_all_fields),
            )
            .field(
                "impl_block_kind",
                &self.impl_block_kind.debug_with(db, include_all_fields),
            )
            .finish()
    }
}

impl ImplBlockId {
    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ImplBlockKind {
    Type { ty: TypePath },
    TypeAsTrait { ty: TypePath, trai: TraitPath },
    Err,
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for ImplBlockKind {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        match self {
            ImplBlockKind::Type { ty } => f
                .debug_struct("Type")
                .field("ty", &ty.debug_with(db, include_all_fields))
                .finish(),
            ImplBlockKind::TypeAsTrait { ty, trai } => f
                .debug_struct("TypeAsTrait")
                .field("ty", &ty.debug_with(db, include_all_fields))
                .field("trai", &trai.debug_with(db, include_all_fields))
                .finish(),
            ImplBlockKind::Err => f.write_str("Err"),
        }
    }
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
) -> EntityTreeCrateBundleResult<IdentPairMap<AssociatedItem>> {
    let crate_path = ty.module_path(db).crate_path(db);
    let entity_tree_crate_bundle = db.entity_tree_crate_bundle(crate_path)?;
    todo!()
    // Ok(entity_tree_crate_bundle
    //     .associated_item_indexed_iter()
    //     .filter_map(|(idx, associated_item)| {
    //         (associated_item.impl_block().kind(db) == ImplBlockKind::Type { ty })
    //             .then_some((associated_item.ident(), idx))
    //     })
    //     .collect())
}
