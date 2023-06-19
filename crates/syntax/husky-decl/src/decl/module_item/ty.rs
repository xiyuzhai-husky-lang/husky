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
    pub fn node_id(self, db: &dyn DeclDb) -> TypeNodeId {
        match self {
            TypeNodeDecl::Enum(decl) => decl.node_id(db),
            TypeNodeDecl::Inductive(decl) => decl.node_id(db),
            TypeNodeDecl::Record(decl) => decl.node_id(db),
            TypeNodeDecl::UnitStruct(decl) => decl.node_id(db),
            TypeNodeDecl::RegularStruct(decl) => decl.node_id(db),
            TypeNodeDecl::TupleStruct(decl) => decl.node_id(db),
            TypeNodeDecl::Structure(decl) => decl.node_id(db),
            TypeNodeDecl::Extern(decl) => decl.node_id(db),
            TypeNodeDecl::Union(decl) => decl.node_id(db),
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

impl HasNodeDecl for TypeNodeId {
    type NodeDecl = TypeNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        ty_node_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn ty_node_decl(db: &dyn DeclDb, node_id: TypeNodeId) -> TypeNodeDecl {
    todo!()
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
    pub fn node_id(self, db: &dyn DeclDb) -> TypeNodeId {
        match self {
            TypeDecl::Enum(decl) => decl.node_id(db),
            TypeDecl::Inductive(decl) => decl.node_id(db),
            TypeDecl::Record(decl) => decl.node_id(db),
            TypeDecl::UnitStruct(decl) => decl.node_id(db),
            TypeDecl::RegularStruct(decl) => decl.node_id(db),
            TypeDecl::TupleStruct(decl) => decl.node_id(db),
            TypeDecl::Structure(decl) => decl.node_id(db),
            TypeDecl::Extern(decl) => decl.node_id(db),
            TypeDecl::Union(decl) => decl.node_id(db),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        todo!()
        // match self {
        //     TypeDecl::Enum(decl) => decl.ast_idx(db),
        //     TypeDecl::UnitStruct(decl) => decl.ast_idx(db),
        //     TypeDecl::TupleStruct(decl) => decl.ast_idx(db),
        //     TypeDecl::RegularStruct(decl) => decl.ast_idx(db),
        //     TypeDecl::Record(decl) => decl.ast_idx(db),
        //     TypeDecl::Inductive(decl) => decl.ast_idx(db),
        //     TypeDecl::Structure(decl) => decl.ast_idx(db),
        //     TypeDecl::Extern(decl) => decl.ast_idx(db),
        //     TypeDecl::Union(decl) => decl.ast_idx(db),
        // }
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
}

impl HasDecl for TypePath {
    type Decl = TypeDecl;

    #[inline(always)]
    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        self.node_id(db).decl(db)
    }
}

impl HasDecl for TypeNodeId {
    type Decl = TypeDecl;

    #[inline(always)]
    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        ty_decl_aux(db, self).as_ref().copied()
    }
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn ty_decl_aux(db: &dyn DeclDb, id: TypeNodeId) -> DeclResult<TypeDecl> {
    DeclParseContext::new(db, id.module_path(db))?.parse_ty_decl(id)
}

impl<'a> DeclParseContext<'a> {
    fn parse_ty_decl(&self, node_id: TypeNodeId) -> DeclResult<TypeDecl> {
        let db = self.db();
        let node = node_id.node(db).expect("should correspond to valid node");
        let ast_idx: AstIdx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                block: DefnBlock::Type { path, variants },
                entity_kind,
                saved_stream_state,
                ..
            } => self.parse_ty_decl_aux(
                node_id,
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

    fn parse_ty_decl_aux(
        &self,
        node_id: TypeNodeId,
        ast_idx: AstIdx,
        type_kind: TypeKind,
        _entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        variants: Option<TypeVariants>,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TypeDecl> {
        match type_kind {
            TypeKind::Enum => self.parse_enum_ty_decl(
                node_id,
                ast_idx,
                token_group_idx,
                variants.expect("guaranteed by `husky-ast`"),
                saved_stream_state,
            ),
            TypeKind::Inductive => self.parse_inductive_ty_decl(
                ast_idx,
                node_id,
                token_group_idx,
                variants.expect("guaranteed by `husky-ast`"),
                saved_stream_state,
            ),
            TypeKind::Record => todo!(),
            TypeKind::Struct => {
                debug_assert!(variants.is_none());
                self.parse_struct_ty_decl(node_id, ast_idx, token_group_idx, saved_stream_state)
            }
            TypeKind::Structure => {
                debug_assert!(variants.is_none());
                self.parse_structure_ty_decl(node_id, ast_idx, token_group_idx, saved_stream_state)
            }
            TypeKind::Extern => {
                debug_assert!(variants.is_none());
                self.parse_extern_ty_decl(node_id, ast_idx, token_group_idx, saved_stream_state)
            }
        }
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_struct_ty_decl(
        &self,
        node_id: TypeNodeId,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TypeDecl> {
        let db = self.db();
        let mut parser = self.expr_parser(node_id, None, AllowSelfType::True, AllowSelfValue::True);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.parse()?;
        if let Some(lcurl) = ctx.parse::<LeftCurlyBraceToken>()? {
            let (parameters, commas) = parse_separated_list2(&mut ctx, |e| e)?;
            let rcurl = ctx.parse_expected(OriginalDeclExprError::ExpectedRightCurlyBrace)?;
            Ok(RegularStructTypeDecl::new(
                db,
                node_id,
                implicit_parameters,
                lcurl,
                (parameters, commas),
                rcurl,
                parser.finish(),
            )
            .into())
        } else if let Some(lpar) = ctx.parse::<LeftParenthesisToken>()? {
            let (parameters, commas) = parse_separated_list2(&mut ctx, |e| e)?;
            let rpar = ctx.parse_expected(
                OriginalDeclExprError::ExpectedRightParenthesisInTupleStructFieldTypeList,
            )?;
            Ok(TupleStructTypeDecl::new(
                db,
                node_id,
                implicit_parameters,
                lpar,
                (parameters, commas),
                rpar,
                parser.finish(),
            )
            .into())
        } else {
            Err(OriginalDeclError::ExpectedLCurlOrLParOrSemicolon(ctx.save_state()).into())
        }
    }
}

#[test]
fn ty_decl_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let menu = db.entity_path_menu(toolchain);
    assert!(menu.never_ty_path().decl(&db).is_ok());
}
