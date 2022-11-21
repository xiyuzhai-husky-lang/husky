use husky_entity_path::EntityPath;
use husky_identifier::Identifier;
use husky_source_path::SourcePath;
use husky_static_defn::EntityStaticDefn;
use husky_term::Ty;
use husky_text::TextRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitySource {
    StaticModuleItem(&'static EntityStaticDefn),
    StaticTypeMember(&'static EntityStaticDefn),
    StaticTraitMember(&'static EntityStaticDefn),
    StaticEnumVariant(&'static EntityStaticDefn),
    StaticTypeAsTraitMember,
    WithinBuiltinModule,
    WithinModule {
        file: SourcePath,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file: SourcePath,
    },
    TargetInput,
    Any {
        ident: Identifier,
        file: SourcePath,
        range: TextRange,
        entity_path: EntityPath,
    },
    ThisType {
        file: SourcePath,
        range: TextRange,
    },
}

impl EntitySource {
    pub fn from_file(file_id: SourcePath, token_group_index: usize) -> EntitySource {
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
