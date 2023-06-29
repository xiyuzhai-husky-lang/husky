mod r#enum;
mod r#extern;
mod inductive;
mod props_struct;
mod record;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use super::*;
use husky_entity_taxonomy::{EntityKind, TypeKind};
use parsec::parse_separated_list2;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeNodeDecl {
    Enum(EnumTypeNodeDecl),
    PropsStruct(PropsStructTypeNodeDecl),
    UnitStruct(UnitStructTypeNodeDecl),
    TupleStruct(TupleStructTypeNodeDecl),
    Record(RecordTypeNodeDecl),
    Inductive(InductiveTypeNodeDecl),
    Structure(StructureTypeNodeDecl),
    Extern(ExternTypeNodeDecl),
    Union(UnionTypeNodeDecl),
}

impl TypeNodeDecl {
    pub fn node_path(self, db: &dyn DeclDb) -> TypeNodePath {
        match self {
            TypeNodeDecl::Enum(node_decl) => node_decl.node_path(db),
            TypeNodeDecl::Inductive(node_decl) => node_decl.node_path(db),
            TypeNodeDecl::Record(node_decl) => node_decl.node_path(db),
            TypeNodeDecl::UnitStruct(node_decl) => node_decl.node_path(db),
            TypeNodeDecl::PropsStruct(node_decl) => node_decl.node_path(db),
            TypeNodeDecl::TupleStruct(node_decl) => node_decl.node_path(db),
            TypeNodeDecl::Structure(node_decl) => node_decl.node_path(db),
            TypeNodeDecl::Extern(node_decl) => node_decl.node_path(db),
            TypeNodeDecl::Union(node_decl) => node_decl.node_path(db),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TypeNodeDecl::Enum(node_decl) => node_decl.ast_idx(db),
            TypeNodeDecl::UnitStruct(node_decl) => node_decl.ast_idx(db),
            TypeNodeDecl::TupleStruct(node_decl) => node_decl.ast_idx(db),
            TypeNodeDecl::PropsStruct(node_decl) => node_decl.ast_idx(db),
            TypeNodeDecl::Record(node_decl) => node_decl.ast_idx(db),
            TypeNodeDecl::Inductive(node_decl) => node_decl.ast_idx(db),
            TypeNodeDecl::Structure(node_decl) => node_decl.ast_idx(db),
            TypeNodeDecl::Extern(node_decl) => node_decl.ast_idx(db),
            TypeNodeDecl::Union(node_decl) => node_decl.ast_idx(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TypeNodeDecl::Enum(node_decl) => node_decl.expr_region(db),
            TypeNodeDecl::UnitStruct(node_decl) => node_decl.expr_region(db),
            TypeNodeDecl::TupleStruct(node_decl) => node_decl.expr_region(db),
            TypeNodeDecl::PropsStruct(node_decl) => node_decl.expr_region(db),
            TypeNodeDecl::Record(node_decl) => node_decl.expr_region(db),
            TypeNodeDecl::Inductive(node_decl) => node_decl.expr_region(db),
            TypeNodeDecl::Structure(node_decl) => node_decl.expr_region(db),
            TypeNodeDecl::Extern(node_decl) => node_decl.expr_region(db),
            TypeNodeDecl::Union(node_decl) => node_decl.expr_region(db),
        }
    }
}

impl HasNodeDecl for TypeNodePath {
    type NodeDecl = TypeNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        ty_node_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn ty_node_decl(db: &dyn DeclDb, node_path: TypeNodePath) -> TypeNodeDecl {
    let ctx = DeclParser::new(db, node_path.module_path(db));
    ctx.parse_ty_node_decl(node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_ty_node_decl(&self, node_path: TypeNodePath) -> TypeNodeDecl {
        let db = self.db();
        let node = node_path.node(db);
        let ast_idx: AstIdx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                block: DefnBlock::Type { path, variants },
                entity_kind,
                saved_stream_state,
                ..
            } => self.parse_ty_node_decl_aux(
                node_path,
                ast_idx,
                path.ty_kind(self.db()),
                entity_kind,
                token_group_idx,
                variants,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_ty_node_decl_aux(
        &self,
        node_path: TypeNodePath,
        ast_idx: AstIdx,
        type_kind: TypeKind,
        _entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        variants: Option<TypeVariants>,
        saved_stream_state: TokenStreamState,
    ) -> TypeNodeDecl {
        match type_kind {
            TypeKind::Enum => self
                .parse_enum_ty_node_decl(
                    node_path,
                    ast_idx,
                    token_group_idx,
                    variants.expect("guaranteed by `husky-ast`"),
                    saved_stream_state,
                )
                .into(),
            TypeKind::Inductive => self
                .parse_inductive_ty_node_decl(
                    ast_idx,
                    node_path,
                    token_group_idx,
                    variants.expect("guaranteed by `husky-ast`"),
                    saved_stream_state,
                )
                .into(),
            TypeKind::Record => todo!(),
            TypeKind::Struct => {
                debug_assert!(variants.is_none());
                self.parse_struct_ty_node_decl(
                    node_path,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
            }
            TypeKind::Structure => {
                debug_assert!(variants.is_none());
                self.parse_structure_ty_node_decl(
                    node_path,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
            }
            TypeKind::Extern => {
                debug_assert!(variants.is_none());
                self.parse_extern_ty_node_decl(
                    node_path,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
                .into()
            }
        }
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_struct_ty_node_decl(
        &self,
        node_path: TypeNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TypeNodeDecl {
        let db = self.db();
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::True, AllowSelfValue::True);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.try_parse_optional();
        if let Some(lpar) = ctx.try_parse_err_as_none::<LeftParenthesisToken>() {
            let field_comma_list = ctx.try_parse();
            let rpar = ctx.try_parse();
            TupleStructTypeNodeDecl::new(
                db,
                node_path,
                ast_idx,
                implicit_parameters,
                lpar,
                field_comma_list,
                rpar,
                parser.finish(),
            )
            .into()
        } else if let Some(semicolon) = ctx.try_parse_err_as_none::<SemiColonToken>() {
            todo!()
            // Err(OriginalDeclError::ExpectedLCurlOrLParOrSemicolon(ctx.save_state()).into())
        } else {
            let lcurl = ctx.try_parse();
            let field_comma_list = ctx.try_parse();
            let rcurl = ctx.try_parse();
            PropsStructTypeNodeDecl::new(
                db,
                node_path,
                ast_idx,
                implicit_parameters,
                lcurl,
                field_comma_list,
                rcurl,
                parser.finish(),
            )
            .into()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeDecl {
    Enum(EnumTypeDecl),
    PropsStruct(PropsStructTypeDecl),
    UnitStruct(UnitStructTypeDecl),
    TupleStruct(TupleStructTypeDecl),
    Record(RecordTypeDecl),
    Inductive(InductiveTypeDecl),
    Structure(StructureTypeDecl),
    Extern(ExternTypeDecl),
    Union(UnionTypeDecl),
}

impl TypeDecl {
    pub fn path(self, db: &dyn DeclDb) -> TypePath {
        match self {
            TypeDecl::Enum(decl) => decl.path(db),
            TypeDecl::Inductive(decl) => decl.path(db),
            TypeDecl::Record(decl) => decl.path(db),
            TypeDecl::UnitStruct(decl) => decl.path(db),
            TypeDecl::PropsStruct(decl) => decl.path(db),
            TypeDecl::TupleStruct(decl) => decl.path(db),
            TypeDecl::Structure(decl) => decl.path(db),
            TypeDecl::Extern(decl) => decl.path(db),
            TypeDecl::Union(decl) => decl.path(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            TypeDecl::Enum(decl) => decl.implicit_parameters(db),
            TypeDecl::UnitStruct(decl) => decl.implicit_parameters(db),
            TypeDecl::TupleStruct(decl) => decl.implicit_parameters(db),
            TypeDecl::PropsStruct(decl) => decl.implicit_parameters(db),
            TypeDecl::Record(decl) => decl.implicit_parameters(db),
            TypeDecl::Inductive(decl) => decl.implicit_parameters(db),
            TypeDecl::Structure(decl) => decl.implicit_parameters(db),
            TypeDecl::Extern(decl) => decl.implicit_parameters(db),
            TypeDecl::Union(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TypeDecl::Enum(decl) => decl.expr_region(db),
            TypeDecl::UnitStruct(decl) => decl.expr_region(db),
            TypeDecl::TupleStruct(decl) => decl.expr_region(db),
            TypeDecl::PropsStruct(decl) => decl.expr_region(db),
            TypeDecl::Record(decl) => decl.expr_region(db),
            TypeDecl::Inductive(decl) => decl.expr_region(db),
            TypeDecl::Structure(decl) => decl.expr_region(db),
            TypeDecl::Extern(decl) => decl.expr_region(db),
            TypeDecl::Union(decl) => decl.expr_region(db),
        }
    }

    #[inline(always)]
    fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: TypeNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match node_decl {
            TypeNodeDecl::Enum(node_decl) => {
                EnumTypeDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TypeNodeDecl::PropsStruct(node_decl) => {
                PropsStructTypeDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TypeNodeDecl::UnitStruct(node_decl) => {
                UnitStructTypeDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TypeNodeDecl::TupleStruct(node_decl) => {
                TupleStructTypeDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TypeNodeDecl::Record(node_decl) => {
                RecordTypeDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TypeNodeDecl::Inductive(node_decl) => {
                InductiveTypeDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TypeNodeDecl::Structure(node_decl) => {
                StructureTypeDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TypeNodeDecl::Extern(node_decl) => {
                ExternTypeDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TypeNodeDecl::Union(node_decl) => {
                UnionTypeDecl::from_node_decl(db, path, node_decl)?.into()
            }
        })
    }
}

impl HasDecl for TypePath {
    type Decl = TypeDecl;

    #[inline(always)]
    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        ty_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn ty_decl(db: &dyn DeclDb, path: TypePath) -> DeclResult<TypeDecl> {
    TypeDecl::from_node_decl(db, path, path.node_path(db).node_decl(db))
}

#[test]
fn ty_decl_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let menu = db.entity_path_menu(toolchain);
    assert!(menu.never_ty_path().decl(&db).is_ok());
}
