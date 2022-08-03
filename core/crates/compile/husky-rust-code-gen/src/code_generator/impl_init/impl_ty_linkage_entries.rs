use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_ty_linkages(
        &mut self,
        ty_kind: TyKind,
        opt_type_call: &Option<Arc<husky_entity_semantics::TypeCallDefn>>,
        entity_route: EntityRoutePtr,
        members: &Arc<Vec<Arc<EntityDefn>>>,
    ) {
        if let Some(_) = opt_type_call {
            self.gen_type_call_linkage(entity_route);
        }
        match ty_kind {
            TyKind::Enum => todo!(),
            TyKind::Record => todo!(),
            TyKind::Struct => todo!(),
            TyKind::Primitive => todo!(),
            TyKind::Vec => todo!(),
            TyKind::Array => todo!(),
            TyKind::Slice => todo!(),
            TyKind::CyclicSlice => todo!(),
            TyKind::Tuple => todo!(),
            TyKind::Mor => todo!(),
            TyKind::Fp => todo!(),
            TyKind::AssociatedAny => todo!(),
            TyKind::ThisAny => todo!(),
            TyKind::SpatialPlaceholderAny => todo!(),
            TyKind::BoxAny => todo!(),
            TyKind::HigherKind => todo!(),
            TyKind::Ref => todo!(),
            TyKind::Option => todo!(),
        }
        for member in members.iter() {
            let is_defn_static = self.db.is_defn_static(entity_route);
            match member.variant {
                EntityDefnVariant::TyField {
                    ty,
                    ref field_variant,
                    liason,
                    opt_linkage,
                } => self.gen_ty_field_linkages(field_variant, member, liason, entity_route, ty),
                _ => {
                    break;
                    // let member_entity_route = match member.base_route.kind {
                    //     EntityRouteKind::Root { ident } => {
                    //         p!(member.base_route, member.ident);
                    //         todo!()
                    //     }
                    //     EntityRouteKind::Package { main, ident } => todo!(),
                    //     EntityRouteKind::Child { parent, ident } => {
                    //         make_subroute(entity_route, member.ident.custom(), Default::default())
                    //     }
                    //     EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
                    //         msg_once!("todo: ignore trait that is generic over a specific type");
                    //         make_type_as_trait_member_route(
                    //             entity_route,
                    //             trai,
                    //             ident,
                    //             Default::default(),
                    //         )
                    //     }
                    //     EntityRouteKind::Input { main } => todo!(),
                    //     EntityRouteKind::Generic { ident, entity_kind } => todo!(),
                    //     EntityRouteKind::ThisType => todo!(),
                    // };
                    // todo!();
                    // self.gen_linkage_entry(member_entity_route, member);
                    // if is_defn_static {
                    //     todo!();
                    // } else {
                    //     break;
                    // }
                }
            }
        }
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

    fn gen_ty_field_linkages(
        &mut self,
        field_variant: &FieldDefnVariant,
        member: &Arc<EntityDefn>,
        liason: MemberLiason,
        entity_route: EntityRoutePtr,
        ty: EntityRoutePtr,
    ) {
        match field_variant {
            FieldDefnVariant::StructOriginal
            | FieldDefnVariant::StructDefault { .. }
            | FieldDefnVariant::StructDerivedEager { .. } => {
                self.write("\n    (\n");
                let field_ident = member.ident.as_str();
                self.write(&format!(
                    r#"        __StaticLinkageKey::StructEagerField {{
            this_ty: "{entity_route}",
            field_ident: "{field_ident}",
        }},
        {}!("#,
                    match liason {
                        MemberLiason::Immutable => "eager_field_linkage",
                        MemberLiason::Mutable => {
                            if ty.is_ref() {
                                todo!()
                            } else {
                                "eager_mut_field_linkage"
                            }
                        }
                        MemberLiason::Derived => todo!(),
                    }
                ));
                self.gen_entity_route(entity_route, EntityRouteRole::Decl);
                self.write(", __registration__::");
                self.write(&self.db.mangled_ty_vtable(ty));
                self.write(", ");
                self.write(field_ident);
                let copy_kind = if self.db.is_copyable(ty).unwrap() {
                    match ty {
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
                    ty,
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
                    self.gen_entity_route(entity_route, EntityRouteRole::Decl);
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_ty_vtable(ty.route));
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
