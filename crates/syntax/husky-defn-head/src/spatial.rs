use husky_entity_kind::{EntityKind, TyKind};
use husky_entity_tree::EntityTreeDb;
use husky_identifier::Identifier;
use husky_source_path::SourcePath;
use husky_text::{RangedIdentifier, TextRange};
use vec_like::VecMapEntry;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpatialParameter {
    pub ident: RangedIdentifier,
    pub variant: SpatialParameterVariant,
    pub file: SourcePath,
    pub range: TextRange,
}

impl VecMapEntry<Identifier> for SpatialParameter {
    fn key(&self) -> Identifier {
        self.ident.ident
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SpatialParameterVariant {
    Const,
    Type {/* traits: Vec<Term> */},
}

impl SpatialParameter {
    // pub fn from_static(_db: &dyn EntityTreeDb, _: &StaticSpatialParameter) -> Self {
    //     todo!()
    // }
}

impl SpatialParameter {
    pub fn husky_entity_kind(&self) -> EntityKind {
        match self.variant {
            SpatialParameterVariant::Const => todo!(),
            SpatialParameterVariant::Type { .. } => EntityKind::Type(TyKind::SpatialPlaceholderAny),
        }
    }
}
