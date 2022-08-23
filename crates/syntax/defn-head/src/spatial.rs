use husky_entity_kind::TyKind;
use husky_entity_route::{EntityKind, RangedEntityRoute};
use husky_entity_syntax::EntitySyntaxQueryGroup;
use husky_file::FilePtr;
use husky_text::{RangedCustomIdentifier, TextRange};
use husky_word::CustomIdentifier;
use static_defn::StaticSpatialParameter;
use vec_like::VecMapEntry;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpatialParameter {
    pub ident: RangedCustomIdentifier,
    pub variant: SpatialParameterVariant,
    pub file: FilePtr,
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
    Type { traits: Vec<RangedEntityRoute> },
}

impl SpatialParameter {
    pub fn from_static(db: &dyn EntitySyntaxQueryGroup, _: &StaticSpatialParameter) -> Self {
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
