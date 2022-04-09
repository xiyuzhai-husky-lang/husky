use entity_route::EntityRoutePtr;
use instantiate::Instantiator;
use vec_map::HasKey;
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
}

impl HasKey<CustomIdentifier> for FieldDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}
