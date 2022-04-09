use crate::*;
use entity_route::{EntityKind, RangedEntityRoute};
use entity_route_query::{EntityRouteQueryGroup, EntityRouteSalsaQueryGroup};
use entity_syntax::TyKind;
use static_decl::StaticGenericPlaceholder;
use vec_dict::HasKey;
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GenericPlaceholder {
    pub ident: CustomIdentifier,
    pub variant: GenericPlaceholderVariant,
}

impl HasKey<CustomIdentifier> for GenericPlaceholder {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GenericPlaceholderVariant {
    Const,
    Type { traits: Vec<RangedEntityRoute> },
}

impl GenericPlaceholder {
    pub fn from_static(db: &dyn EntityRouteQueryGroup, _: &StaticGenericPlaceholder) -> Self {
        todo!()
    }
}

impl GenericPlaceholder {
    pub fn entity_kind(&self) -> EntityKind {
        match self.variant {
            GenericPlaceholderVariant::Const => todo!(),
            GenericPlaceholderVariant::Type { ref traits } => EntityKind::Type(TyKind::Other),
        }
    }
}
