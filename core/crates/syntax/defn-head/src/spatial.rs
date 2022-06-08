use entity_kind::TyKind;
use entity_route::{EntityKind, RangedEntityRoute};
use entity_syntax::EntitySyntaxQueryGroup;
use static_defn::StaticGenericPlaceholder;
use text::RangedCustomIdentifier;
use vec_map::VecMapEntry;
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpatialParameter {
    pub ident: RangedCustomIdentifier,
    pub variant: GenericPlaceholderVariant,
}

impl VecMapEntry<CustomIdentifier> for SpatialParameter {
    fn key(&self) -> CustomIdentifier {
        self.ident.ident
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GenericPlaceholderVariant {
    Const,
    Type { traits: Vec<RangedEntityRoute> },
}

impl SpatialParameter {
    pub fn from_static(db: &dyn EntitySyntaxQueryGroup, _: &StaticGenericPlaceholder) -> Self {
        todo!()
    }
}

impl SpatialParameter {
    pub fn entity_kind(&self) -> EntityKind {
        match self.variant {
            GenericPlaceholderVariant::Const => todo!(),
            GenericPlaceholderVariant::Type { .. } => EntityKind::Type(TyKind::Other),
        }
    }
}
