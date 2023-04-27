mod ill_formed_impl_block;
mod registry;
mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::ill_formed_impl_block::*;
pub use self::registry::*;
pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use crate::*;
use husky_print_utils::p;
use husky_token::*;
use husky_word::IdentPairMap;
use parsec::StreamParser;
use thiserror::Error;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum ImplBlock {
    Type(TypeImplBlock),
    TraitForType(TraitForTypeImplBlock),
    IllFormed(IllFormedImplBlock),
}

impl ImplBlock {
    pub fn id(self, db: &dyn EntityTreeDb) -> ImplBlockId {
        match self {
            ImplBlock::Type(impl_block) => impl_block.id(db).into(),
            ImplBlock::TraitForType(impl_block) => impl_block.id(db).into(),
            ImplBlock::IllFormed(impl_block) => impl_block.id(db).into(),
        }
    }
}

impl ImplBlock {
    pub(crate) fn parse_from_token_group<'a>(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        module_symbol_context: ModuleSymbolContext<'a>,
        module_path: ModulePath,
        ast_idx: AstIdx,
        body: ImplBlockItems,
        token_stream: TokenStream<'a>,
        princiapl_entity_path_expr_arena: &mut MajorPathExprArena,
    ) -> Self {
        let mut parser = MajorPathExprParser::new(
            db,
            token_stream,
            princiapl_entity_path_expr_arena,
            module_symbol_context,
        );
        let impl_token = parser.parse::<ImplToken>().unwrap().unwrap();
        if let Some(_) = parser.try_parse::<LeftAngleBracketOrLessThanToken>() {
            match ignore_implicit_parameters(&mut parser) {
                Ok(_) => (),
                Err(_e) => todo!(),
            }
        }
        let (expr, path) = match parser.parse_major_path_expr() {
            Ok((expr, path)) => (expr, path),
            Err(e) => {
                if module_path.ident(db).data(db) == "list" {
                    todo!()
                }
                return IllFormedImplBlock::new(
                    db,
                    registry,
                    impl_token,
                    module_path,
                    ast_idx,
                    body,
                    ImplBlockIllForm::MajorPath(e),
                )
                .into();
            }
        };
        match path {
            ModuleItemPath::Type(ty) => TypeImplBlock::new(
                db,
                impl_token,
                registry,
                module_path,
                ast_idx,
                body,
                ty,
                expr,
            )
            .into(),
            ModuleItemPath::Trait(trai_path) => {
                let trai_expr = expr;
                let for_token = match ignore_util_for_is_eaten(&mut parser) {
                    Ok(for_token) => for_token,
                    Err(_) => todo!(),
                };
                let (ty_expr, ty_path) = match parser.parse_major_path_expr() {
                    Ok((expr, ModuleItemPath::Type(path))) => (expr, path),
                    Ok((expr, path)) => {
                        return IllFormedImplBlock::new(
                            db,
                            registry,
                            impl_token,
                            module_path,
                            ast_idx,
                            body,
                            ImplBlockIllForm::ExpectTypePathAfterForKeyword,
                        )
                        .into();
                    }
                    Err(e) => {
                        return IllFormedImplBlock::new(
                            db,
                            registry,
                            impl_token,
                            module_path,
                            ast_idx,
                            body,
                            ImplBlockIllForm::MissingForKeyword,
                        )
                        .into();
                    }
                };
                TraitForTypeImplBlock::new(
                    db,
                    registry,
                    module_path,
                    ast_idx,
                    impl_token,
                    trai_expr,
                    trai_path,
                    for_token,
                    ty_expr,
                    ty_path,
                    body,
                )
                .into()
            }
            ModuleItemPath::Form(_) => todo!(),
        }
    }

    pub fn module_path(&self, _db: &dyn EntityTreeDb) -> ModulePath {
        todo!()
        // self.id(db).module_path
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum ImplBlockId {
    Type(TypeImplBlockId),
    TraitForType(TraitForTypeImplBlockId),
    IllFormed(IllFormedImplBlockId),
}

impl ImplBlockId {
    pub fn module(self) -> ModulePath {
        match self {
            ImplBlockId::Type(id) => id.module_path(),
            ImplBlockId::TraitForType(id) => id.module(),
            ImplBlockId::IllFormed(id) => id.module(),
        }
    }
}

fn new_impl(
    _db: &dyn EntityTreeDb,
    _registry: &mut ImplBlockRegistry,
    _module_path: ModulePath,
    _ast_idx: ArenaIdx<Ast>,
    _body: ArenaIdxRange<Ast>,
    _variant: (),
) -> ImplBlock {
    // let impl_kind = variant.kind();
    todo!()
    // ImplBlock::new(
    //     db,
    //     ImplId {
    //         module_path,
    //         impl_kind,
    //         disambiguator: registry.issue_disambiguitor(module_path, impl_kind),
    //     },
    //     ast_idx,
    //     body,
    //     variant,
    // )
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
    let mut layer = 1;
    while let Some(token) = token_stream.next() {
        match token {
            Token::Punctuation(Punctuation::LA_OR_LT) => layer += 1,
            Token::Punctuation(Punctuation::RA_OR_GT) => {
                layer -= 1;
                if layer == 0 {
                    break;
                }
            }
            Token::Error(e) => return Err(e.clone().into()),
            _ => (),
        }
    }
    match layer {
        0 => Ok(()),
        _ => Err(ImplError::UnmatchedAngleBras),
    }
}

fn ignore_util_for_is_eaten<'a>(token_stream: &mut TokenStream<'a>) -> ImplResult<TokenIdx> {
    while let Some(token) = token_stream.next() {
        match token {
            Token::Keyword(Keyword::Connection(ConnectionKeyword::For)) => {
                return Ok(token_stream.state() - 1)
            }
            Token::Error(e) => return Err(e.clone().into()),
            _ => continue,
        }
    }
    todo!()
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_impl_blocks(
    db: &dyn EntityTreeDb,
    ty: TypePath,
) -> EntityTreeBundleResult<Vec<TypeImplBlock>> {
    let crate_path = ty.module_path(db).crate_path(db);
    let entity_tree_crate_bundle = db.entity_tree_bundle(crate_path)?;
    Ok(entity_tree_crate_bundle
        .all_ty_impl_blocks()
        .filter_map(|impl_block| (impl_block.ty_path(db) == ty).then_some(impl_block))
        .collect())
}

pub trait HasItems: Copy {
    fn items<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, AssociatedItem)]>;
}

impl HasItems for TypePath {
    fn items<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, AssociatedItem)]> {
        ty_items(db, self).as_ref().map(|v| v as &[_])
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_items(
    db: &dyn EntityTreeDb,
    path: TypePath,
) -> EntityTreeBundleResult<IdentPairMap<AssociatedItem>> {
    let crate_path = path.module_path(db).crate_path(db);
    let entity_tree_crate_bundle = db.entity_tree_bundle(crate_path)?;
    Ok(entity_tree_crate_bundle
        .all_ty_impl_blocks()
        .filter_map(|impl_block| {
            // ad hoc
            // todo: guard against two methods with the same ident
            (impl_block.ty_path(db) == path).then(|| {
                ty_impl_block_items(db, impl_block)
                    .iter()
                    .map(|(ident, associated_item)| (*ident, *associated_item))
            })
        })
        .flatten()
        .collect())
}
