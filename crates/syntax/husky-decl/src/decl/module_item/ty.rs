mod r#enum;
mod r#extern;
mod inductive;
mod record;
mod regular_struct;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::regular_struct::*;
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
    RegularStruct(RegularStructTypeNodeDecl),
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
            TypeNodeDecl::Enum(decl) => decl.node_path(db),
            TypeNodeDecl::Inductive(decl) => decl.node_path(db),
            TypeNodeDecl::Record(decl) => decl.node_path(db),
            TypeNodeDecl::UnitStruct(decl) => decl.node_path(db),
            TypeNodeDecl::RegularStruct(decl) => decl.node_path(db),
            TypeNodeDecl::TupleStruct(decl) => decl.node_path(db),
            TypeNodeDecl::Structure(decl) => decl.node_path(db),
            TypeNodeDecl::Extern(decl) => decl.node_path(db),
            TypeNodeDecl::Union(decl) => decl.node_path(db),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        todo!()
        // match self {
        //     TypeNodeDecl::Enum(decl) => decl.ast_idx(db),
        //     TypeNodeDecl::UnitStruct(decl) => decl.ast_idx(db),
        //     TypeNodeDecl::TupleStruct(decl) => decl.ast_idx(db),
        //     TypeNodeDecl::RegularStruct(decl) => decl.ast_idx(db),
        //     TypeNodeDecl::Record(decl) => decl.ast_idx(db),
        //     TypeNodeDecl::Inductive(decl) => decl.ast_idx(db),
        //     TypeNodeDecl::Structure(decl) => decl.ast_idx(db),
        //     TypeNodeDecl::Extern(decl) => decl.ast_idx(db),
        //     TypeNodeDecl::Union(decl) => decl.ast_idx(db),
        // }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            TypeNodeDecl::Enum(decl) => decl.implicit_parameters(db),
            TypeNodeDecl::UnitStruct(decl) => decl.implicit_parameters(db),
            TypeNodeDecl::TupleStruct(decl) => decl.implicit_parameters(db),
            TypeNodeDecl::RegularStruct(decl) => decl.implicit_parameters(db),
            TypeNodeDecl::Record(decl) => decl.implicit_parameters(db),
            TypeNodeDecl::Inductive(decl) => decl.implicit_parameters(db),
            TypeNodeDecl::Structure(decl) => decl.implicit_parameters(db),
            TypeNodeDecl::Extern(decl) => decl.implicit_parameters(db),
            TypeNodeDecl::Union(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TypeNodeDecl::Enum(decl) => decl.expr_region(db),
            TypeNodeDecl::UnitStruct(decl) => decl.expr_region(db),
            TypeNodeDecl::TupleStruct(decl) => decl.expr_region(db),
            TypeNodeDecl::RegularStruct(decl) => decl.expr_region(db),
            TypeNodeDecl::Record(decl) => decl.expr_region(db),
            TypeNodeDecl::Inductive(decl) => decl.expr_region(db),
            TypeNodeDecl::Structure(decl) => decl.expr_region(db),
            TypeNodeDecl::Extern(decl) => decl.expr_region(db),
            TypeNodeDecl::Union(decl) => decl.expr_region(db),
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
    let ctx = DeclParseContext::new(db, node_path.module_path(db));
    ctx.parse_ty_node_decl(node_path)
}

impl<'a> DeclParseContext<'a> {
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

impl<'a> DeclParseContext<'a> {
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
        if let Some(lcurl) = ctx.parse_err_as_none::<LeftCurlyBraceToken>() {
            let field_comma_list = ctx.parse();
            RegularStructTypeNodeDecl::new(
                db,
                node_path,
                ast_idx,
                implicit_parameters,
                field_comma_list,
                parser.finish(),
            )
            .into()
        } else if let Some(lpar) = ctx.parse_err_as_none::<LeftParenthesisToken>() {
            let field_comma_list = ctx.parse();
            TupleStructTypeNodeDecl::new(
                db,
                node_path,
                ast_idx,
                implicit_parameters,
                lpar,
                field_comma_list,
                parser.finish(),
            )
            .into()
        } else {
            todo!()
            // Err(OriginalDeclError::ExpectedLCurlOrLParOrSemicolon(ctx.save_state()).into())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeDecl {
    Enum(EnumTypeDecl),
    RegularStruct(RegularStructTypeDecl),
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
            TypeDecl::RegularStruct(decl) => decl.path(db),
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
            TypeDecl::RegularStruct(decl) => decl.implicit_parameters(db),
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
            TypeDecl::RegularStruct(decl) => decl.expr_region(db),
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
            TypeNodeDecl::RegularStruct(node_decl) => {
                RegularStructTypeDecl::from_node_decl(db, path, node_decl)?.into()
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
