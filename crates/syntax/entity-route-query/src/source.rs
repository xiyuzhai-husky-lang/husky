use file::FilePtr;
use static_defn::EntityStaticDefn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityLocus {
    StaticModuleItem(&'static EntityStaticDefn),
    StaticTypeMember,
    StaticTypeAsTraitMember,
    WithinBuiltinModule,
    WithinModule {
        file: FilePtr,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file: FilePtr,
    },
    Input {
        main: FilePtr,
    },
}

impl EntityLocus {
    pub fn from_file(file_id: FilePtr, token_group_index: usize) -> EntityLocus {
        EntityLocus::WithinModule {
            file: file_id,
            token_group_index: token_group_index,
        }
    }
}

impl From<&'static EntityStaticDefn> for EntityLocus {
    fn from(data: &'static EntityStaticDefn) -> Self {
        Self::StaticModuleItem(data)
    }
}
