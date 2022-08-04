use husky_entity_route::make_subroute;

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
                linkage: __Linkage,
            } => Self::TyField {
                field_ty: symbol_context.parse_entity_route(field_ty).unwrap(),
                liason,
                field_variant: match field_kind {
                    FieldKind::StructOriginal => FieldDefnVariant::StructOriginal,
                    FieldKind::StructDefault => todo!(),
                    FieldKind::StructDerivedEager => todo!(),
                    FieldKind::StructDerivedLazy => todo!(),
                    FieldKind::RecordOriginal => todo!(),
                    FieldKind::RecordDerived => todo!(),
                },
                opt_linkage: Some(__Linkage),
            },
            _ => todo!(),
        }
    }

    pub(crate) fn ty_field_from_ast(
        db: &dyn EntityDefnQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        ty_route: EntityRoutePtr,
        ast: &Ast,
        children: Option<AstIter>,
    ) -> SemanticResult<Self> {
        match ast.variant {
            AstVariant::FieldDefnHead {
                liason,
                ranged_ident,
                field_ty,
                field_ast_kind: field_kind,
            } => {
                let field_variant = match field_kind {
                    FieldAstKind::StructOriginal => FieldDefnVariant::StructOriginal,
                    FieldAstKind::StructDerivedLazy { paradigm } => {
                        FieldDefnVariant::StructDerivedLazy {
                            defn_repr: parse_definition_repr(
                                db,
                                paradigm,
                                make_subroute(ty_route, ranged_ident.ident, thin_vec![]),
                                field_ty,
                                arena,
                                children,
                                file,
                            )?,
                        }
                    }
                    FieldAstKind::RecordOriginal => FieldDefnVariant::RecordOriginal,
                    FieldAstKind::RecordDerived => FieldDefnVariant::RecordDerived {
                        defn_repr: Arc::new(DefinitionRepr::LazyBlock {
                            stmts: husky_lazy_semantics::parse_lazy_stmts(
                                db.upcast(),
                                arena,
                                children.unwrap(),
                                file,
                                field_ty,
                            )?,
                            ty: field_ty,
                        }),
                    },
                    FieldAstKind::StructDefault { default } => FieldDefnVariant::StructDefault {
                        default: parse_eager_expr(db.upcast(), arena, file, default)?,
                    },
                    FieldAstKind::StructDerivedEager { derivation } => {
                        FieldDefnVariant::StructDerivedEager {
                            derivation: parse_eager_expr(db.upcast(), arena, file, derivation)?,
                        }
                    }
                };
                Ok(Self::TyField {
                    field_ty: field_ty.route,
                    liason,
                    field_variant,
                    opt_linkage: None,
                })
            }
            _ => panic!(),
        }
    }

    pub(crate) fn collect_original_fields(
        db: &dyn EntityDefnQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        ty_route: EntityRoutePtr,
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<Arc<EntityDefn>>,
    ) -> SemanticResult<()> {
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref().unwrap();
            match ast.variant {
                AstVariant::FieldDefnHead {
                    field_ast_kind: field_kind,
                    ranged_ident,
                    ..
                } => {
                    match field_kind {
                        FieldAstKind::StructOriginal => (),
                        FieldAstKind::RecordOriginal => (),
                        _ => break,
                    }
                    members.insert_new(EntityDefn::new(
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
                            kind: EntityRouteKind::Child {
                                parent: ty_route,
                                ident: ranged_ident.ident,
                            },
                            temporal_arguments: thin_vec![],
                            spatial_arguments: thin_vec![],
                        }),
                        file,
                        ast.range,
                    ));
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
