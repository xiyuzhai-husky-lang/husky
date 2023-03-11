use crate::*;
use husky_token::*;
use husky_word::IdentPairMap;
use parsec::ParseContext;
use thiserror::Error;
use vec_like::VecPairMap;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct ImplBlock {
    #[id]
    pub id: ImplId,
    pub ast_idx: AstIdx,
    pub body: AstIdxRange,
    #[return_ref]
    pub variant: ImplVariant,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ImplVariant {
    Type { ty: TypePath },
    TypeAsTrait { ty: TypePath, trai: TraitPath },
    Err(ImplError),
}

impl ImplVariant {
    pub fn kind(&self) -> ImplKind {
        match self {
            ImplVariant::Type { ty } => ImplKind::Type { ty: *ty },
            ImplVariant::TypeAsTrait { ty: _, trai: _ } => todo!(),
            ImplVariant::Err(_) => ImplKind::Err,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct ImplId {
    module_path: ModulePath,
    impl_kind: ImplKind,
    disambiguator: u8,
}

impl ImplId {
    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ImplKind {
    Type { ty: TypePath },
    TypeAsTrait { ty: TypePath, trai: TraitPath },
    Err,
}

#[derive(Default)]
pub struct ImplRegistry {
    next_disambiguitors: VecPairMap<(ModulePath, ImplKind), u8>,
}
impl ImplRegistry {
    fn issue_disambiguitor(&mut self, module_path: ModulePath, impl_kind: ImplKind) -> u8 {
        let next_disambiguitor = self
            .next_disambiguitors
            .get_mut_or_insert_default((module_path, impl_kind));
        let new_disambiguitor = *next_disambiguitor;
        *next_disambiguitor += 1;
        new_disambiguitor
    }
}

impl ImplBlock {
    pub(crate) fn parse_from_token_group<'a>(
        db: &dyn EntityTreeDb,
        impl_registry: &mut ImplRegistry,
        module_symbol_context: ModuleSymbolContext<'a>,
        module_path: ModulePath,
        ast_idx: AstIdx,
        body: AstIdxRange,
        token_stream: TokenStream<'a>,
        princiapl_entity_path_expr_arena: &mut MajorPathExprArena,
    ) -> Self {
        let mut parser = MajorPathExprParser::new(
            db,
            token_stream,
            princiapl_entity_path_expr_arena,
            module_symbol_context,
        );
        parser.parse::<ImplToken>().unwrap().unwrap();
        if let Some(_) = parser.try_parse::<LeftAngleBracketOrLessThanToken>() {
            match ignore_implicit_parameters(&mut parser) {
                Ok(_) => (),
                Err(_e) => todo!(),
            }
        }
        let (_expr, path) = match parser.parse_principal_path_expr() {
            Ok((expr, path)) => (expr, path),
            Err(e) => {
                return new_impl(
                    db,
                    impl_registry,
                    module_path,
                    ast_idx,
                    body,
                    ImplVariant::Err(e.into()),
                );
            }
        };
        match path {
            ModuleItemPath::Type(ty) => new_impl(
                db,
                impl_registry,
                module_path,
                ast_idx,
                body,
                ImplVariant::Type { ty },
            ),
            ModuleItemPath::Trait(_) => {
                todo!();

                new_impl(
                    db,
                    impl_registry,
                    module_path,
                    ast_idx,
                    body,
                    ImplVariant::TypeAsTrait {
                        ty: todo!(),
                        trai: todo!(),
                    },
                )
            }
            ModuleItemPath::Form(_) => todo!(),
        }
    }

    pub fn kind(&self, db: &dyn EntityTreeDb) -> ImplKind {
        self.variant(db).kind()
    }

    pub fn module_path(&self, db: &dyn EntityTreeDb) -> ModulePath {
        self.id(db).module_path
    }
}

fn new_impl(
    db: &dyn EntityTreeDb,
    registry: &mut ImplRegistry,
    module_path: ModulePath,
    ast_idx: ArenaIdx<Ast>,
    body: ArenaIdxRange<Ast>,
    variant: ImplVariant,
) -> ImplBlock {
    let impl_kind = variant.kind();
    ImplBlock::new(
        db,
        ImplId {
            module_path,
            impl_kind,
            disambiguator: registry.issue_disambiguitor(module_path, impl_kind),
        },
        ast_idx,
        body,
        variant,
    )
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ImplError {
    #[error("unmatched angle bras")]
    UnmatchedAngleBras,
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("principal path expr error")]
    MajorPath(#[from] MajorPathExprError),
}

pub type ImplResult<T> = Result<T, ImplError>;

fn ignore_implicit_parameters<'a>(token_stream: &mut TokenStream<'a>) -> ImplResult<()> {
    let layer = 1;
    while let Some(token) = token_stream.next() {
        match token {
            Token::Punctuation(_) => todo!(),
            Token::Error(e) => return Err(e.clone().into()),
            _ => (),
        }
    }
    match layer {
        0 => Ok(()),
        _ => Err(ImplError::UnmatchedAngleBras),
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
        .impl_iter()
        .filter_map(|impl_block| {
            (impl_block.variant(db) == &ImplVariant::Type { ty }).then_some(impl_block)
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
    Ok(entity_tree_crate_bundle
        .impl_iter()
        .filter_map(|impl_block| {
            (impl_block.kind(db) == ImplKind::Type { ty }).then_some({
                db.impl_associated_items(impl_block)
                    .iter()
                    .map(|(ident, associated_item)| (*ident, *associated_item))
            })
        })
        .flatten()
        .collect())
}
