use std::sync::Arc;

use super::*;
use ast::*;
use entity_route::{EntityRoutePtr, InputPlaceholder, RangedEntityRoute};
use entity_syntax::RawTyKind;
use file::FilePtr;
use infer_total::InferQueryGroup;
use semantics_eager::{FuncStmt, ProcStmt};
use semantics_error::SemanticResult;
use semantics_lazy::LazyStmt;
use syntax_types::{EnumVariantClass, MembAccessDecl, RawMembRoutineKind, RoutineClass};
use vec_map::VecMap;
use vm::InputContract;
use word::{CustomIdentifier, IdentMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TyDefn {
    pub kind: TyDefnVariant,
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
                AstKind::TypeDefnHead {
                    ident,
                    kind,
                    generic_placeholders: ref generics,
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

    fn enum_from_ast(children: AstIter) -> SemanticResult<TyDefnVariant> {
        let mut variants = VecMap::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: raw_variant_kind,
                } => {
                    let variant_kind = match raw_variant_kind {
                        EnumVariantClass::Constant => EnumVariant::Constant,
                    };
                    variants.insert_new(ident, variant_kind);
                }
                _ => panic!(),
            }
        }
        Ok(TyDefnVariant::Enum { variants })
    }

    fn enum_variant_from_ast(children: Option<AstIter>) -> SemanticResult<EnumVariant> {
        todo!()
    }

    fn struct_from_ast(
        db: &dyn InferQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<TyDefnVariant> {
        let mut memb_vars = VecMap::default();
        let mut memb_routines = VecMap::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::Use { ident, scope } => (),
                AstKind::RoutineDefnHead {
                    routine_class: ref routine_kind,
                    ref routine_head,
                } => todo!(),
                AstKind::MembVarDefn { ident, signature } => memb_vars.insert_new(ident, signature),
                AstKind::MembRoutineDefnHead {
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
                            memb_routine_head.ident,
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
        Ok(TyDefnVariant::Struct {
            memb_vars,
            memb_routines,
        })
    }

    fn class_from_ast(
        db: &dyn InferQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<TyDefnVariant> {
        let mut memb_vars = VecMap::default();
        let mut memb_features = VecMap::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::Use { ident, scope } => (),
                AstKind::RoutineDefnHead {
                    routine_class: ref routine_kind,
                    ref routine_head,
                } => todo!(),
                AstKind::MembVarDefn { ident, signature } => memb_vars.insert_new(ident, signature),
                AstKind::MembFeatureDefnHead { ident, ty } => {
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
        Ok(TyDefnVariant::Record {
            memb_vars,
            memb_features,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TyDefnVariant {
    Enum {
        variants: IdentMap<EnumVariant>,
    },
    Struct {
        memb_vars: IdentMap<MembAccessDecl>,
        memb_routines: IdentMap<MembRoutineDefn>,
    },
    Record {
        memb_vars: IdentMap<MembAccessDecl>,
        memb_features: IdentMap<MembFeatureDefn>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembRoutineDefn {
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedEntityRoute,
    pub kind: MembRoutineKind,
    pub this_contract: InputContract,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembFeatureDefn {
    pub ty: EntityRoutePtr,
    pub stmts: Arc<Vec<Arc<LazyStmt>>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MembRoutineKind {
    Func { stmts: Arc<Vec<Arc<FuncStmt>>> },
    Proc { stmts: Arc<Vec<Arc<ProcStmt>>> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariant {
    Constant,
}

impl EntityDefnVariant {
    pub fn enum_variant(
        db: &dyn EntityQueryGroup,
        enum_variant_class: EnumVariantClass,
        children: Option<AstIter>,
    ) -> EntityDefnVariant {
        EntityDefnVariant::EnumVariant(match enum_variant_class {
            EnumVariantClass::Constant => EnumVariant::Constant,
        })
    }
}
