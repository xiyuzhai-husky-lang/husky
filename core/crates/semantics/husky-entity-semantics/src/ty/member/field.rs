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
                ty,
                __Linkage,
            } => Self::TyField {
                ty: symbol_context.parse_entity_route(ty).unwrap(),
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
        db: &dyn InferQueryGroup,
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
                ty,
                field_ast_kind: field_kind,
            } => {
                let field_variant = match field_kind {
                    FieldAstKind::StructOriginal => FieldDefnVariant::StructOriginal,
                    FieldAstKind::StructDerivedLazy { paradigm } => {
                        FieldDefnVariant::StructDerivedLazy {
                            defn_repr: Arc::new(match paradigm {
                                Paradigm::LazyFunctional => {
                                    let stmts = husky_lazy_semantics::parse_lazy_stmts(
                                        db,
                                        arena,
                                        children.unwrap(),
                                        file,
                                        ty,
                                    )?;
                                    DefinitionRepr::LazyBlock { stmts, ty }
                                }
                                Paradigm::EagerFunctional => {
                                    let stmts = husky_eager_semantics::parse_func_stmts(
                                        db,
                                        arena,
                                        children.unwrap(),
                                        file,
                                    )?;
                                    DefinitionRepr::FuncBlock {
                                        route: db.make_subroute(
                                            ty_route,
                                            ranged_ident.ident,
                                            thin_vec![],
                                        ),
                                        file,
                                        range: FuncStmt::text_range(&*stmts),
                                        stmts,
                                        ty,
                                    }
                                }
                                Paradigm::EagerProcedural => todo!(),
                            }),
                        }
                    }
                    FieldAstKind::RecordOriginal => FieldDefnVariant::RecordOriginal,
                    FieldAstKind::RecordDerived => FieldDefnVariant::RecordDerived {
                        defn_repr: Arc::new(DefinitionRepr::LazyBlock {
                            stmts: husky_lazy_semantics::parse_lazy_stmts(
                                db,
                                arena,
                                children.unwrap(),
                                file,
                                ty,
                            )?,
                            ty,
                        }),
                    },
                    FieldAstKind::StructDefault { default } => FieldDefnVariant::StructDefault {
                        default: parse_eager_expr(db, arena, file, default)?,
                    },
                    FieldAstKind::StructDerivedEager { derivation } => {
                        FieldDefnVariant::StructDerivedEager {
                            derivation: parse_eager_expr(db, arena, file, derivation)?,
                        }
                    }
                };
                Ok(Self::TyField {
                    ty: ty.route,
                    liason,
                    field_variant,
                    opt_linkage: None,
                })
            }
            _ => panic!(),
        }
    }

    pub(crate) fn collect_original_fields(
        db: &dyn InferQueryGroup,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DefinitionRepr {
    LazyExpr {
        expr: Arc<LazyExpr>,
    },
    LazyBlock {
        stmts: Arc<Vec<Arc<LazyStmt>>>,
        ty: RangedEntityRoute,
    },
    FuncBlock {
        route: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
        ty: RangedEntityRoute,
    },
    ProcBlock {
        file: FilePtr,
        range: TextRange,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
        ty: RangedEntityRoute,
    },
}
