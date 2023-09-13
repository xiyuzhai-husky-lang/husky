mod ill_formed_impl_block;
mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::ill_formed_impl_block::*;
pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use crate::*;
use husky_coword::IdentPairMap;
use husky_entity_taxonomy::TypeItemKind;
use husky_print_utils::p;
use husky_token::ImplToken;
use husky_token::*;
use maybe_result::*;
use parsec::{HasStreamState, StreamParser};
use smallvec::SmallVec;
use thiserror::Error;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
#[enum_class::from_variants]
pub enum ImplBlockSynNodePath {
    TypeImplBlock(TypeImplBlockSynNodePath),
    TraitForTypeImplBlock(TraitForTypeImplBlockSynNodePath),
    IllFormedImplBlock(IllFormedImplBlockSynNodePath),
}

pub(crate) struct ImplBlockNodePathRegistry {}

impl ImplBlockSynNodePath {
    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<ImplBlockPath> {
        match self {
            ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => Some(syn_node_path.path().into()),
            ImplBlockSynNodePath::TraitForTypeImplBlock(syn_node_path) => {
                Some(syn_node_path.path().into())
            }
            ImplBlockSynNodePath::IllFormedImplBlock(_) => None,
        }
    }

    pub(crate) fn syn_node(self, db: &dyn EntitySynTreeDb) -> ImplBlockSynNode {
        todo!()
    }

    pub fn item_syn_node_paths(self, db: &dyn EntitySynTreeDb) -> &[AssociatedItemPath] {
        todo!()
    }
}

impl<Db> HasModulePath<Db> for ImplBlockSynNodePath
where
    Db: ?Sized + EntitySynTreeDb,
{
    fn module_path(self, db: &Db) -> ModulePath {
        match self {
            ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => syn_node_path.module_path(db),
            ImplBlockSynNodePath::TraitForTypeImplBlock(syn_node_path) => {
                syn_node_path.module_path(db)
            }
            ImplBlockSynNodePath::IllFormedImplBlock(syn_node_path) => {
                syn_node_path.module_path(db)
            }
        }
    }
}

impl HasSynNodePath for ImplBlockPath {
    type SynNodePath = ImplBlockSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
#[enum_class::from_variants]
pub enum ImplBlockSynNode {
    TypeImplBlock(TypeImplBlockSynNode),
    TraitForTypeImplBlock(TraitForTypeImplBlockSynNode),
    IllFormedImplBlock(IllFormedImplBlockSynNode),
}

impl ImplBlockSynNode {
    pub fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> ImplBlockSynNodePath {
        match self {
            ImplBlockSynNode::TypeImplBlock(impl_block) => impl_block.syn_node_path(db).into(),
            ImplBlockSynNode::TraitForTypeImplBlock(impl_block) => {
                impl_block.syn_node_path(db).into()
            }
            ImplBlockSynNode::IllFormedImplBlock(impl_block) => impl_block.syn_node_path(db).into(),
        }
    }

    pub fn for_each_item(
        self,
        db: &dyn EntitySynTreeDb,
        f: impl FnMut(),
    ) -> &[AssociatedItemSynNodePath] {
        todo!()
    }
}

impl ImplBlockSynNode {
    pub(crate) fn parse_from_token_group<'a, 'b>(
        db: &dyn EntitySynTreeDb,
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
        db: &dyn EntitySynTreeDb,
        crate_root_path: ModulePath,
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
                            if let Some(at_token) = parser.try_parse_option::<AtToken>()? {
                                let derive_token: DeriveToken = parser
                                    .try_parse_expected(ImplBlockIllForm::ExpectedDeriveIdent)?;
                                if let Some(underscore_token) =
                                    parser.try_parse_option::<UnderscoreToken>()?
                                {
                                    (
                                        SelfTypeSketchExpr::DeriveAny {
                                            at_token,
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
            MajorItemPath::Fugitive(_) => todo!(),
        })
    }

    pub fn module_path(&self, _db: &dyn EntitySynTreeDb) -> ModulePath {
        todo!()
        // self.id(db).module_path
    }

    pub fn items(self, db: &dyn EntitySynTreeDb) -> &[(Ident, AssociatedItemSynNode)] {
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
#[salsa::debug_with_db(db = EntitySynTreeDb)]
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
