use husky_entity_route::EntityRouteItd;
use husky_file::FileItd;
use husky_static_defn::EntityStaticDefn;
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
        file: FileItd,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file: FileItd,
    },
    TargetInput,
    Any {
        ident: CustomIdentifier,
        file: FileItd,
        range: TextRange,
        route: EntityRouteItd,
    },
    ThisType {
        file: FileItd,
        range: TextRange,
    },
}

impl EntitySource {
    pub fn from_file(file_id: FileItd, token_group_index: usize) -> EntitySource {
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
