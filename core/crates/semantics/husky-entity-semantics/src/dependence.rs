use husky_ast::{CasePattern, CasePatternVariant};
use husky_entity_route::EntityRouteKind;
use print_utils::emsg_once;
use semantics_error::*;
use vec_map::{VecMap, VecPairMap};

use crate::*;

pub type DependeeMap = VecMap<EntityRoutePtr, (EntityRoutePtr, EntityDefnUid)>;

pub struct DependeeMapBuilder<'a> {
    db: &'a dyn EntityDefnQueryGroup,
    map: VecMap<EntityRoutePtr, (EntityRoutePtr, EntityDefnUid)>,
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
            EntityRouteKind::Root { ident, .. } => {
                if ident == RootIdentifier::Ref {
                    self.push(entity_route.deref_route());
                    return;
                } else {
                    if entity_route.spatial_arguments.len() == 0 {
                        return;
                    }
                }
            }
            EntityRouteKind::Input { main } => return,
            EntityRouteKind::Package { main, ident } => todo!(),
            EntityRouteKind::Child { parent, ident } => {
                emsg_once!("dependences on entity from external packs should be merged");
                ()
            }
            EntityRouteKind::Generic { ident, entity_kind } => todo!(),
            EntityRouteKind::ThisType => todo!(),
            EntityRouteKind::TypeAsTraitMember {
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
    ) -> VecPairMap<EntityRoutePtr, EntityDefnUid> {
        let mut builder = DependeeMapBuilder::new(db);
        match self.variant {
            EntityDefnVariant::Module {
                ref module_items, ..
            } => {
                for item in module_items.iter() {
                    builder.push(item.base_route)
                }
            }
            EntityDefnVariant::Feature { ty, ref defn_repr } => {
                builder.push(ty.route);
                extract_defn_repr_dependees(defn_repr, &mut builder);
            }
            EntityDefnVariant::Func {
                ref parameters,
                output,
                ref stmts,
                ..
            } => {
                extract_call_head_dependees(parameters, output, &mut builder);
                extract_func_stmts_dependees(stmts, &mut builder);
            }
            EntityDefnVariant::Proc {
                parameters: ref parameters,
                output,
                ref stmts,
                ..
            } => {
                extract_call_head_dependees(parameters, output, &mut builder);
                extract_proc_stmts_dependees(stmts, &mut builder);
            }
            EntityDefnVariant::Ty {
                ref ty_members,
                ref variants,
                kind,
                ref trait_impls,
                ref members,
                ..
            } => {
                ty_members.iter().for_each(|member| match member.variant {
                    EntityDefnVariant::TyField { ty, .. } => builder.push(ty),
                    _ => (),
                });
                variants.iter().for_each(|enum_variant| {
                    extract_enum_variant_dependees(enum_variant, &mut builder)
                });
            }
            EntityDefnVariant::Main(_) => todo!(),
            EntityDefnVariant::Builtin => (),
            EntityDefnVariant::EnumVariant { ref variant, .. } => match variant {
                EnumVariantDefnVariant::Constant => (),
            },
            EntityDefnVariant::TyField {
                ty,
                ref field_variant,
                ..
            } => {
                builder.push(ty);
                match field_variant {
                    FieldDefnVariant::StructOriginal => todo!(),
                    FieldDefnVariant::RecordOriginal => todo!(),
                    FieldDefnVariant::StructDerivedLazy { defn_repr } => {
                        extract_defn_repr_dependees(defn_repr, &mut builder)
                    }
                    FieldDefnVariant::RecordDerived { defn_repr } => {
                        extract_defn_repr_dependees(defn_repr, &mut builder)
                    }
                    FieldDefnVariant::StructDefault { default } => todo!(),
                    FieldDefnVariant::StructDerivedEager { derivation: value } => todo!(),
                }
            }
            EntityDefnVariant::Method {
                parameters: ref parameters,
                output_ty,
                // ref method_variant,
                ref opt_source,
                ..
            } => {
                extract_call_head_dependees(parameters, output_ty, &mut builder);
                // let opt_source = match method_variant {
                //     MethodDefnKind::TypeMethod { ty, method_source } => {
                //         builder.push(*ty);
                //         Some(method_source)
                //     }
                //     MethodDefnKind::TraitMethod {
                //         trai,
                //         opt_default_source,
                //     } => todo!(),
                //     MethodDefnKind::TraitMethodImpl { trai, opt_source } => todo!(),
                // };
                if let Some(source) = opt_source {
                    match source {
                        CallFormSource::Func { stmts } => {
                            extract_func_stmts_dependees(stmts, &mut builder)
                        }
                        CallFormSource::Proc { stmts } => {
                            extract_proc_stmts_dependees(stmts, &mut builder)
                        }
                        CallFormSource::Lazy { stmts } => {
                            extract_lazy_stmts_dependees(stmts, &mut builder)
                        }
                        CallFormSource::Static(_) => todo!(),
                    }
                }
            }
            EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            EntityDefnVariant::Trait {
                ref generic_parameters,
                ref members,
            } => todo!(),
            EntityDefnVariant::Function {
                ref spatial_parameters,
                ref parameters,
                output,
                ref source,
            } => todo!(),
        };
        return builder.finish();

        fn extract_call_head_dependees(
            inputs: &[Parameter],
            output: RangedEntityRoute,
            builder: &mut DependeeMapBuilder,
        ) {
            for input_placeholder in inputs.iter() {
                builder.push(input_placeholder.ranged_ty.route)
            }
            builder.push(output.route);
        }

        fn extract_defn_repr_dependees(
            defn_repr: &DefinitionRepr,
            builder: &mut DependeeMapBuilder,
        ) {
            match defn_repr {
                DefinitionRepr::LazyExpr { ref expr } => extract_lazy_expr_dependees(expr, builder),
                DefinitionRepr::LazyBlock { stmts, ty } => {
                    extract_lazy_stmts_dependees(stmts, builder)
                }
                DefinitionRepr::FuncBlock {
                    stmts,
                    file,
                    range,
                    route,
                    ..
                } => {
                    builder.push(*route);
                    extract_func_stmts_dependees(stmts, builder)
                }
                DefinitionRepr::ProcBlock {
                    stmts, file, range, ..
                } => extract_proc_stmts_dependees(stmts, builder),
            }
        }

        fn extract_lazy_stmts_dependees(stmts: &[Arc<LazyStmt>], builder: &mut DependeeMapBuilder) {
            for stmt in stmts {
                match stmt.variant {
                    LazyStmtVariant::Init { varname, ref value } => todo!(),
                    LazyStmtVariant::Assert { ref condition } => todo!(),
                    LazyStmtVariant::Return { ref result } => {
                        extract_lazy_expr_dependees(result, builder)
                    }
                    LazyStmtVariant::ConditionFlow { .. } => todo!(),
                    LazyStmtVariant::Match { .. } => todo!(),
                    LazyStmtVariant::ReturnXml { ref xml_expr } => todo!(),
                }
            }
        }

        fn extract_func_stmts_dependees(stmts: &[Arc<FuncStmt>], builder: &mut DependeeMapBuilder) {
            for stmt in stmts {
                match stmt.variant {
                    FuncStmtVariant::Init {
                        varname,
                        initial_value: ref value,
                    } => extract_eager_expr_dependees(value, builder),
                    FuncStmtVariant::Assert { ref condition } => {
                        extract_eager_expr_dependees(condition, builder)
                    }
                    FuncStmtVariant::Return { ref result } => {
                        extract_eager_expr_dependees(result, builder)
                    }
                    FuncStmtVariant::ConditionFlow { ref branches } => {
                        for branch in branches {
                            extract_func_stmts_dependees(&branch.stmts, builder)
                        }
                    }
                    FuncStmtVariant::Match {
                        ref match_expr,
                        ref branches,
                    } => {
                        extract_eager_expr_dependees(match_expr, builder);
                        for branch in branches {
                            extract_func_pattern_branch_dependees(branch, builder)
                        }
                    }
                }
            }
        }

        fn extract_proc_stmts_dependees(stmts: &[Arc<ProcStmt>], builder: &mut DependeeMapBuilder) {
            for stmt in stmts {
                match stmt.variant {
                    ProcStmtVariant::Init {
                        varname,
                        ref initial_value,
                        ..
                    } => extract_eager_expr_dependees(initial_value, builder),
                    ProcStmtVariant::Assert { ref condition } => {
                        extract_eager_expr_dependees(condition, builder)
                    }
                    ProcStmtVariant::Return { ref result } => {
                        extract_eager_expr_dependees(result, builder)
                    }
                    ProcStmtVariant::Execute { ref expr } => {
                        extract_eager_expr_dependees(expr, builder)
                    }
                    ProcStmtVariant::ConditionFlow { ref branches } => {
                        for branch in branches {
                            extract_proc_condition_branch_dependees(branch, builder)
                        }
                    }
                    ProcStmtVariant::Loop {
                        loop_variant: ref loop_kind,
                        ref stmts,
                    } => {
                        match loop_kind {
                            LoopVariant::For {
                                ref initial_boundary,
                                ref final_boundary,
                                ..
                            } => {
                                extract_boundary_dependees(initial_boundary, builder);
                                extract_boundary_dependees(final_boundary, builder);
                            }
                            LoopVariant::ForExt {
                                ref final_boundary, ..
                            } => {
                                extract_boundary_dependees(final_boundary, builder);
                            }
                            LoopVariant::While { condition } => {
                                extract_eager_expr_dependees(condition, builder)
                            }
                            LoopVariant::DoWhile { condition } => {
                                extract_eager_expr_dependees(condition, builder)
                            }
                        }
                        extract_proc_stmts_dependees(stmts, builder)
                    }
                    ProcStmtVariant::Break => (),
                    ProcStmtVariant::Match {
                        ref match_expr,
                        ref branches,
                    } => {
                        extract_eager_expr_dependees(match_expr, builder);
                        for branch in branches {
                            extract_proc_pattern_branch_dependees(branch, builder)
                        }
                    }
                }
            }
        }

        fn extract_lazy_expr_dependees(expr: &LazyExpr, builder: &mut DependeeMapBuilder) {
            match expr.variant {
                LazyExprVariant::Variable { .. } | LazyExprVariant::PrimitiveLiteral(_) => (),
                LazyExprVariant::EntityRoute { route: scope, .. } => builder.push(scope),
                LazyExprVariant::EnumLiteral { .. } => todo!(),
                LazyExprVariant::Bracketed(_) => todo!(),
                LazyExprVariant::Opn { opn_kind, ref opds } => {
                    match opn_kind {
                        LazyOpnKind::Binary { .. }
                        | LazyOpnKind::Prefix(_)
                        | LazyOpnKind::FieldAccess { .. }
                        | LazyOpnKind::MethodCall { .. } => (),
                        LazyOpnKind::FunctionModelCall(routine) => builder.push(routine.route),
                        LazyOpnKind::FunctionRoutineCall(routine) => builder.push(routine.route),
                        LazyOpnKind::StructCall(ty) => builder.push(ty.route),
                        LazyOpnKind::RecordCall(ty) => builder.push(ty.route),
                        LazyOpnKind::ElementAccess { .. } => todo!(),
                    }
                    for opd in opds {
                        extract_lazy_expr_dependees(opd, builder)
                    }
                }
                LazyExprVariant::Lambda(_, _) => todo!(),
                LazyExprVariant::ThisValue { .. } => (),
                LazyExprVariant::EntityFeature {
                    entity_route: route,
                } => builder.push(route),
                LazyExprVariant::ThisField {
                    field_ident,
                    this_ty,
                    this_binding,
                    field_binding,
                } => todo!(),
            }
        }

        fn extract_eager_expr_dependees(expr: &EagerExpr, builder: &mut DependeeMapBuilder) {
            match expr.variant {
                EagerExprVariant::Variable { .. } => (),
                EagerExprVariant::EntityRoute { route } => builder.push(route),
                EagerExprVariant::PrimitiveLiteral(_) => (),
                EagerExprVariant::Bracketed(ref expr) => {
                    extract_eager_expr_dependees(expr, builder)
                }
                EagerExprVariant::Opn {
                    opn_variant: ref opn_kind,
                    ref opds,
                    ..
                } => {
                    match opn_kind {
                        EagerOpnVariant::Binary { .. }
                        | EagerOpnVariant::Prefix { .. }
                        | EagerOpnVariant::Suffix { .. }
                        | EagerOpnVariant::FieldAccess { .. }
                        | EagerOpnVariant::MethodCall { .. }
                        | EagerOpnVariant::Index { .. } => (),
                        EagerOpnVariant::RoutineCall(routine) => builder.push(routine.route),
                        EagerOpnVariant::TypeCall { ranged_ty, .. } => {
                            builder.push(ranged_ty.route)
                        }
                    }
                    for opd in opds {
                        extract_eager_expr_dependees(opd, builder)
                    }
                }
                EagerExprVariant::Lambda(_, _) => todo!(),
                EagerExprVariant::ThisValue { .. } => builder.push(expr.ty()),
                EagerExprVariant::ThisField { this_ty, .. } => builder.push(this_ty),
                EagerExprVariant::EnumKindLiteral(_) => builder.push(expr.ty()),
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
                EntityDefnVariant::EnumVariant { ref variant, .. } => match variant {
                    EnumVariantDefnVariant::Constant => (),
                },
                _ => panic!(),
            }
        }

        fn extract_member_dependees(member_defn: &EntityDefn, builder: &mut DependeeMapBuilder) {
            match member_defn.variant {
                EntityDefnVariant::Main(_) => todo!(),
                EntityDefnVariant::Module { .. } => panic!("shouldn't be here"),
                EntityDefnVariant::Feature { ty, ref defn_repr } => todo!(),
                EntityDefnVariant::Func { .. } => todo!(),
                EntityDefnVariant::Proc { .. } => todo!(),
                EntityDefnVariant::Ty { .. } => todo!(),
                EntityDefnVariant::EnumVariant { .. } => todo!(),
                EntityDefnVariant::Builtin => todo!(),
                EntityDefnVariant::TyField {
                    ty,
                    field_variant: ref field_variant,
                    liason: contract,
                    ..
                } => todo!(),
                EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => todo!(),
                EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
                EntityDefnVariant::Method { .. } => todo!(),
                EntityDefnVariant::Trait {
                    ref generic_parameters,
                    ref members,
                } => todo!(),
                EntityDefnVariant::Function {
                    ref spatial_parameters,
                    ref parameters,
                    output,
                    ref source,
                } => todo!(),
            }
        }

        fn extract_method_dependees(method_defn: &EntityDefn, builder: &mut DependeeMapBuilder) {
            todo!()
            // for input_placeholder in method_defn.parameters.iter() {
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

        fn extract_proc_condition_branch_dependees(
            branch: &ProcConditionFlowBranch,
            builder: &mut DependeeMapBuilder,
        ) {
            match branch.variant {
                ProcConditionBranchVariant::If { ref condition } => {
                    extract_eager_expr_dependees(condition, builder)
                }
                ProcConditionBranchVariant::Elif { ref condition } => {
                    extract_eager_expr_dependees(condition, builder)
                }
                ProcConditionBranchVariant::Else => (),
            }
            extract_proc_stmts_dependees(&branch.stmts, builder)
        }

        fn extract_proc_pattern_branch_dependees(
            branch: &ProcPatternBranch,
            builder: &mut DependeeMapBuilder,
        ) {
            match branch.variant {
                ProcPatternBranchVariant::Case { ref pattern } => {
                    extract_case_pattern_dependees(pattern, builder)
                }
                ProcPatternBranchVariant::Default => (),
            }
            extract_proc_stmts_dependees(&branch.stmts, builder)
        }

        fn extract_func_condition_branch_dependees(
            branch: &FuncConditionFlowBranch,
            builder: &mut DependeeMapBuilder,
        ) {
            match branch.variant {
                FuncConditionFlowBranchVariant::If { ref condition } => {
                    extract_eager_expr_dependees(condition, builder)
                }
                FuncConditionFlowBranchVariant::Elif { ref condition } => {
                    extract_eager_expr_dependees(condition, builder)
                }
                FuncConditionFlowBranchVariant::Else => todo!(),
            }
            extract_func_stmts_dependees(&branch.stmts, builder)
        }

        fn extract_func_pattern_branch_dependees(
            branch: &FuncPatternBranch,
            builder: &mut DependeeMapBuilder,
        ) {
            match branch.variant {
                FuncPatternBranchVariant::Case { ref pattern } => {
                    extract_case_pattern_dependees(pattern, builder)
                }
                FuncPatternBranchVariant::Default => todo!(),
            }
            extract_func_stmts_dependees(&branch.stmts, builder)
        }

        fn extract_case_pattern_dependees(pattern: &CasePattern, builder: &mut DependeeMapBuilder) {
            builder.push(pattern.ty)
        }
    }
}
