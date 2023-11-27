mod ill_formed_impl_block;
mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::ill_formed_impl_block::*;
pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use crate::*;

use husky_print_utils::p;
use husky_token::ImplToken;
use husky_token::*;

use parsec::{HasStreamState, IsStreamParser};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[enum_class::from_variants]
pub enum ImplBlockSynNodePath {
    TypeImplBlock(TypeImplBlockSynNodePath),
    TraitForTypeImplBlock(TraitForTypeImplBlockSynNodePath),
    IllFormedImplBlock(IllFormedImplBlockSynNodePath),
}

impl std::ops::Deref for ImplBlockSynNodePath {
    type Target = ItemSynNodePathId;

    fn deref(&self) -> &Self::Target {
        let slf: &(u32, ItemSynNodePathId) = unsafe { std::mem::transmute(self) };
        &slf.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[enum_class::from_variants]
pub enum ImplBlockSynNodePathData {
    TypeImplBlock(TypeImplBlockSynNodePathData),
    TraitForTypeImplBlock(TraitForTypeImplBlockSynNodePathData),
    IllFormedImplBlock(IllFormedImplBlockSynNodePathData),
}

pub(crate) struct ImplBlockNodePathRegistry {}

impl ImplBlockSynNodePath {
    pub fn path(self, db: &::salsa::Db) -> Option<ImplBlockPath> {
        Some(match (*self).path(db)? {
            ItemPath::ImplBlock(path) => path,
            _ => unreachable!(),
        })
    }

    pub(crate) fn syn_node(self, _db: &::salsa::Db) -> ImplBlockSynNode {
        todo!()
    }

    pub fn item_syn_node_paths(self, _db: &::salsa::Db) -> &[AssociatedItemPath] {
        todo!()
    }
}

impl HasSynNodePath for ImplBlockPath {
    type SynNodePath = ImplBlockSynNodePath;

    fn syn_node_path(self, _db: &::salsa::Db) -> Self::SynNodePath {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub(crate) enum ImplBlockSynNode {
    TypeImplBlock(TypeImplBlockSynNode),
    TraitForTypeImplBlock(TraitForTypeImplBlockSynNode),
    IllFormedImplBlock(IllFormedImplBlockSynNode),
}

impl ImplBlockSynNode {
    pub fn syn_node_path(self, db: &::salsa::Db) -> ImplBlockSynNodePath {
        match self {
            ImplBlockSynNode::TypeImplBlock(impl_block) => impl_block.syn_node_path.into(),
            ImplBlockSynNode::TraitForTypeImplBlock(impl_block) => impl_block.syn_node_path.into(),
            ImplBlockSynNode::IllFormedImplBlock(impl_block) => impl_block.syn_node_path.into(),
        }
    }

    pub fn for_each_item(
        self,
        _db: &::salsa::Db,
        _f: impl FnMut(),
    ) -> &[AssociatedItemSynNodePath] {
        todo!()
    }
}

impl ImplBlockSynNode {
    pub(crate) fn parse_from_token_group<'a, 'b>(
        db: &::salsa::Db,
        crate_root_path: ModulePath,
        registry: &mut ImplBlockRegistry,
        item_tree_context: EntityTreeSymbolContext<'a, 'b>,
        module_path: ModulePath,
        ast_idx: AstIdx,
        items: Option<ImplBlockItems>, // there could be no items for trait impl block
        token_stream: TokenStream<'a>,
        princiapl_item_path_expr_arena: &mut MajorPathExprArena,
    ) -> Self {
        let mut parser = MajorItemPathExprParser::new(
            db,
            crate_root_path,
            token_stream,
            princiapl_item_path_expr_arena,
            item_tree_context,
        );
        let impl_regional_token = parser
            .try_parse_option::<ImplToken>()
            .expect("okay guaranteed by `husky-ast`")
            .expect("some guaranteed by `husky-ast`");
        match Self::parse_from_token_group_aux(
            db,
            crate_root_path,
            registry,
            module_path,
            ast_idx,
            items,
            parser,
            impl_regional_token,
        ) {
            Ok(node) => node,
            Err(ill_form) => IllFormedImplBlockSynNode::new(
                db,
                registry,
                impl_regional_token,
                module_path,
                ast_idx,
                items,
                ill_form,
            )
            .into(),
        }
    }

    pub(crate) fn parse_from_token_group_aux<'a, 'b>(
        db: &::salsa::Db,
        _crate_root_path: ModulePath,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        items: Option<ImplBlockItems>, // there could be no items for trait impl block
        mut parser: MajorItemPathExprParser,
        impl_token: ImplToken,
    ) -> Result<Self, ImplBlockIllForm> {
        if let Some(_) = parser.try_parse_err_as_none::<LaOrLtToken>() {
            match ignore_template_parameters(&mut parser) {
                Ok(_) => (),
                Err(_e) => todo!(),
            }
        }
        let (expr, path) = parser.parse_major_path_expr_expected()?;
        Ok(match path {
            MajorItemPath::Type(ty) => {
                let Some(ImplBlockItems::Type(items)) = items else {
                    unreachable!("it should be guaranteed in `husky-ast` that items are not none")
                };
                TypeImplBlockSynNode::new(
                    db,
                    impl_token,
                    registry,
                    module_path,
                    ast_idx,
                    items,
                    ty,
                    expr,
                )
                .into()
            }
            MajorItemPath::Trait(trai_path) => {
                let trai_expr = expr;
                let for_token = match ignore_util_for_is_eaten(&mut parser) {
                    Ok(for_token) => for_token,
                    Err(_) => todo!(),
                };
                let (ty_path_expr, ty_sketch) =
                    match parser.parse_major_path_expr().into_result_option()? {
                        Some((expr, MajorItemPath::Type(path))) => {
                            (SelfTypeSketchExpr::Path(expr), TypeSketch::Path(path))
                        }
                        Some(_) => Err(ImplBlockIllForm::ExpectTypePathAfterForKeyword)?,
                        None => {
                            if let Some(pound_token) = parser.try_parse_option::<PoundToken>()? {
                                let derive_token: DeriveToken = parser
                                    .try_parse_expected(ImplBlockIllForm::ExpectedDeriveIdent)?;
                                if let Some(underscore_token) =
                                    parser.try_parse_option::<UnderscoreToken>()?
                                {
                                    (
                                        SelfTypeSketchExpr::DeriveAny {
                                            pound_token,
                                            derive_token,
                                            underscore_token,
                                        },
                                        TypeSketch::DeriveAny,
                                    )
                                } else {
                                    todo!()
                                }
                            } else {
                                use husky_print_utils::ep;
                                ep!(module_path.debug(db), for_token);
                                todo!()
                            }
                        }
                    };
                match TraitForTypeImplBlockSynNode::new(
                    db,
                    registry,
                    module_path,
                    ast_idx,
                    impl_token,
                    trai_expr,
                    trai_path,
                    for_token,
                    ty_path_expr,
                    ty_sketch,
                    items,
                ) {
                    Ok(node) => node.into(),
                    Err(_) => todo!(),
                }
            }
            MajorItemPath::Fugitive(fugitive_path) => {
                p!(module_path.debug(db));
                p!(fugitive_path.debug(db));
                todo!()
            }
        })
    }

    pub fn module_path(&self, _db: &::salsa::Db) -> ModulePath {
        todo!()
        // self.id(db).module_path
    }

    pub(crate) fn ast_idx(&self) -> AstIdx {
        match self {
            ImplBlockSynNode::TypeImplBlock(data) => data.ast_idx(),
            ImplBlockSynNode::TraitForTypeImplBlock(data) => data.ast_idx,
            ImplBlockSynNode::IllFormedImplBlock(data) => data.ast_idx,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub enum ImplError {
    #[error("unmatched angle bras")]
    UnmatchedAngleBras,
    #[error("token error")]
    TokenData(#[from] TokenDataError),
    #[error("principal path expr error")]
    MajorPath(#[from] MajorPathExprError),
}

pub type ImplResult<T> = Result<T, ImplError>;

fn ignore_template_parameters<'a>(token_stream: &mut TokenStream<'a>) -> ImplResult<()> {
    let mut layer = 1;
    while let Some(token) = token_stream.next() {
        match token {
            TokenData::Punctuation(Punctuation::LA_OR_LT) => layer += 1,
            TokenData::Punctuation(Punctuation::RA_OR_GT) => {
                layer -= 1;
                if layer == 0 {
                    break;
                }
            }
            TokenData::Error(e) => return Err(e.clone().into()),
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
            TokenData::Keyword(Keyword::Connection(ConnectionKeyword::For)) => {
                return Ok(token_stream.save_state().next_token_idx() - 1)
            }
            TokenData::Error(e) => return Err(e.clone().into()),
            _ => continue,
        }
    }
    todo!()
}

// #[salsa::tracked(jar = EntityTreeJar, return_ref)]
// pub(crate) fn ty_impl_blocks(
//     db: &dyn EntityTreeDb,
//     ty: TypePath,
// ) -> EntityTreeBundleResult<Vec<TypeImplBlockNode>> {
//     let crate_path = ty.module_path(db).crate_path(db);
//     let item_tree_crate_bundle = db.item_tree_bundle(crate_path)?;
//     Ok(item_tree_crate_bundle
//         .all_ty_impl_block_syn_nodes()
//         .filter_map(|impl_block| (impl_block.ty_path(db) == ty).then_some(impl_block))
//         .collect())
// }
