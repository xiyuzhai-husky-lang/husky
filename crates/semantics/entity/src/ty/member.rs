mod field;
mod method;

pub use field::*;
pub use method::*;
use word::RoutineKeyword;

use super::*;

pub fn collect_all_members(
    type_members: &[Arc<EntityDefn>],
    trait_impls: &[Arc<TraitImplDefn>],
) -> Avec<EntityDefn> {
    let mut members = type_members.to_vec();
    for trait_impl in trait_impls {
        members.extend(
            trait_impl
                .member_impls
                .iter()
                .map(|member_impl| member_impl.clone()),
        );
    }
    Arc::new(members)
}

pub fn member_defn(db: &dyn EntityDefnQueryGroup, member_route: EntityRoutePtr) -> Arc<EntityDefn> {
    let ty = member_route.parent();
    let ty_defn = db.entity_defn(ty).unwrap();
    let member_idx = db.member_idx(member_route);
    match ty_defn.variant {
        EntityDefnVariant::Type { ref members, .. } => members[member_idx.0 as usize].clone(),
        _ => panic!(),
    }
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
            let (ident, variant): (CustomIdentifier, _) = match ast.variant {
                AstKind::TypeDefnHead {
                    ident,
                    kind,
                    ref generic_placeholders,
                } => todo!(),
                AstKind::MainDefn => todo!(),
                AstKind::RoutineDefnHead(_) => todo!(),
                AstKind::PatternDefnHead => todo!(),
                AstKind::FeatureDecl { ident, ty } => todo!(),
                AstKind::TypeAssociatedRoutineDefnHead(ref routine_defn_head) => (
                    routine_defn_head.ident.ident,
                    EntityDefnVariant::routine(
                        db,
                        routine_defn_head,
                        child.opt_children.unwrap(),
                        arena,
                        file,
                    )?,
                ),
                AstKind::TypeMethodDefnHead(ref head) => {
                    let method_source = match head.routine_kind {
                        RoutineKeyword::Proc => todo!(),
                        RoutineKeyword::Func => {
                            let stmts = semantics_eager::parse_func_stmts(
                                &head.input_placeholders,
                                db,
                                arena,
                                child.opt_children.unwrap(),
                                file,
                            )?;
                            MethodSource::Func { stmts }
                        }
                    };
                    let method_variant = MethodDefnVariant::TypeMethod {
                        ty: ty_route,
                        method_source,
                    };
                    (
                        head.ident.ident,
                        EntityDefnVariant::Method {
                            input_placeholders: head.input_placeholders.clone(),
                            output_ty: head.output_ty,
                            this_contract: head.this_contract,
                            method_variant,
                            output_contract: OutputLiason::Transfer,
                            generic_placeholders: head.generic_placeholders.clone(),
                        },
                    )
                }
                AstKind::Use { .. } => todo!(),
                AstKind::FieldDefnHead(ref field_defn_head) => (
                    field_defn_head.ident.ident,
                    EntityDefnVariant::type_field_from_ast(
                        db,
                        arena,
                        file,
                        field_defn_head,
                        child.opt_children.clone(),
                    )?,
                ),
                AstKind::DatasetConfigDefnHead => todo!(),
                AstKind::Stmt(_) => todo!(),
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => todo!(),
                AstKind::Submodule { ident, source_file } => todo!(),
                AstKind::Visual => continue,
            };
            members.insert_new(EntityDefn::new(
                ident.into(),
                variant,
                db.make_subroute(ty_route, ident, Vec::new()),
                file,
                ast.range,
            ))
        }
        Ok(())
    }
}
