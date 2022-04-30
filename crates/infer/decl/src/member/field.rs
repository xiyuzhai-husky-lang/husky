use crate::*;
use entity_route::EntityRoutePtr;
use instantiate::Instantiator;
use vec_dict::HasKey;
use vm::FieldContract;
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FieldDecl {
    pub ident: CustomIdentifier,
    pub contract: FieldContract,
    pub ty: EntityRoutePtr,
    pub kind: FieldKind,
}

impl FieldDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        todo!()
    }

    pub fn from_static(db: &dyn DeclQueryGroup, static_decl: &EntityStaticDefn) -> Self {
        match static_decl.variant {
            EntityStaticDefnVariant::TypeField { ref field_variant } => Self {
                ident: db.intern_word(static_decl.name).custom(),
                contract: todo!(),
                ty: todo!(),
                kind: match *field_variant {},
            },
            _ => panic!(""),
        }
    }

    pub fn from_ast(field_defn_head: &FieldDefnHead) -> Arc<Self> {
        Arc::new(Self {
            ident: field_defn_head.ident.ident,
            contract: field_defn_head.contract,
            ty: field_defn_head.ty,
            kind: field_defn_head.kind,
        })
    }
}

impl HasKey<CustomIdentifier> for FieldDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}
