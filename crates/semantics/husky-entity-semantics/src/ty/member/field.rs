use super::*;

impl EntityDefnVariant {
    pub(crate) fn ty_field_from_static(
        symbol_context: &mut dyn AtomContext,
        static_defn: &EntityStaticDefn,
    ) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::TyField {
                field_kind,
                liason,
                field_ty,
                linkage,
            } => Self::TyField {
                field_ty: symbol_context.parse_entity_route(field_ty).unwrap(),
                liason,
                field_variant: match field_kind {
                    FieldKind::StructRegular => FieldDefnVariant::StructOriginal,
                    FieldKind::StructDefault => todo!(),
                    FieldKind::StructDerived => todo!(),
                    FieldKind::StructMemo => todo!(),
                    FieldKind::RecordRegular => todo!(),
                    FieldKind::RecordProperty => todo!(),
                },
                opt_linkage: Some(linkage),
            },
            _ => todo!(),
        }
    }

    pub(crate) fn ty_field_from_ast(
        _db: &dyn EntityDefnQueryGroup,
        _arena: &RawExprArena,
        _file: FileItd,
        _ty_route: EntityRouteItd,
        _ast: &Ast,
        _children: Option<AstIter>,
    ) -> SemanticResult<Self> {
        todo!()
        // match ast.variant {
        //     AstVariant::FieldDefnHead {
        //         liason,
        //         ranged_ident,
        //         field_ty,
        //         ast_field_kind: field_kind,
        //     } => {
        //         let field_variant = match field_kind {
        //             AstFieldKind::StructOriginal => FieldDefnVariant::StructOriginal,
        //             AstFieldKind::StructProperty { paradigm } => {
        //                 FieldDefnVariant::StructDerivedLazy {
        //                     defn_repr: parse_definition_repr(
        //                         db,
        //                         paradigm,
        //                         db.subroute(ty_route, ranged_ident.ident, thin_vec![]),
        //                         field_ty,
        //                         arena,
        //                         children,
        //                         file,
        //                     )?,
        //                 }
        //             }
        //             AstFieldKind::RecordOriginal => FieldDefnVariant::RecordOriginal,
        //             AstFieldKind::RecordDerived => FieldDefnVariant::RecordDerived {
        //                 defn_repr: Arc::new(DefinitionRepr::LazyBlock {
        //                     stmts: husky_lazy_semantics::parse_lazy_stmts(
        //                         db.upcast(),
        //                         arena,
        //                         children.unwrap(),
        //                         file,
        //                         field_ty,
        //                     )?,
        //                     ty: field_ty,
        //                 }),
        //             },
        //             AstFieldKind::StructDefault { default } => FieldDefnVariant::StructDefault {
        //                 default: parse_eager_expr(db.upcast(), arena, file, default)?,
        //             },
        //             AstFieldKind::StructDerivedEager { derivation } => {
        //                 FieldDefnVariant::StructDerivedEager {
        //                     derivation: parse_eager_expr(db.upcast(), arena, file, derivation)?,
        //                 }
        //             }
        //         };
        //         Ok(Self::TyField {
        //             field_ty: field_ty.route,
        //             liason,
        //             field_variant,
        //             opt_linkage: None,
        //         })
        //     }
        //     _ => panic!(),
        // }
    }

    pub(crate) fn collect_original_fields(
        db: &dyn EntityDefnQueryGroup,
        arena: &RawExprArena,
        file: FileItd,
        ty_route: EntityRouteItd,
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<Arc<EntityDefn>>,
    ) -> SemanticResult<()> {
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref().unwrap();
            match ast.variant {
                AstVariant::FieldDefnHead {
                    ast_field_kind: field_kind,
                    ranged_ident,
                    ..
                } => {
                    match field_kind {
                        AstFieldKind::StructOriginal => (),
                        AstFieldKind::RecordOriginal => (),
                        _ => break,
                    }
                    members
                        .insert_new(EntityDefn::new(
                            db,
                            ranged_ident.ident.into(),
                            EntityDefnVariant::ty_field_from_ast(
                                db,
                                arena,
                                file,
                                ty_route,
                                ast,
                                child.opt_children.clone(),
                            )?,
                            db.intern_entity_route(EntityRoute {
                                variant: EntityRouteVariant::Child {
                                    parent: ty_route,
                                    ident: ranged_ident.ident,
                                },
                                temporal_arguments: thin_vec![],
                                spatial_arguments: thin_vec![],
                            }),
                            file,
                            ast.range,
                        ))
                        .unwrap();
                    children.next();
                }
                _ => break,
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldDefnVariant {
    StructOriginal,
    StructDefault { default: Arc<EagerExpr> },
    StructDerivedEager { derivation: Arc<EagerExpr> },
    StructDerivedLazy { defn_repr: Arc<DefinitionRepr> },
    RecordOriginal,
    RecordDerived { defn_repr: Arc<DefinitionRepr> },
}
