use husky_entity_path::EntityPathItd;
use husky_file::PathItd;
use husky_static_defn::EntityStaticDefn;
use husky_term::Ty;
use husky_text::TextRange;
use husky_word::CustomIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitySource {
    StaticModuleItem(&'static EntityStaticDefn),
    StaticTypeMember(&'static EntityStaticDefn),
    StaticTraitMember(&'static EntityStaticDefn),
    StaticEnumVariant(&'static EntityStaticDefn),
    StaticTypeAsTraitMember,
    WithinBuiltinModule,
    WithinModule {
        file: PathItd,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file: PathItd,
    },
    TargetInput,
    Any {
        ident: CustomIdentifier,
        file: PathItd,
        range: TextRange,
        entity_path: EntityPathItd,
    },
    ThisType {
        file: PathItd,
        range: TextRange,
    },
}

impl EntitySource {
    pub fn from_file(file_id: PathItd, token_group_index: usize) -> EntitySource {
        EntitySource::WithinModule {
            file: file_id,
            token_group_index,
        }
    }
}

impl From<&'static EntityStaticDefn> for EntitySource {
    fn from(data: &'static EntityStaticDefn) -> Self {
        Self::StaticModuleItem(data)
    }
}
