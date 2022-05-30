use super::*;

impl EntityDefnVariant {
    pub(crate) fn type_field_from_ast(
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
                field_kind,
            } => {
                let field_variant = match field_kind {
                    FieldKind::StructOriginal => FieldDefnVariant::StructOriginal,
                    FieldKind::StructDerivedLazy { paradigm } => {
                        FieldDefnVariant::StructDerivedLazy {
                            defn_repr: Arc::new(match paradigm {
                                Paradigm::LazyFunctional => {
                                    DefinitionRepr::LazyBlock { stmts: todo!() }
                                }
                                Paradigm::EagerFunctional => {
                                    let stmts = semantics_eager::parse_func_stmts(
                                        db,
                                        arena,
                                        children.unwrap(),
                                        file,
                                    )?;
                                    DefinitionRepr::FuncBlock {
                                        route: db.make_subroute(
                                            ty_route,
                                            ranged_ident.ident,
                                            vec![],
                                        ),
                                        file,
                                        range: FuncStmt::text_range(&*stmts),
                                        stmts,
                                    }
                                }
                                Paradigm::EagerProcedural => todo!(),
                            }),
                        }
                    }
                    FieldKind::RecordOriginal => FieldDefnVariant::RecordOriginal,
                    FieldKind::RecordDerived => FieldDefnVariant::RecordDerived {
                        defn_repr: Arc::new(DefinitionRepr::LazyBlock {
                            stmts: semantics_lazy::parse_lazy_stmts(
                                &[],
                                db,
                                arena,
                                children.unwrap(),
                                file,
                            )?,
                        }),
                    },
                    FieldKind::StructDefault { default } => FieldDefnVariant::StructDefault {
                        default: parse_eager_expr(db, arena, file, default)?,
                    },
                    FieldKind::StructDerivedEager { derivation } => {
                        FieldDefnVariant::StructDerivedEager {
                            derivation: parse_eager_expr(db, arena, file, derivation)?,
                        }
                    }
                };
                Ok(Self::TypeField {
                    ty: ty.route,
                    liason,
                    field_variant,
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
            let ast = child.value.as_ref()?;
            match ast.variant {
                AstVariant::FieldDefnHead {
                    field_kind,
                    ranged_ident,
                    ..
                } => {
                    match field_kind {
                        FieldKind::StructOriginal => (),
                        FieldKind::RecordOriginal => (),
                        _ => break,
                    }
                    members.insert_new(EntityDefn::new(
                        ranged_ident.ident.into(),
                        EntityDefnVariant::type_field_from_ast(
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
                            generic_arguments: vec![],
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
    EagerExpr {},
    LazyBlock {
        stmts: Arc<Vec<Arc<LazyStmt>>>,
    },
    FuncBlock {
        route: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    ProcBlock {
        file: FilePtr,
        range: TextRange,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
}
