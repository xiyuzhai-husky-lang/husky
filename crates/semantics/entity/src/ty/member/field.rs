use super::*;

impl EntityDefnVariant {
    pub(crate) fn type_field_from_ast(
        db: &dyn InferQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        field_defn_head: &FieldDefnHead,
        children: Option<AstIter>,
    ) -> SemanticResult<Self> {
        let variant = match field_defn_head.kind {
            FieldKind::StructOriginal => FieldDefnVariant::StructOriginal,
            FieldKind::StructDerived => FieldDefnVariant::StructDerived { stmts: todo!() },
            FieldKind::RecordOriginal => FieldDefnVariant::RecordOriginal,
            FieldKind::RecordDerived => FieldDefnVariant::RecordDerived {
                stmts: semantics_lazy::parse_lazy_stmts(&[], db, arena, children.unwrap(), file)?,
            },
        };
        Ok(Self::TypeField {
            ty: field_defn_head.ty,
            contract: field_defn_head.contract,
            field_variant: variant,
        })
    }

    pub(crate) fn collect_original_fields(
        db: &dyn InferQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<Arc<EntityDefn>>,
        ty_route: EntityRoutePtr,
    ) -> SemanticResult<()> {
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref()?;
            match ast.kind {
                AstKind::FieldDefnHead(ref field_defn_head) => {
                    match field_defn_head.kind {
                        FieldKind::StructOriginal => (),
                        FieldKind::StructDerived => break,
                        FieldKind::RecordOriginal => (),
                        FieldKind::RecordDerived => break,
                    }
                    members.insert_new(EntityDefn::new(
                        field_defn_head.ident.ident.into(),
                        EntityDefnVariant::type_field_from_ast(
                            db,
                            arena,
                            file,
                            field_defn_head,
                            child.children.clone(),
                        )?,
                        db.intern_entity_route(EntityRoute {
                            kind: EntityRouteKind::Child {
                                parent: ty_route,
                                ident: field_defn_head.ident.ident,
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
    RecordOriginal,
    StructDerived { stmts: Arc<Vec<Arc<LazyStmt>>> },
    RecordDerived { stmts: Arc<Vec<Arc<LazyStmt>>> },
}
