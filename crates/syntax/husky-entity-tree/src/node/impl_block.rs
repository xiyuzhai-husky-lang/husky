mod ill_formed_impl_block;
mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::ill_formed_impl_block::*;
pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use crate::*;

use husky_token::ImplToken;
use husky_token::*;

use parsec::{HasStreamState, IsStreamParser};

use thiserror::Error;

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum ImplBlockSynNodePathData {
    TypeImplBlock(TypeImplBlockSynNodePathData),
    TraitForTypeImplBlock(TraitForTypeImplBlockSynNodePathData),
    IllFormedImplBlock(IllFormedImplBlockSynNodePathData),
}

impl ImplBlockSynNodePath {
    pub fn path(self, db: &::salsa::Db) -> Option<ImplBlockPath> {
        Some(match (*self).path(db)? {
            ItemPath::ImplBlock(path) => path,
            _ => unreachable!(),
        })
    }
}

impl ImplBlockSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> ImplBlockSynNodePath {
        match self {
            ImplBlockSynNodePathData::TypeImplBlock(slf) => slf.syn_node_path(id).into(),
            ImplBlockSynNodePathData::TraitForTypeImplBlock(slf) => slf.syn_node_path(id).into(),
            ImplBlockSynNodePathData::IllFormedImplBlock(slf) => slf.syn_node_path(id).into(),
        }
    }

    pub fn path(self) -> Option<ImplBlockPath> {
        match self {
            ImplBlockSynNodePathData::TypeImplBlock(slf) => Some(slf.path().into()),
            ImplBlockSynNodePathData::TraitForTypeImplBlock(slf) => Some(slf.path.into()),
            ImplBlockSynNodePathData::IllFormedImplBlock(_slf) => None,
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            ImplBlockSynNodePathData::TypeImplBlock(slf) => slf.module_path(db),
            ImplBlockSynNodePathData::TraitForTypeImplBlock(slf) => slf.module_path(db),
            ImplBlockSynNodePathData::IllFormedImplBlock(slf) => slf.module_path(db),
        }
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        match self {
            ImplBlockSynNodePathData::TypeImplBlock(slf) => slf.ast_idx(id, db),
            ImplBlockSynNodePathData::TraitForTypeImplBlock(slf) => slf.ast_idx(id, db),
            ImplBlockSynNodePathData::IllFormedImplBlock(slf) => slf.ast_idx(id, db),
        }
    }
}

impl HasSynNodePath for ImplBlockPath {
    type SynNodePath = ImplBlockSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        match self {
            ImplBlockPath::TypeImplBlock(slf) => slf.syn_node_path(db).into(),
            ImplBlockPath::TraitForTypeImplBlock(slf) => slf.syn_node_path(db).into(),
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub(crate) enum ImplBlockSynNode {
    TypeImplBlock(TypeImplBlockSynNode),
    TraitForTypeImplBlock(TraitForTypeImplBlockSynNode),
    IllFormedImplBlock(IllFormedImplBlockSynNode),
}

impl ImplBlockSynNode {
    pub fn syn_node_path(&self) -> ImplBlockSynNodePath {
        match self {
            ImplBlockSynNode::TypeImplBlock(impl_block) => impl_block.syn_node_path().into(),
            ImplBlockSynNode::TraitForTypeImplBlock(impl_block) => {
                impl_block.syn_node_path().into()
            }
            ImplBlockSynNode::IllFormedImplBlock(impl_block) => impl_block.syn_node_path().into(),
        }
    }
}

impl ImplBlockSynNode {
    pub(crate) fn parse_from_token_verse<'a, 'b>(
        db: &::salsa::Db,
        crate_root_path: ModulePath,
        item_tree_context: EntityTreeSymbolContext<'a, 'b>,
        module_path: ModulePath,
        ast_idx: AstIdx,
        items: Option<ImplBlockItems>, // there could be no items for trait impl block
        token_stream: TokenStream<'a>,
        registry: &mut ImplBlockRegistry,
        princiapl_item_path_expr_arena: &mut MajorPathExprArena,
    ) -> Self {
        let mut parser = MajorItemPathExprParser::new(
            db,
            crate_root_path,
            module_path,
            token_stream,
            princiapl_item_path_expr_arena,
            item_tree_context,
        );
        let impl_regional_token = parser
            .try_parse_option::<ImplToken>()
            .expect("okay guaranteed by `husky-ast`")
            .expect("some guaranteed by `husky-ast`");
        match Self::parse_from_token_verse_aux(
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

    pub(crate) fn parse_from_token_verse_aux<'a, 'b>(
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
            ignore_template_parameters(&mut parser)?
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
                let for_token = ignore_util_for_is_eaten(&mut parser)?;
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
                                    Err(ImplBlockIllForm::InvalidTypeSketch)?
                                }
                            } else {
                                Err(ImplBlockIllForm::InvalidTypeSketch)?
                            }
                        }
                    };
                TraitForTypeImplBlockSynNode::new(
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
                )?
                .into()
            }
            MajorItemPath::Fugitive(fugitive_path) => {
                Err(ImplBlockIllForm::UnexpectedFugitivePath(fugitive_path))?
            }
        })
    }
}

pub type ImplBlockResult<T> = Result<T, ImplBlockIllForm>;

fn ignore_template_parameters<'a>(token_stream: &mut TokenStream<'a>) -> ImplBlockResult<()> {
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
        _ => Err(ImplBlockIllForm::UnmatchedAngleBras),
    }
}

fn ignore_util_for_is_eaten<'a>(token_stream: &mut TokenStream<'a>) -> ImplBlockResult<TokenIdx> {
    while let Some(token) = token_stream.next() {
        match token {
            TokenData::Keyword(Keyword::Connection(ConnectionKeyword::For)) => {
                return Ok(token_stream.state().next_token_idx() - 1)
            }
            TokenData::Error(e) => return Err(e.clone().into()),
            _ => continue,
        }
    }
    Err(ImplBlockIllForm::MissingForKeyword)
}
