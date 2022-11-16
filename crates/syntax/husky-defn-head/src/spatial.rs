use husky_entity_kind::{EntityKind, TyKind};
use husky_entity_syntax::EntitySyntaxQueryGroup;
use husky_file::PathItd;
use husky_static_defn::StaticSpatialParameter;
use husky_text::{RangedCustomIdentifier, TextRange};
use husky_word::CustomIdentifier;
use vec_like::VecMapEntry;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpatialParameter {
    pub ident: RangedCustomIdentifier,
    pub variant: SpatialParameterVariant,
    pub file: PathItd,
    pub range: TextRange,
}

impl VecMapEntry<CustomIdentifier> for SpatialParameter {
    fn key(&self) -> CustomIdentifier {
        self.ident.ident
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SpatialParameterVariant {
    Const,
    Type {/* traits: Vec<Ty> */},
}

impl SpatialParameter {
    pub fn from_static(_db: &dyn EntitySyntaxQueryGroup, _: &StaticSpatialParameter) -> Self {
        todo!()
    }
}

impl SpatialParameter {
    pub fn husky_entity_kind(&self) -> EntityKind {
        match self.variant {
            SpatialParameterVariant::Const => todo!(),
            SpatialParameterVariant::Type { .. } => EntityKind::Type(TyKind::SpatialPlaceholderAny),
        }
    }
}
