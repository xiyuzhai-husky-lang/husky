use std::sync::Arc;

use super::*;
use ast::*;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use entity_syntax::RawTyKind;
use file::FilePtr;
use infer_total::InferQueryGroup;
use semantics_eager::{FuncStmt, ProcStmt};
use semantics_error::SemanticResult;
use semantics_lazy::LazyStmt;
use syntax_types::{EnumVariantKind, RawMembRoutineKind, RoutineKind};
use vec_map::VecDict;
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
        let mut variants = VecDict::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: raw_variant_kind,
                } => {
                    let variant_kind = match raw_variant_kind {
                        EnumVariantKind::Constant => EnumVariant::Constant,
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
        let mut memb_vars = VecDict::default();
        let mut memb_routines = VecDict::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::Use { ident, scope } => (),
                AstKind::RoutineDefnHead(_) => todo!(),
                AstKind::MembVarDefn(ref memb_var_defn) => {
                    memb_vars.insert_new(memb_var_defn.ident, memb_var_defn.clone())
                }
                AstKind::MembRoutineDefnHead(ref head) => match head.routine_kind {
                    RoutineKind::Proc => todo!(),
                    RoutineKind::Func => {
                        let stmts = semantics_eager::parse_decl_stmts(
                            &head.input_placeholders,
                            db,
                            arena,
                            subitem.children.unwrap(),
                            file,
                        )?;
                        memb_routines.insert_new(
                            head.ident,
                            MembRoutineDefn {
                                kind: MembRoutineKind::Func { stmts },
                                input_placeholders: head.input_placeholders.clone(),
                                output: head.output,
                                this_contract: head.this_contract,
                            },
                        )
                    }
                    RoutineKind::Test => todo!(),
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
        let mut memb_vars = VecDict::default();
        let mut memb_features = VecDict::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::Use { ident, scope } => (),
                AstKind::RoutineDefnHead(_) => todo!(),
                AstKind::MembVarDefn(ref memb_var_defn) => {
                    memb_vars.insert_new(memb_var_defn.ident, memb_var_defn.clone())
                }
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
        memb_vars: IdentMap<MembVarDefn>,
        memb_routines: IdentMap<MembRoutineDefn>,
    },
    Record {
        memb_vars: IdentMap<MembVarDefn>,
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
        enum_variant_class: EnumVariantKind,
        children: Option<AstIter>,
    ) -> EntityDefnVariant {
        EntityDefnVariant::EnumVariant(match enum_variant_class {
            EnumVariantKind::Constant => EnumVariant::Constant,
        })
    }
}
