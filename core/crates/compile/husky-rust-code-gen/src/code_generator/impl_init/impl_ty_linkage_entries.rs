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
        self.gen_specific_routine_linkage(
            None,
            |this| {
                this.gen_entity_route(entity_route, EntityRouteRole::Caller);
                this.write("::__call__")
            },
            |this| {
                this.gen_entity_route(entity_route, EntityRouteRole::Caller);
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
                    let member_entity_route = match member.base_route.kind {
                        EntityRouteKind::TypeAsTraitMember { trai, ident, .. } => {
                            if trai.kind == entity_route_menu().std_ops_index_trai.kind {
                                make_type_as_trait_member_route(ty, trai, ident, Default::default())
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
                self.write(&format!(
                    r#"        __StaticLinkageKey::StructEagerField {{
            this_ty: "{ty}",
            field_ident: "{field_ident}",
        }},
        {}!("#,
                    match liason {
                        MemberLiason::Immutable => "eager_field_linkage",
                        MemberLiason::Mutable => {
                            if field_ty.is_ref() {
                                todo!()
                            } else {
                                "eager_mut_field_linkage"
                            }
                        }
                        MemberLiason::Derived => todo!(),
                    }
                ));
                self.gen_entity_route(ty, EntityRouteRole::Decl);
                self.write(", __registration__::");
                self.write(&self.db.mangled_ty_vtable(ty));
                self.write(", __registration__::");
                self.write(&self.db.mangled_ty_vtable(field_ty));
                self.write(", ");
                self.write(field_ident);
                let copy_kind = if self.db.is_copyable(field_ty).unwrap() {
                    match field_ty {
                        EntityRoutePtr::Root(root_identifer) => match root_identifer {
                            RootIdentifier::Void
                            | RootIdentifier::I32
                            | RootIdentifier::I64
                            | RootIdentifier::F32
                            | RootIdentifier::F64
                            | RootIdentifier::B32
                            | RootIdentifier::B64
                            | RootIdentifier::Bool => "direct",
                            _ => panic!(),
                        },
                        EntityRoutePtr::Custom(_) => "box",
                        EntityRoutePtr::ThisType => todo!(),
                    }
                } else {
                    "invalid"
                };
                self.write(", ");
                self.write(copy_kind);
                self.write(")\n    ),");
            }
            FieldDefnVariant::StructDerivedLazy { ref defn_repr } => match **defn_repr {
                DefinitionRepr::LazyExpr { .. } => (),
                DefinitionRepr::LazyBlock { .. } => (),
                DefinitionRepr::FuncBlock {
                    route,
                    file,
                    range,
                    ref stmts,
                    output_ty: field_ty,
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
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_ty_vtable(ty));
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_ty_vtable(field_ty.route));
                    self.write(", ");
                    self.write(field_ident);
                    self.write(
                        r#")
    ),"#,
                    );
                }
                DefinitionRepr::ProcBlock {
                    file,
                    range,
                    ref stmts,
                    ty,
                } => todo!(),
            },
            FieldDefnVariant::RecordOriginal | FieldDefnVariant::RecordDerived { .. } => (),
        }
    }
}
