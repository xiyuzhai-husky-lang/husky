use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FieldDefn {
    pub ident: CustomIdentifier,
    pub ty: EntityRoutePtr,
    pub variant: FieldDefnVariant,
    pub contract: FieldContract,
}

impl FieldDefn {
    pub(crate) fn from_ast(field_defn_head: &FieldDefnHead) -> SemanticResultArc<Self> {
        let variant = match field_defn_head.kind {
            FieldKind::StructOriginal => FieldDefnVariant::Original,
            FieldKind::StructDerived => FieldDefnVariant::Derived { stmts: todo!() },
            FieldKind::RecordOriginal => FieldDefnVariant::Original,
            FieldKind::RecordDerived => FieldDefnVariant::Derived { stmts: todo!() },
        };
        Ok(Arc::new(Self {
            ident: field_defn_head.ident,
            ty: field_defn_head.ty,
            contract: field_defn_head.contract,
            variant,
        }))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldDefnVariant {
    Original,
    Derived { stmts: Arc<Vec<Arc<LazyStmt>>> },
}

impl HasKey<CustomIdentifier> for FieldDefn {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}
