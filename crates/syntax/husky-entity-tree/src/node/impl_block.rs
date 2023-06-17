mod ill_formed_impl_block;
mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::ill_formed_impl_block::*;
pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use crate::*;
use husky_print_utils::p;
use husky_token::*;
use husky_word::IdentPairMap;
use parsec::{HasStreamState, StreamParser};
use thiserror::Error;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum ImplBlockNodePath {
    TypeImplBlock(TypeImplBlockNodePath),
    TraitForTypeImplBlock(TraitForTypeImplBlockNodePath),
    IllFormedImplBlock(IllFormedImplBlockNodePath),
}

pub(crate) struct ImplBlockNodePathRegistry {}

impl ImplBlockNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            ImplBlockNodePath::TypeImplBlock(id) => id.module_path(db),
            ImplBlockNodePath::TraitForTypeImplBlock(id) => id.module_path(db),
            ImplBlockNodePath::IllFormedImplBlock(id) => id.module_path(db),
        }
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> ImplBlockNode {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum ImplBlockNode {
    TypeImplBlock(TypeImplBlockNode),
    TraitForTypeImplBlock(TraitForTypeImplBlockNode),
    IllFormedImplBlock(IllFormedImplBlockNode),
}

impl ImplBlockNode {
    pub fn id(self, db: &dyn EntityTreeDb) -> ImplBlockNodePath {
        match self {
            ImplBlockNode::TypeImplBlock(impl_block) => impl_block.id(db).into(),
            ImplBlockNode::TraitForTypeImplBlock(impl_block) => impl_block.id(db).into(),
            ImplBlockNode::IllFormedImplBlock(impl_block) => impl_block.id(db).into(),
        }
    }
}

impl ImplBlockNode {
    pub(crate) fn parse_from_token_group<'a, 'b>(
        db: &dyn EntityTreeDb,
        crate_root_path: ModulePath,
        registry: &mut ImplBlockRegistry,
        module_symbol_context: ModuleSymbolContext<'a>,
        entity_tree_context: EntityTreeSymbolContext<'a, 'b>,
        module_path: ModulePath,
        ast_idx: AstIdx,
        items: Option<ImplBlockItems>, // there could be no items for trait impl block
        token_stream: TokenStream<'a>,
        princiapl_entity_path_expr_arena: &mut MajorPathExprArena,
    ) -> Self {
        let mut parser = ModuleItemPathExprParser::new(
            db,
            crate_root_path,
            token_stream,
            princiapl_entity_path_expr_arena,
            entity_tree_context,
        );
        let impl_token = parser.parse::<ImplToken>().unwrap().unwrap();
        if let Some(_) = parser.parse_err_as_none::<LeftAngleBracketOrLessThanToken>() {
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
                return IllFormedImplBlockNode::new(
                    db,
                    registry,
                    impl_token,
                    module_path,
                    ast_idx,
                    items,
                    ImplBlockIllForm::MajorPath(e),
                )
                .into();
            }
        };
        match path {
            ModuleItemPath::Type(ty) => TypeImplBlockNode::new(
                db,
                impl_token,
                registry,
                module_path,
                ast_idx,
                items.expect("it should be guaranteed in `husky-ast` that items are not none"),
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
                        return IllFormedImplBlockNode::new(
                            db,
                            registry,
                            impl_token,
                            module_path,
                            ast_idx,
                            items,
                            ImplBlockIllForm::ExpectTypePathAfterForKeyword,
                        )
                        .into();
                    }
                    Err(e) => {
                        return IllFormedImplBlockNode::new(
                            db,
                            registry,
                            impl_token,
                            module_path,
                            ast_idx,
                            items,
                            ImplBlockIllForm::MissingForKeyword,
                        )
                        .into();
                    }
                };
                TraitForTypeImplBlockNode::new(
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
                    items,
                )
                .into()
            }
            ModuleItemPath::Fugitive(_) => todo!(),
        }
    }

    pub fn module_path(&self, _db: &dyn EntityTreeDb) -> ModulePath {
        todo!()
        // self.id(db).module_path
    }

    pub fn items(self, db: &dyn EntityTreeDb) -> &[(Ident, AssociatedItemNode)] {
        todo!()
        // match self {
        //     ImplBlockNode::TypeImplBlock(impl_block) => ty_impl_block_items(db, impl_block),
        //     ImplBlockNode::TraitForTypeImplBlock(impl_block) => {
        //         trai_for_ty_impl_block_items(db, impl_block)
        //     }
        //     ImplBlockNode::IllFormedImplBlock(_) => &[],
        // }
    }
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
                return Ok(token_stream.save_state().next_token_idx() - 1)
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
) -> EntityTreeBundleResult<Vec<TypeImplBlockNode>> {
    let crate_path = ty.module_path(db).crate_path(db);
    let entity_tree_crate_bundle = db.entity_tree_bundle(crate_path)?;
    Ok(entity_tree_crate_bundle
        .all_ty_impl_blocks()
        .filter_map(|impl_block| (impl_block.ty_path(db) == ty).then_some(impl_block))
        .collect())
}

pub trait HasItemNodes: Copy {
    type ItemNode;

    fn items<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Self::ItemNode)]>;
}

impl HasItemNodes for TypePath {
    type ItemNode = TypeItemNode;

    fn items<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, TypeItemNode)]> {
        ty_items(db, self).as_ref().map(|v| v as &[_])
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_items(
    db: &dyn EntityTreeDb,
    path: TypePath,
) -> EntityTreeBundleResult<Vec<(Ident, TypeItemNode)>> {
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
