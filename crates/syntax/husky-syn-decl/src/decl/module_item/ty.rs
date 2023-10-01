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
use husky_entity_kind::{EntityKind, TypeKind};
use parsec::parse_separated_list2;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TypeSynNodeDecl {
    Enum(EnumTypeSynNodeDecl),
    PropsStruct(PropsStructTypeSynNodeDecl),
    UnitStruct(UnitStructTypeSynNodeDecl),
    TupleStruct(TupleStructTypeSynNodeDecl),
    Record(RecordTypeSynNodeDecl),
    Inductive(InductiveTypeSynNodeDecl),
    Structure(StructureTypeSynNodeDecl),
    Extern(ExternTypeSynNodeDecl),
    Union(UnionTypeSynNodeDecl),
}

impl TypeSynNodeDecl {
    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> TypeSynNodePath {
        match self {
            TypeSynNodeDecl::Enum(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::Inductive(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::Record(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::UnitStruct(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::PropsStruct(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::TupleStruct(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::Structure(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::Extern(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::Union(syn_node_decl) => syn_node_decl.syn_node_path(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            TypeSynNodeDecl::Enum(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::UnitStruct(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::TupleStruct(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::PropsStruct(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::Record(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::Inductive(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::Structure(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::Extern(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::Union(syn_node_decl) => syn_node_decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        match self {
            TypeSynNodeDecl::Enum(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::PropsStruct(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::UnitStruct(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::TupleStruct(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::Record(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::Inductive(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::Structure(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::Extern(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::Union(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasSynNodeDecl for TypeSynNodePath {
    type NodeDecl = TypeSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        ty_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_node_decl(db: &dyn SynDeclDb, syn_node_path: TypeSynNodePath) -> TypeSynNodeDecl {
    DeclParser::new(db, syn_node_path).parse_ty_node_decl()
}

impl<'a> DeclParser<'a, TypeSynNodePath> {
    fn parse_ty_node_decl(&self) -> TypeSynNodeDecl {
        match self.syn_node_path().ty_kind(self.db()) {
            TypeKind::Enum => self.parse_enum_ty_node_decl().into(),
            TypeKind::Inductive => self.parse_inductive_ty_node_decl().into(),
            TypeKind::Record => todo!(),
            TypeKind::Struct => self.parse_struct_ty_node_decl(),
            TypeKind::Structure => self.parse_structure_ty_node_decl(),
            TypeKind::Extern => self.parse_extern_ty_node_decl().into(),
        }
    }
}

impl<'a> DeclParser<'a, TypeSynNodePath> {
    pub(super) fn parse_struct_ty_node_decl(&self) -> TypeSynNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(None, AllowSelfType::True, AllowSelfValue::False, None);
        let template_parameters = parser.try_parse_option();
        if let Some(lpar) = parser.try_parse_err_as_none::<LparRegionalToken>() {
            let field_comma_list = parser.try_parse();
            let rpar = parser.try_parse();
            TupleStructTypeSynNodeDecl::new(
                db,
                self.syn_node_path(),
                template_parameters,
                lpar,
                field_comma_list,
                rpar,
                parser.finish(),
            )
            .into()
        } else if let Some(semicolon) = parser.try_parse_err_as_none::<SemiColonRegionalToken>() {
            todo!()
            // Err(OriginalDeclError::ExpectedLCurlOrLParOrSemicolon(ctx.save_state()).into())
        } else {
            let lcurl = parser.try_parse();
            let field_comma_list = parser.try_parse();
            let rcurl = parser.try_parse();
            PropsStructTypeSynNodeDecl::new(
                db,
                self.syn_node_path(),
                template_parameters,
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
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TypeSynDecl {
    Enum(EnumTypeSynDecl),
    PropsStruct(PropsStructTypeSynDecl),
    UnitStruct(UnitStructTypeSynDecl),
    TupleStruct(TupleStructTypeSynDecl),
    Record(RecordTypeSynDecl),
    Inductive(InductiveTypeSynDecl),
    Structure(StructureTypeSynDecl),
    Extern(ExternTypeSynDecl),
    Union(UnionTypeSynDecl),
}

impl TypeSynDecl {
    pub fn path(self, db: &dyn SynDeclDb) -> TypePath {
        match self {
            TypeSynDecl::Enum(decl) => decl.path(db),
            TypeSynDecl::Inductive(decl) => decl.path(db),
            TypeSynDecl::Record(decl) => decl.path(db),
            TypeSynDecl::UnitStruct(decl) => decl.path(db),
            TypeSynDecl::PropsStruct(decl) => decl.path(db),
            TypeSynDecl::TupleStruct(decl) => decl.path(db),
            TypeSynDecl::Structure(decl) => decl.path(db),
            TypeSynDecl::Extern(decl) => decl.path(db),
            TypeSynDecl::Union(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [TemplateParameterObelisk] {
        match self {
            TypeSynDecl::Enum(decl) => decl.template_parameters(db),
            TypeSynDecl::UnitStruct(decl) => decl.template_parameters(db),
            TypeSynDecl::TupleStruct(decl) => decl.template_parameters(db),
            TypeSynDecl::PropsStruct(decl) => decl.template_parameters(db),
            TypeSynDecl::Record(decl) => decl.template_parameters(db),
            TypeSynDecl::Inductive(decl) => decl.template_parameters(db),
            TypeSynDecl::Structure(decl) => decl.template_parameters(db),
            TypeSynDecl::Extern(decl) => decl.template_parameters(db),
            TypeSynDecl::Union(decl) => decl.template_parameters(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            TypeSynDecl::Enum(decl) => decl.syn_expr_region(db),
            TypeSynDecl::UnitStruct(decl) => decl.syn_expr_region(db),
            TypeSynDecl::TupleStruct(decl) => decl.syn_expr_region(db),
            TypeSynDecl::PropsStruct(decl) => decl.syn_expr_region(db),
            TypeSynDecl::Record(decl) => decl.syn_expr_region(db),
            TypeSynDecl::Inductive(decl) => decl.syn_expr_region(db),
            TypeSynDecl::Structure(decl) => decl.syn_expr_region(db),
            TypeSynDecl::Extern(decl) => decl.syn_expr_region(db),
            TypeSynDecl::Union(decl) => decl.syn_expr_region(db),
        }
    }

    #[inline(always)]
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: TypeSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            TypeSynNodeDecl::Enum(syn_node_decl) => {
                EnumTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::PropsStruct(syn_node_decl) => {
                PropsStructTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::UnitStruct(syn_node_decl) => {
                UnitStructTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::TupleStruct(syn_node_decl) => {
                TupleStructTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::Record(syn_node_decl) => {
                RecordTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::Inductive(syn_node_decl) => {
                InductiveTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::Structure(syn_node_decl) => {
                StructureTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::Extern(syn_node_decl) => {
                ExternTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::Union(syn_node_decl) => {
                UnionTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
        })
    }
}

impl HasSynDecl for TypePath {
    type Decl = TypeSynDecl;

    #[inline(always)]
    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        ty_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_decl(db: &dyn SynDeclDb, path: TypePath) -> DeclResult<TypeSynDecl> {
    TypeSynDecl::from_node_decl(db, path, path.syn_node_path(db).syn_node_decl(db))
}

#[test]
fn ty_decl_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let menu = db.item_path_menu(toolchain);
    assert!(menu.never_ty_path().syn_decl(&db).is_ok());
}
