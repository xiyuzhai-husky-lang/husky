use crate::*;
use entity_route::EntityRoutePtr;
use instantiate::Instantiator;
use vec_dict::HasKey;
use vm::MembAccessContract;
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FieldDecl {
    pub ident: CustomIdentifier,
    pub contract: MembAccessContract,
    pub ty: EntityRoutePtr,
}

impl FieldDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        todo!()
    }

    pub fn from_static(db: &dyn DeclQueryGroup, static_decl: &StaticFieldDecl) -> Self {
        Self {
            ident: db.intern_word(static_decl.name).custom(),
            contract: todo!(),
            ty: todo!(),
        }
    }
}

impl HasKey<CustomIdentifier> for FieldDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}
