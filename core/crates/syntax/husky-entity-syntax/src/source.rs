use husky_entity_route::EntityRoutePtr;
use husky_file::FilePtr;
use husky_text::TextRange;
use husky_word::CustomIdentifier;
use static_defn::EntityStaticDefn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitySource {
    StaticModuleItem(&'static EntityStaticDefn),
    StaticTypeMember(&'static EntityStaticDefn),
    StaticTraitMember(&'static EntityStaticDefn),
    StaticEnumVariant(&'static EntityStaticDefn),
    StaticTypeAsTraitMember,
    WithinBuiltinModule,
    WithinModule {
        file: FilePtr,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file: FilePtr,
    },
    TargetInput,
    Any {
        ident: CustomIdentifier,
        file: FilePtr,
        range: TextRange,
        route: EntityRoutePtr,
    },
}

impl EntitySource {
    pub fn from_file(file_id: FilePtr, token_group_index: usize) -> EntitySource {
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
