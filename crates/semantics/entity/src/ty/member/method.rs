use avec::Avec;
use entity_route::EntityRouteKind;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MethodDefnVariant {
    Func {
        stmts: Avec<FuncStmt>,
    },
    Proc {
        stmts: Avec<ProcStmt>,
    },
    Pattern {
        stmts: Avec<LazyStmt>,
    },
    StaticMemberAccess {
        ref_access: Linkage,
        move_access: Linkage,
        borrow_mut_access: Linkage,
    },
}

impl EntityDefnVariant {
    pub(crate) fn collect_other_members(
        db: &dyn InferQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        ty_route: EntityRoutePtr,
        mut children: Peekable<AstIter>,
        members: &mut IdentDict<Arc<EntityDefn>>,
    ) -> SemanticResult<()> {
        while let Some(child) = children.next() {
            let ast = child.value.as_ref()?;
            match ast.kind {
                AstKind::TypeDefnHead {
                    ident,
                    kind,
                    ref generic_placeholders,
                } => todo!(),
                AstKind::MainDefn => todo!(),
                AstKind::RoutineDefnHead(_) => todo!(),
                AstKind::PatternDefnHead => todo!(),
                AstKind::FeatureDecl { ident, ty } => todo!(),
                AstKind::TypeMethodDefnHead(ref head) => {
                    let method_variant = match head.routine_kind {
                        RoutineKind::Proc => todo!(),
                        RoutineKind::Func => {
                            let stmts = semantics_eager::parse_decl_stmts(
                                &head.input_placeholders,
                                db,
                                arena,
                                child.children.unwrap(),
                                file,
                            )?;
                            MethodDefnVariant::Func { stmts }
                        }
                        RoutineKind::Test => todo!(),
                    };
                    members.insert_new(EntityDefn::new(
                        head.ident.into(),
                        EntityDefnVariant::TypeMethod {
                            input_placeholders: head.input_placeholders.clone(),
                            output: head.output_ty,
                            this_contract: head.this_contract,
                            method_variant,
                        },
                        db.intern_entity_route(EntityRoute {
                            kind: EntityRouteKind::Child {
                                parent: ty_route,
                                ident: head.ident,
                            },
                            generic_arguments: vec![],
                        }),
                        file,
                        ast.range,
                    ))
                }
                AstKind::Use { .. } => todo!(),
                AstKind::FieldDefnHead(_) => todo!(),
                AstKind::DatasetConfigDefnHead => todo!(),
                AstKind::Stmt(_) => todo!(),
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => todo!(),
            }
        }
        Ok(())
    }
}
