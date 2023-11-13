mod props_ty_variant;
mod tuple_ty_variant;
mod unit_ty_variant;

pub use self::props_ty_variant::*;
pub use self::tuple_ty_variant::*;
pub use self::unit_ty_variant::*;

use super::*;
use husky_token_data::Punctuation;

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

    pub fn errors(self, _db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        match self {
            TypeVariantSynNodeDecl::Props(_) => todo!(),
            TypeVariantSynNodeDecl::Unit(_) => todo!(),
            TypeVariantSynNodeDecl::Tuple(_) => todo!(),
        }
    }
}

impl HasSynNodeDecl for TypeVariantSynNodePath {
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
    DeclParser::new(db, syn_node_path).parse_ty_variant_syn_node_decl()
}

impl<'a> DeclParser<'a, TypeVariantSynNodePath> {
    fn parse_ty_variant_syn_node_decl(&self) -> TypeVariantSynNodeDecl {
        use parsec::HasStreamState;
        let db = self.db();
        let mut parser = self.expr_parser(
            Some(
                self.syn_node_path()
                    .parent_ty_node_path(db)
                    .syn_node_decl(db)
                    .syn_expr_region(db),
            ),
            AllowSelfType::True,
            AllowSelfValue::False,
            None,
        );
        let state = parser.save_state();
        match parser.next() {
            Some(TokenData::Punctuation(Punctuation::LPAR)) => {
                let field_comma_list = parser.try_parse();
                let rpar = parser.try_parse();
                TupleTypeVariantSynNodeDecl::new(
                    db,
                    self.syn_node_path(),
                    state.next_regional_token_idx(),
                    field_comma_list,
                    rpar,
                    parser.finish(),
                )
                .into()
            }
            Some(TokenData::Punctuation(Punctuation::LCURL)) => todo!(),
            None => {
                UnitTypeVariantSynNodeDecl::new(db, self.syn_node_path(), parser.finish()).into()
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

    pub fn path(self, _db: &dyn SynDeclDb) -> TypeVariantPath {
        match self {
            TypeVariantSynDecl::Props(_) => todo!(),
            TypeVariantSynDecl::Unit(_) => todo!(),
            TypeVariantSynDecl::Tuple(_) => todo!(),
        }
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
