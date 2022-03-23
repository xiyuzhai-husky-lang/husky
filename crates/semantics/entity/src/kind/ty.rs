use std::sync::Arc;

use ast::*;
use file::FilePtr;
use infer_total::InferQueryGroup;
use scope::{InputPlaceholder, RangedScope};
use semantics_eager::{DeclStmt, ImprStmt};
use semantics_error::SemanticResult;
use syntax_types::{
    MembVarSignature, RawEnumVariantKind, RawMembRoutineKind, RawTyKind, RoutineKind,
};
use vec_map::VecMap;
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ty {
    pub kind: TyKind,
}

impl Ty {
    pub(crate) fn from_ast(
        db: &dyn InferQueryGroup,
        head: &Ast,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<Ty> {
        Ok(Ty {
            kind: match head.kind {
                AstKind::TypeDef {
                    ident,
                    kind,
                    ref generics,
                } => match kind {
                    RawTyKind::Enum => Self::enum_from_ast(children)?,
                    RawTyKind::Struct => Self::struct_from_ast(db, children, arena, file)?,
                },
                _ => panic!(),
            },
        })
    }

    fn enum_from_ast(children: AstIter) -> SemanticResult<TyKind> {
        let mut variants = VecMap::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::EnumVariant {
                    ident,
                    raw_variant_kind,
                } => {
                    let variant_kind = match raw_variant_kind {
                        RawEnumVariantKind::Constant => EnumVariantKind::Constant,
                    };
                    variants.insert_new(ident, variant_kind);
                }
                _ => panic!(),
            }
        }
        Ok(TyKind::Enum { variants })
    }

    fn struct_from_ast(
        db: &dyn InferQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<TyKind> {
        let mut memb_vars = VecMap::default();
        let mut memb_routines = VecMap::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::Use { ident, scope } => (),
                AstKind::RoutineDecl {
                    ref routine_kind,
                    ref routine_head,
                } => todo!(),
                AstKind::MembVar { ident, signature } => memb_vars.insert_new(ident, signature),
                AstKind::MembRoutineDecl {
                    ref memb_routine_head,
                    ..
                } => match memb_routine_head.kind {
                    RawMembRoutineKind::Proc => todo!(),
                    RawMembRoutineKind::Func => {
                        let stmts = semantics_eager::parse_decl_stmts(
                            &memb_routine_head.input_placeholders,
                            db,
                            arena,
                            subitem.children.unwrap(),
                            file,
                        )?;
                        memb_routines.insert_new(
                            memb_routine_head.routine_name,
                            MembRoutine {
                                kind: MembRoutineKind::Func { stmts },
                                input_placeholders: memb_routine_head.input_placeholders.clone(),
                                output: memb_routine_head.output,
                            },
                        )
                    }
                },
                _ => panic!(),
            }
        }
        Ok(TyKind::Struct {
            memb_vars,
            memb_routines,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TyKind {
    Enum {
        variants: VecMap<CustomIdentifier, EnumVariantKind>,
    },
    Struct {
        memb_vars: VecMap<CustomIdentifier, MembVarSignature>,
        memb_routines: VecMap<CustomIdentifier, MembRoutine>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembRoutine {
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedScope,
    pub kind: MembRoutineKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MembRoutineKind {
    Func { stmts: Arc<Vec<Arc<DeclStmt>>> },
    Proc { stmts: Arc<Vec<Arc<ImprStmt>>> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantKind {
    Constant,
}
