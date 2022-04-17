use super::*;

impl EntityDefnVariant {
    pub(crate) fn type_field_from_ast(field_defn_head: &FieldDefnHead) -> SemanticResult<Self> {
        let variant = match field_defn_head.kind {
            FieldKind::StructOriginal => FieldDefnVariant::StructOriginal,
            FieldKind::StructDerived => FieldDefnVariant::StructDerived { stmts: todo!() },
            FieldKind::RecordOriginal => FieldDefnVariant::RecordOriginal,
            FieldKind::RecordDerived => FieldDefnVariant::RecordDerived { stmts: todo!() },
        };
        Ok(Self::TypeField {
            ident: field_defn_head.ident,
            ty: field_defn_head.ty,
            contract: field_defn_head.contract,
            field_variant: variant,
        })
    }

    pub(crate) fn collect_fields(
        db: &dyn InferQueryGroup,
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<Arc<EntityDefn>>,
        ty_route: EntityRoutePtr,
        file: FilePtr,
    ) -> SemanticResult<()> {
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref()?;
            match ast.kind {
                AstKind::FieldDefn(ref field_defn_head) => {
                    children.next();
                    members.insert_new(EntityDefn::new(
                        field_defn_head.ident.into(),
                        EntityDefnVariant::type_field_from_ast(field_defn_head)?,
                        db.intern_entity_route(EntityRoute {
                            kind: EntityRouteKind::Child {
                                parent: ty_route,
                                ident: field_defn_head.ident,
                            },
                            generic_arguments: vec![],
                        }),
                        file,
                        ast.range,
                    ))
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
