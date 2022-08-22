use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_ty_linkages(
        &mut self,
        ty_kind: TyKind,
        opt_type_call: &Option<Arc<husky_entity_semantics::TypeCallDefn>>,
        ty: EntityRoutePtr,
        members: &Arc<Vec<Arc<EntityDefn>>>,
    ) {
        if let Some(_) = opt_type_call {
            self.gen_type_call_linkage(ty);
        }
        // currently field and index are always generated
        self.gen_member_access_linkages(members, ty);
    }

    fn gen_type_call_linkage(&mut self, entity_route: EntityRoutePtr) {
        self.write("\n    (\n");
        self.write(&format!(
            r#"        __StaticLinkageKey::TypeCall {{
            ty: "{}"
        }},
"#,
            entity_route
        ));
        let call_form_decl = self.db.entity_call_form_decl(entity_route).unwrap();
        self.gen_transfer_linkage(
            self.db.needs_eval_context(entity_route),
            None,
            |this| {
                this.gen_entity_route(entity_route, EntityRouteRole::Caller);
                this.write("::__call__")
            },
            |this| {
                this.gen_entity_route(entity_route, EntityRouteRole::StaticCallRoute);
                this.write("::__call__")
            },
            &call_form_decl,
        );
        self.write("\n    ),");
    }

    fn gen_member_access_linkages(
        &mut self,
        members: &Arc<Vec<Arc<EntityDefn>>>,
        ty: EntityRoutePtr,
    ) {
        // todo: use decl rather than defn
        for member in members.iter() {
            let is_defn_static = self.db.is_defn_static(ty);
            match member.variant {
                EntityDefnVariant::TyField {
                    field_ty,
                    ref field_variant,
                    liason,
                    opt_linkage,
                } => self.gen_struct_field_linkages(field_variant, member, liason, ty, field_ty),
                _ => {
                    let member_entity_route = match member.base_route.variant {
                        EntityRouteVariant::TypeAsTraitMember { trai, ident, .. } => {
                            if trai.variant
                                == self.db.entity_route_menu().std_ops_index_trai.variant
                            {
                                self.db
                                    .ty_as_trai_subroute(ty, trai, ident, Default::default())
                            } else {
                                todo!()
                            }
                        }
                        _ => continue,
                    };
                    self.gen_linkage_entry(member_entity_route, member);
                }
            }
        }
    }

    fn gen_struct_field_linkages(
        &mut self,
        field_variant: &FieldDefnVariant,
        member: &Arc<EntityDefn>,
        liason: MemberLiason,
        ty: EntityRoutePtr,
        field_ty: EntityRoutePtr,
    ) {
        match field_variant {
            FieldDefnVariant::StructOriginal
            | FieldDefnVariant::StructDefault { .. }
            | FieldDefnVariant::StructDerivedEager { .. } => {
                self.write("\n    (\n");
                let field_ident = member.ident.as_str();
                let canonical_field_ty = field_ty.canonicalize();
                let field_ty_canonical_kind = canonical_field_ty.kind();
                let field_ty_reg_memory_kind = self.db.reg_memory_kind(field_ty);
                self.write(&format!(
                    r#"        __StaticLinkageKey::StructEagerField {{
            this_ty: "{ty}",
            field_ident: "{field_ident}",
        }},
        eager_field_linkage!(
            {liason},
            {field_ty_canonical_kind},
            {field_ty_reg_memory_kind},
            "#
                ));
                self.gen_entity_route(ty, EntityRouteRole::Decl);
                // INTRINSIC_THIS_TY_VTABLE
                self.write(", __registration__::");
                self.write(&self.db.mangled_intrinsic_ty_vtable(ty));
                // INTRINSIC_FIELD_TY
                self.write(", ");
                self.gen_entity_route(field_ty.intrinsic(), EntityRouteRole::Decl);
                // INTRINSIC_FIELD_TY_VTABLE
                self.write(", __registration__::");
                self.write(&self.db.mangled_intrinsic_ty_vtable(field_ty));
                self.write(format!(
                    r#",
            {field_ident}
        )
    ),"#,
                ));
            }
            FieldDefnVariant::StructDerivedLazy { ref defn_repr } => match **defn_repr {
                DefinitionRepr::LazyExpr { .. } => (),
                DefinitionRepr::LazyBlock { .. } => (),
                DefinitionRepr::FuncBlock {
                    route,
                    file,
                    range,
                    ref stmts,
                    return_ty,
                } => {
                    let field_ident = member.ident.as_str();
                    self.write(&format!(
                        r#"
    (
        __StaticLinkageKey::FeatureEagerBlock {{
            route: "{route}",
        }},
        lazy_field_linkage!("#,
                    ));
                    self.gen_entity_route(ty, EntityRouteRole::Decl);
                    // INTRINSIC_THIS_TY_VTABLE
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_intrinsic_ty_vtable(ty));
                    // INTRINSIC_FIELD_TY
                    self.write(", ");
                    self.gen_entity_route(return_ty.route.intrinsic(), EntityRouteRole::Decl);
                    // INTRINSIC_FIELD_TY_VTABLE
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_intrinsic_ty_vtable(return_ty.route));
                    self.write(", ");
                    self.write(field_ident);
                    self.write(
                        r#")
    ),"#,
                    );
                }
                DefinitionRepr::ProcBlock {
                    route,
                    file,
                    range,
                    ref stmts,
                    return_ty,
                } => {
                    let field_ident = member.ident.as_str();
                    self.write(&format!(
                        r#"
    (
        __StaticLinkageKey::FeatureEagerBlock {{
            route: "{route}",
        }},
        lazy_field_linkage!("#,
                    ));
                    self.gen_entity_route(ty, EntityRouteRole::Decl);
                    // INTRINSIC_THIS_TY_VTABLE
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_intrinsic_ty_vtable(ty));
                    // INTRINSIC_FIELD_TY
                    self.write(", ");
                    self.gen_entity_route(return_ty.route.intrinsic(), EntityRouteRole::Decl);
                    // INTRINSIC_FIELD_TY_VTABLE
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_intrinsic_ty_vtable(return_ty.route));
                    self.write(", ");
                    self.write(field_ident);
                    self.write(
                        r#")
    ),"#,
                    );
                }
            },
            FieldDefnVariant::RecordOriginal | FieldDefnVariant::RecordDerived { .. } => (),
        }
    }
}
