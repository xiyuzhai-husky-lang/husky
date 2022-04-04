use std::sync::Arc;

use ast::*;
use entity_syntax::RawTyKind;
use file::FilePtr;
use infer_total::InferQueryGroup;
use scope::{InputPlaceholder, RangedScope, ScopePtr};
use semantics_eager::{DeclStmt, ImprStmt};
use semantics_error::SemanticResult;
use semantics_lazy::LazyStmt;
use syntax_types::{MembAccessSignature, RawEnumVariantKind, RawMembRoutineKind, RoutineKind};
use vec_map::VecMap;
use vm::InputContract;
use word::{CustomIdentifier, IdentMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TyDefn {
    pub kind: TyDefnKind,
}

impl TyDefn {
    pub(crate) fn from_ast(
        db: &dyn InferQueryGroup,
        head: &Ast,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<TyDefn> {
        Ok(TyDefn {
            kind: match head.kind {
                AstKind::TypeDecl {
                    ident,
                    kind,
                    ref generics,
                } => match kind {
                    RawTyKind::Enum => Self::enum_from_ast(children)?,
                    RawTyKind::Struct => Self::struct_from_ast(db, children, arena, file)?,
                    RawTyKind::Record => Self::class_from_ast(db, children, arena, file)?,
                    RawTyKind::Primitive => todo!(),
                    RawTyKind::Vec => todo!(),
                    RawTyKind::Array => todo!(),
                    RawTyKind::Other => todo!(),
                },
                _ => panic!(),
            },
        })
    }

    fn enum_from_ast(children: AstIter) -> SemanticResult<TyDefnKind> {
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
        Ok(TyDefnKind::Enum { variants })
    }

    fn struct_from_ast(
        db: &dyn InferQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<TyDefnKind> {
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
                            MembRoutineDefn {
                                kind: MembRoutineKind::Func { stmts },
                                input_placeholders: memb_routine_head.input_placeholders.clone(),
                                output: memb_routine_head.output,
                                this_contract: memb_routine_head.this_contract,
                            },
                        )
                    }
                },
                _ => panic!(),
            }
        }
        Ok(TyDefnKind::Struct {
            memb_vars,
            memb_routines,
        })
    }

    fn class_from_ast(
        db: &dyn InferQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<TyDefnKind> {
        let mut memb_vars = VecMap::default();
        let mut memb_features = VecMap::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::Use { ident, scope } => (),
                AstKind::RoutineDecl {
                    ref routine_kind,
                    ref routine_head,
                } => todo!(),
                AstKind::MembVar { ident, signature } => memb_vars.insert_new(ident, signature),
                AstKind::MembFeatureDecl { ident, ty } => {
                    let stmts = semantics_lazy::parse_lazy_stmts(
                        &[],
                        db,
                        arena,
                        subitem.children.unwrap(),
                        file,
                    )?;
                    memb_features.insert_new(ident, MembFeatureDefn { ty, stmts });
                }
                _ => panic!(),
            }
        }
        Ok(TyDefnKind::Record {
            memb_vars,
            memb_features,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TyDefnKind {
    Enum {
        variants: IdentMap<EnumVariantKind>,
    },
    Struct {
        memb_vars: IdentMap<MembAccessSignature>,
        memb_routines: IdentMap<MembRoutineDefn>,
    },
    Record {
        memb_vars: IdentMap<MembAccessSignature>,
        memb_features: IdentMap<MembFeatureDefn>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembRoutineDefn {
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedScope,
    pub kind: MembRoutineKind,
    pub this_contract: InputContract,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembFeatureDefn {
    pub ty: ScopePtr,
    pub stmts: Arc<Vec<Arc<LazyStmt>>>,
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
