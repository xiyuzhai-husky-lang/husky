use std::sync::Arc;

use super::*;
use ast::*;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use entity_syntax::TyKind;
use file::FilePtr;
use infer_total::InferQueryGroup;
use semantics_eager::{FuncStmt, ProcStmt};
use semantics_error::SemanticResult;
use semantics_lazy::LazyStmt;
use syntax_types::{EnumVariantKind, RawMembRoutineKind, RoutineKind};
use vec_map::{HasKey, VecDict};
use vm::{InputContract, MembAccessContract};
use word::{CustomIdentifier, IdentDict};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TyDefn {
    pub fields: IdentDict<FieldDefn>,
    pub methods: IdentDict<MethodDefn>,
    pub variants: IdentDict<EnumVariantDefn>,
    pub kind: TyDefnKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyDefnKind {
    Enum,
    Struct,
    Record,
}

impl TyDefn {
    pub(crate) fn from_ast(
        db: &dyn InferQueryGroup,
        head: &Ast,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<TyDefn> {
        todo!()
        // Ok(TyDefn {
        //     kind: match head.kind {
        //         AstKind::TypeDefnHead {
        //             ident,
        //             kind,
        //             generic_placeholders: ref generics,
        //         } => match kind {
        //             RawTyKind::Enum => Self::enum_from_ast(children)?,
        //             RawTyKind::Struct => Self::struct_from_ast(db, children, arena, file)?,
        //             RawTyKind::Record => Self::record_from_ast(db, children, arena, file)?,
        //             RawTyKind::Primitive => todo!(),
        //             RawTyKind::Vec => todo!(),
        //             RawTyKind::Array => todo!(),
        //             RawTyKind::Other => todo!(),
        //         },
        //         _ => panic!(),
        //     },
        // })
    }

    fn enum_from_ast(children: AstIter) -> SemanticResult<TyDefn> {
        let mut variants = VecDict::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: raw_variant_kind,
                } => {
                    variants.insert_new(EnumVariantDefn {
                        ident,
                        variant: match raw_variant_kind {
                            EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
                        },
                    });
                }
                _ => panic!(),
            }
        }
        todo!()
        // Ok(TyDefnKind::Enum { variants })
    }

    fn enum_variant_from_ast(children: Option<AstIter>) -> SemanticResult<EnumVariantDefn> {
        todo!()
    }

    fn struct_from_ast(
        db: &dyn InferQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<TyDefn> {
        todo!()
        // let mut field_vars = VecDict::default();
        // let mut field_routines = VecDict::default();
        // for subitem in children {
        //     match subitem.value.as_ref()?.kind {
        //         AstKind::Use { ident, scope } => (),
        //         AstKind::RoutineDefnHead(_) => todo!(),
        //         AstKind::FieldDefn(ref field_var_defn) => {
        //             field_vars.insert_new(field_var_defn.clone())
        //         }
        //         AstKind::MembRoutineDefnHead(ref head) => match head.routine_kind {
        //             RoutineKind::Proc => todo!(),
        //             RoutineKind::Func => {
        //                 let stmts = semantics_eager::parse_decl_stmts(
        //                     &head.input_placeholders,
        //                     db,
        //                     arena,
        //                     subitem.children.unwrap(),
        //                     file,
        //                 )?;
        //                 field_routines.insert_new(MethodDefn {
        //                     ident: head.ident,
        //                     kind: MethodKind::Func { stmts },
        //                     input_placeholders: head.input_placeholders.clone(),
        //                     output: head.output,
        //                     this_contract: head.this_contract,
        //                 })
        //             }
        //             RoutineKind::Test => todo!(),
        //         },
        //         _ => panic!(),
        //     }
        // }
        // Ok(TyDefnKind::Struct {
        //     fields: field_vars,
        //     methods: field_routines,
        // })
    }

    fn record_from_ast(
        db: &dyn InferQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<TyDefn> {
        todo!()
        // let mut fields = VecDict::default();
        // for subitem in children {
        //     match subitem.value.as_ref()?.kind {
        //         AstKind::Use { ident, scope } => (),
        //         AstKind::RoutineDefnHead(_) => todo!(),
        //         AstKind::FieldDefn(ref field_var_defn) => fields.insert_new(field_var_defn.clone()),
        //         AstKind::MembFeatureDefnHead { ident, ty } => {
        //             let stmts = semantics_lazy::parse_lazy_stmts(
        //                 &[],
        //                 db,
        //                 arena,
        //                 subitem.children.unwrap(),
        //                 file,
        //             )?;
        //             fields.insert_new(FieldDefn {
        //                 ident,
        //                 output_ty: ty,
        //                 stmts,
        //             });
        //         }
        //         _ => panic!(),
        //     }
        // }
        // Ok(TyDefnKind::Record { fields })
    }
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub enum TyDefnKind {
//     Enum {
//         variants: IdentDict<EnumVariantDefn>,
//     },
//     Struct {
//         fields: IdentDict<FieldDefnHead>,
//         methods: IdentDict<MethodDefn>,
//     },
//     Record {
//         fields: IdentDict<FieldDefnHead>,
//         field_features: IdentDict<DerivedFieldDefn>,
//     },
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MethodDefn {
    pub ident: CustomIdentifier,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedEntityRoute,
    pub kind: MethodKind,
    pub this_contract: InputContract,
}

impl HasKey<CustomIdentifier> for MethodDefn {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FieldDefn {
    pub ident: CustomIdentifier,
    pub ty: EntityRoutePtr,
    pub variant: FieldDefnVariant,
}

impl FieldDefn {
    pub fn contract(&self) -> MembAccessContract {
        match self.variant {
            FieldDefnVariant::OriginalStructField { contract } => contract,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldDefnVariant {
    OriginalStructField {
        contract: MembAccessContract,
    },
    OriginalRecordField,
    Derived {
        opt_stmts: Option<Arc<Vec<Arc<LazyStmt>>>>,
    },
}

impl HasKey<CustomIdentifier> for FieldDefn {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MethodKind {
    Func { stmts: Arc<Vec<Arc<FuncStmt>>> },
    Proc { stmts: Arc<Vec<Arc<ProcStmt>>> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumVariantDefn {
    pub ident: CustomIdentifier,
    pub variant: EnumVariantDefnVariant,
}

impl HasKey<CustomIdentifier> for EnumVariantDefn {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantDefnVariant {
    Constant,
}

impl EntityDefnVariant {
    pub fn enum_variant(
        db: &dyn EntityQueryGroup,
        ident: CustomIdentifier,
        enum_variant_kind: EnumVariantKind,
        children: Option<AstIter>,
    ) -> EntityDefnVariant {
        EntityDefnVariant::EnumVariant(EnumVariantDefn {
            ident,
            variant: match enum_variant_kind {
                EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
            },
        })
    }
}
