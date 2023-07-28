mod props_ty_variant;
mod tuple_ty_variant;
mod unit_ty_variant;

pub use self::props_ty_variant::*;
pub use self::tuple_ty_variant::*;
pub use self::unit_ty_variant::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TypeVariantSynNodeDecl {
    Props(PropsTypeVariantSynNodeDecl),
    Unit(UnitTypeVariantSynNodeDecl),
    Tuple(TupleTypeVariantSynNodeDecl),
}

impl TypeVariantSynNodeDecl {
    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> TypeVariantSynNodePath {
        match self {
            TypeVariantSynNodeDecl::Props(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeVariantSynNodeDecl::Unit(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeVariantSynNodeDecl::Tuple(syn_node_decl) => syn_node_decl.syn_node_path(db),
        }
    }

    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        match self {
            TypeVariantSynNodeDecl::Props(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeVariantSynNodeDecl::Unit(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeVariantSynNodeDecl::Tuple(syn_node_decl) => syn_node_decl.ast_idx(db),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        match self {
            TypeVariantSynNodeDecl::Props(_) => todo!(),
            TypeVariantSynNodeDecl::Unit(_) => todo!(),
            TypeVariantSynNodeDecl::Tuple(_) => todo!(),
        }
    }
}

impl HasNodeDecl for TypeVariantSynNodePath {
    type NodeDecl = TypeVariantSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        ty_variant_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_variant_syn_node_decl(
    db: &dyn SynDeclDb,
    syn_node_path: TypeVariantSynNodePath,
) -> TypeVariantSynNodeDecl {
    DeclParser::new(db, syn_node_path.module_path(db)).parse_ty_variant_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_ty_variant_syn_node_decl(
        &self,
        syn_node_path: TypeVariantSynNodePath,
    ) -> TypeVariantSynNodeDecl {
        let db = self.db();
        let node = syn_node_path.syn_node(db);
        let ast_idx = node.ast_idx(db);
        let Ast::TypeVariant {
            token_group_idx,
            vertical_token,
            ident_token,
            state_after,
            ..
        } = self.ast_sheet()[ast_idx]
        else {
            unreachable!()
        };
        let mut parser = self.expr_parser(
            syn_node_path,
            Some(
                syn_node_path
                    .parent_ty_node_path(db)
                    .syn_node_decl(db)
                    .syn_expr_region(db),
            ),
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, Some(state_after));
        let state = ctx.save_state();
        match ctx.next() {
            Some(Token::Punctuation(Punctuation::LPAR)) => {
                let field_comma_list = ctx.try_parse();
                let rpar = ctx.try_parse();
                TupleTypeVariantSynNodeDecl::new(
                    db,
                    syn_node_path,
                    ast_idx,
                    state.next_token_idx(),
                    field_comma_list,
                    rpar,
                    parser.finish(),
                )
                .into()
            }
            Some(Token::Punctuation(Punctuation::LCURL)) => todo!(),
            None => {
                UnitTypeVariantSynNodeDecl::new(db, syn_node_path, ast_idx, parser.finish()).into()
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TypeVariantSynDecl {
    Props(PropsTypeVariantSynDecl),
    Unit(UnitTypeVariantSynDecl),
    Tuple(TupleTypeVariantSynDecl),
}

impl TypeVariantSynDecl {
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypeVariantPath,
        syn_node_decl: TypeVariantSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            TypeVariantSynNodeDecl::Props(syn_node_decl) => {
                PropsTypeVariantSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeVariantSynNodeDecl::Unit(syn_node_decl) => {
                UnitTypeVariantSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeVariantSynNodeDecl::Tuple(syn_node_decl) => {
                TupleTypeVariantSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
        })
    }

    pub fn path(self, db: &dyn SynDeclDb) -> TypeVariantPath {
        match self {
            TypeVariantSynDecl::Props(_) => todo!(),
            TypeVariantSynDecl::Unit(_) => todo!(),
            TypeVariantSynDecl::Tuple(_) => todo!(),
        }
    }

    pub(crate) fn ast_idx(self, _db: &dyn SynDeclDb) -> AstIdx {
        todo!()
    }
}

impl HasSynDecl for TypeVariantPath {
    type Decl = TypeVariantSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        ty_variant_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_variant_syn_decl(
    db: &dyn SynDeclDb,
    path: TypeVariantPath,
) -> DeclResult<TypeVariantSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    TypeVariantSynDecl::from_node_decl(db, path, syn_node_decl)
}
