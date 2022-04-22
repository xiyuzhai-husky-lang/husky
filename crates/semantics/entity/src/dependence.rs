use entity_route::EntityRouteKind;
use print_utils::msg_once;
use semantics_error::*;
use vec_dict::VecDict;

use crate::*;

pub type DependeeMap = VecDict<EntityRoutePtr, (EntityRoutePtr, EntityDefnUid)>;

pub struct DependeeMapBuilder<'a> {
    db: &'a dyn EntityDefnQueryGroup,
    map: VecDict<EntityRoutePtr, (EntityRoutePtr, EntityDefnUid)>,
}

impl<'a> DependeeMapBuilder<'a> {
    fn new(db: &'a dyn EntityDefnQueryGroup) -> DependeeMapBuilder<'a> {
        Self {
            db,
            map: Default::default(),
        }
    }

    fn push(&mut self, entity_route: EntityRoutePtr) {
        match entity_route.kind {
            EntityRouteKind::Root { .. } => return,
            EntityRouteKind::Package { main, ident } => todo!(),
            EntityRouteKind::Child { parent, ident } => {
                msg_once!("dependences on entity from external packs should be merged");
                ()
            }
            EntityRouteKind::Input { main } => todo!(),
            EntityRouteKind::Generic {
                ident,
                entity_kind: raw_entity_kind,
            } => todo!(),
            EntityRouteKind::ThisType => todo!(),
            EntityRouteKind::TraitMember {
                ty: parent,
                trai,
                ident,
            } => todo!(),
        }
        if !self.map.has(entity_route) {
            self.map
                .insert_new((entity_route, self.db.entity_defn_uid(entity_route)));
        }
    }

    fn finish(self) -> DependeeMap {
        self.map
    }
}

pub(crate) fn entity_immediate_dependees(
    db: &dyn EntityDefnQueryGroup,
    entity_route: EntityRoutePtr,
) -> SemanticResultArc<DependeeMap> {
    let defn = db.entity_defn(entity_route)?;
    Ok(Arc::new(defn.immediate_dependees(db)))
}

pub(crate) fn entity_dependees(
    db: &dyn EntityDefnQueryGroup,
    entity_route: EntityRoutePtr,
) -> SemanticResultArc<DependeeMap> {
    let mut immediate_dependees = (*db.entity_immediate_dependees(entity_route)?).clone();
    visit_all(db, &mut immediate_dependees, 0)?;
    return Ok(Arc::new(immediate_dependees));

    fn visit_all(
        db: &dyn EntityDefnQueryGroup,
        map: &mut DependeeMap,
        start: usize,
    ) -> SemanticResult<()> {
        let len0 = map.data().len();
        for i in start..len0 {
            let (subroute, _) = map.data()[i];
            let submap = db.entity_immediate_dependees(subroute)?;
            map.extends_from_ref(&submap);
        }
        if map.data().len() > len0 {
            visit_all(db, map, len0)
        } else {
            Ok(())
        }
    }
}

impl EntityDefn {
    fn immediate_dependees(
        &self,
        db: &dyn EntityDefnQueryGroup,
    ) -> VecDict<EntityRoutePtr, (EntityRoutePtr, EntityDefnUid)> {
        let mut builder = DependeeMapBuilder::new(db);
        match self.variant {
            EntityDefnVariant::Module { .. } => Default::default(),
            EntityDefnVariant::Feature { ty, ref lazy_stmts } => {
                builder.push(ty.route);
                extract_lazy_stmts_dependees(lazy_stmts, &mut builder);
            }
            EntityDefnVariant::Pattern { .. } => todo!(),
            EntityDefnVariant::Func {
                ref input_placeholders,
                output,
                ref stmts,
            } => {
                extract_call_head_dependees(input_placeholders, output, &mut builder);
                extract_func_stmts_dependees(stmts, &mut builder);
            }
            EntityDefnVariant::Proc {
                ref input_placeholders,
                output,
                ref stmts,
            } => {
                extract_call_head_dependees(input_placeholders, output, &mut builder);
                extract_proc_stmts_dependees(stmts, &mut builder);
            }
            EntityDefnVariant::Type {
                ref type_members,
                ref variants,
                kind,
                ref trait_impls,
                ref members,
            } => {
                type_members.iter().for_each(|member| match member.variant {
                    EntityDefnVariant::TypeField { ty, .. } => builder.push(ty),
                    _ => (),
                });
                variants.iter().for_each(|enum_variant| {
                    extract_enum_variant_dependees(enum_variant, &mut builder)
                });
                // trait_impls.iter().for_each(|trait_impl| todo!())
            }
            EntityDefnVariant::Main(_) => todo!(),
            EntityDefnVariant::Builtin => (),
            EntityDefnVariant::EnumVariant { ref variant, .. } => match variant {
                EnumVariantDefnVariant::Constant => (),
            },
            EntityDefnVariant::TypeField {
                ty,
                ref field_variant,
                ..
            } => {
                builder.push(ty);
                match field_variant {
                    FieldDefnVariant::StructOriginal => todo!(),
                    FieldDefnVariant::RecordOriginal => todo!(),
                    FieldDefnVariant::StructDerived { stmts } => todo!(),
                    FieldDefnVariant::RecordDerived { stmts } => {
                        extract_lazy_stmts_dependees(stmts, &mut builder)
                    }
                }
            }
            EntityDefnVariant::Method {
                ref input_placeholders,
                output_ty,
                ref method_variant,
                ..
            } => {
                extract_call_head_dependees(input_placeholders, output_ty, &mut builder);
                todo!()
                // match method_variant {
                //     MethodSource::Func { stmts } => {
                //         extract_func_stmts_dependees(stmts, &mut builder)
                //     }
                //     MethodSource::Proc { stmts } => todo!(),
                //     MethodSource::Pattern { stmts } => todo!(),
                //     MethodSource::StaticMemberAccess { .. } => (),
                // }
            }
            EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
        };
        return builder.finish();

        fn extract_call_head_dependees(
            inputs: &[InputPlaceholder],
            output: RangedEntityRoute,
            builder: &mut DependeeMapBuilder,
        ) {
            for input_placeholder in inputs.iter() {
                builder.push(input_placeholder.ranged_ty.route)
            }
            builder.push(output.route);
        }

        fn extract_lazy_stmts_dependees(stmts: &[Arc<LazyStmt>], v: &mut DependeeMapBuilder) {
            for stmt in stmts {
                match stmt.kind {
                    LazyStmtKind::Init { varname, ref value } => todo!(),
                    LazyStmtKind::Assert { ref condition } => todo!(),
                    LazyStmtKind::Return { ref result } => extract_lazy_expr_dependees(result, v),
                    LazyStmtKind::Branches { kind, ref branches } => todo!(),
                }
            }
        }

        fn extract_func_stmts_dependees(stmts: &[Arc<FuncStmt>], v: &mut DependeeMapBuilder) {
            for stmt in stmts {
                match stmt.kind {
                    FuncStmtKind::Init {
                        varname,
                        initial_value: ref value,
                    } => extract_eager_expr_dependees(value, v),
                    FuncStmtKind::Assert { ref condition } => {
                        extract_eager_expr_dependees(condition, v)
                    }
                    FuncStmtKind::Return { ref result } => extract_eager_expr_dependees(result, v),
                    FuncStmtKind::Branches { kind, ref branches } => {
                        for branch in branches {
                            extract_func_stmts_dependees(&branch.stmts, v)
                        }
                    }
                }
            }
        }

        fn extract_proc_stmts_dependees(stmts: &[Arc<ProcStmt>], v: &mut DependeeMapBuilder) {
            for stmt in stmts {
                match stmt.kind {
                    ProcStmtKind::Init {
                        varname,
                        ref initial_value,
                        ..
                    } => extract_eager_expr_dependees(initial_value, v),
                    ProcStmtKind::Assert { ref condition } => {
                        extract_eager_expr_dependees(condition, v)
                    }
                    ProcStmtKind::Return { ref result } => extract_eager_expr_dependees(result, v),
                    ProcStmtKind::Execute { ref expr } => extract_eager_expr_dependees(expr, v),
                    ProcStmtKind::BranchGroup { kind, ref branches } => {
                        for branch in branches {
                            extract_proc_stmts_dependees(&branch.stmts, v)
                        }
                    }
                    ProcStmtKind::Loop {
                        ref loop_kind,
                        ref stmts,
                    } => {
                        match loop_kind {
                            LoopKind::For {
                                ref initial_boundary,
                                ref final_boundary,
                                ..
                            } => {
                                extract_boundary_dependees(initial_boundary, v);
                                extract_boundary_dependees(final_boundary, v);
                            }
                            LoopKind::ForExt {
                                ref final_boundary, ..
                            } => {
                                extract_boundary_dependees(final_boundary, v);
                            }
                            LoopKind::While { condition } => {
                                extract_eager_expr_dependees(condition, v)
                            }
                            LoopKind::DoWhile { condition } => {
                                extract_eager_expr_dependees(condition, v)
                            }
                        }
                        extract_proc_stmts_dependees(stmts, v)
                    }
                }
            }
        }

        fn extract_lazy_expr_dependees(expr: &LazyExpr, v: &mut DependeeMapBuilder) {
            match expr.kind {
                LazyExprKind::Variable(_) | LazyExprKind::PrimitiveLiteral(_) => (),
                LazyExprKind::Scope { scope, .. } => v.push(scope),
                LazyExprKind::EnumLiteral { scope, ref value } => todo!(),
                LazyExprKind::Bracketed(_) => todo!(),
                LazyExprKind::Opn { opn_kind, ref opds } => {
                    match opn_kind {
                        LazyOpnKind::Binary { .. }
                        | LazyOpnKind::Prefix(_)
                        | LazyOpnKind::FieldAccess { .. }
                        | LazyOpnKind::MethodCall { .. } => (),
                        LazyOpnKind::RoutineCall(routine) => v.push(routine.route),
                        LazyOpnKind::StructCall(ty) => v.push(ty.route),
                        LazyOpnKind::RecordCall(ty) => v.push(ty.route),
                        LazyOpnKind::PatternCall => todo!(),
                        LazyOpnKind::ElementAccess => todo!(),
                    }
                    for opd in opds {
                        extract_lazy_expr_dependees(opd, v)
                    }
                }
                LazyExprKind::Lambda(_, _) => todo!(),
                LazyExprKind::This => (),
                LazyExprKind::EntityFeature { route: scope } => todo!(),
            }
        }

        fn extract_eager_expr_dependees(expr: &EagerExpr, builder: &mut DependeeMapBuilder) {
            match expr.kind {
                EagerExprKind::Variable(_) => (),
                EagerExprKind::Scope { scope } => builder.push(scope),
                EagerExprKind::PrimitiveLiteral(_) => (),
                EagerExprKind::Bracketed(ref expr) => extract_eager_expr_dependees(expr, builder),
                EagerExprKind::Opn {
                    ref opn_kind,
                    ref opds,
                    ..
                } => {
                    match opn_kind {
                        EagerOpnKind::Binary { .. }
                        | EagerOpnKind::Prefix { .. }
                        | EagerOpnKind::Suffix { .. }
                        | EagerOpnKind::FieldAccess { .. }
                        | EagerOpnKind::MethodCall { .. }
                        | EagerOpnKind::ElementAccess => (),
                        EagerOpnKind::RoutineCall(routine) => builder.push(routine.route),
                        EagerOpnKind::TypeCall { ranged_ty, .. } => builder.push(ranged_ty.route),
                        EagerOpnKind::PatternCall => todo!(),
                    }
                    for opd in opds {
                        extract_eager_expr_dependees(opd, builder)
                    }
                }
                EagerExprKind::Lambda(_, _) => todo!(),
                EagerExprKind::This => builder.push(expr.ty),
            }
        }

        fn extract_boundary_dependees(boundary: &Boundary, builder: &mut DependeeMapBuilder) {
            boundary
                .opt_bound
                .as_ref()
                .map(|bound| extract_eager_expr_dependees(bound, builder));
        }

        fn extract_enum_variant_dependees(
            variant_defn: &EntityDefn,
            builder: &mut DependeeMapBuilder,
        ) {
            match variant_defn.variant {
                EntityDefnVariant::EnumVariant { .. } => todo!(),
                _ => panic!(),
            }
        }

        fn extract_member_dependees(member_defn: &EntityDefn, builder: &mut DependeeMapBuilder) {
            match member_defn.variant {
                EntityDefnVariant::Main(_) => todo!(),
                EntityDefnVariant::Module {} => todo!(),
                EntityDefnVariant::Feature { ty, ref lazy_stmts } => todo!(),
                EntityDefnVariant::Pattern {} => todo!(),
                EntityDefnVariant::Func {
                    ref input_placeholders,
                    output,
                    ref stmts,
                } => todo!(),
                EntityDefnVariant::Proc {
                    ref input_placeholders,
                    output,
                    ref stmts,
                } => todo!(),
                EntityDefnVariant::Type {
                    ref type_members,
                    ref variants,
                    kind,
                    ref trait_impls,
                    ref members,
                } => todo!(),
                EntityDefnVariant::EnumVariant { .. } => todo!(),
                EntityDefnVariant::Builtin => todo!(),
                EntityDefnVariant::TypeField {
                    ty,
                    ref field_variant,
                    contract,
                } => todo!(),
                EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => todo!(),
                EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
                EntityDefnVariant::Method { .. } => todo!(),
            }
        }

        fn extract_method_dependees(method_defn: &EntityDefn, builder: &mut DependeeMapBuilder) {
            todo!()
            // for input_placeholder in method_defn.input_placeholders.iter() {
            //     builder.push(input_placeholder.ranged_ty.route)
            // }
            // builder.push(method_defn.output.route);
            // match method_defn.variant {
            //     MethodDefnVariant::Func { ref stmts } => {
            //         extract_func_stmts_dependees(stmts, builder)
            //     }
            //     MethodDefnVariant::Proc { ref stmts } => todo!(),
            //     MethodDefnVariant::Pattern { ref stmts } => todo!(),
            // }
        }
    }
}
