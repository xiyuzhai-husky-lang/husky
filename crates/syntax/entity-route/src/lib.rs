mod alias;
mod allocate_unique;
mod generic;
mod kind;

pub use alias::ScopeAliasTable;
pub use allocate_unique::{
    new_scope_unique_allocator, AllocateUniqueScope, EntityRouteInterner, EntityRoutePtr,
};
pub use entity_syntax::EntityKind;
use entity_syntax::TyKind;
use file::FilePtr;
pub use generic::*;
use static_decl::StaticEntityDecl;
use text::{TextRange, TextRanged};
use word::{ContextualIdentifier, CustomIdentifier, Identifier, RootIdentifier};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EntityRoute {
    pub kind: EntityRouteKind,
    pub generics: Vec<GenericArgument>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RangedEntityRoute {
    pub route: EntityRoutePtr,
    pub range: TextRange,
}

impl TextRanged for RangedEntityRoute {
    fn text_range_ref(&self) -> &TextRange {
        &self.range
    }
}

impl std::fmt::Debug for EntityRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.kind {
            EntityRouteKind::Root { ident } => ident.fmt(f)?,
            EntityRouteKind::Package { main, ident } => {
                // f.write_str("[pack=")?;
                // main.fmt(f)?;
                // f.write_str("]")?;
                // ident.fmt(f)?
                f.write_str("pack")?
            }
            EntityRouteKind::ChildScope { parent, ident } => {
                parent.fmt(f)?;
                f.write_str("::")?;
                ident.fmt(f)?
            }
            EntityRouteKind::Input { main } => todo!(),
            EntityRouteKind::Generic { ident, .. } => todo!(),
            EntityRouteKind::ThisType => todo!(),
        };
        if self.generics.len() > 0 {
            f.write_str("<")?;
            for (i, generic) in self.generics.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                match generic {
                    GenericArgument::Const(_) => todo!(),
                    GenericArgument::Scope(scope) => scope.fmt(f)?,
                }
            }
            f.write_str(">")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericArgument {
    Const(usize),
    Scope(EntityRoutePtr),
}

impl GenericArgument {
    pub fn as_scope(&self) -> EntityRoutePtr {
        match self {
            GenericArgument::Const(_) => panic!(),
            GenericArgument::Scope(scope) => *scope,
        }
    }
}

impl From<usize> for GenericArgument {
    fn from(size: usize) -> Self {
        Self::Const(size)
    }
}

impl From<EntityRoutePtr> for GenericArgument {
    fn from(scope: EntityRoutePtr) -> Self {
        GenericArgument::Scope(scope)
    }
}

impl From<RootIdentifier> for EntityRouteKind {
    fn from(ident: RootIdentifier) -> Self {
        Self::Root { ident }
    }
}

impl From<&RootIdentifier> for EntityRouteKind {
    fn from(ident: &RootIdentifier) -> Self {
        Self::Root { ident: *ident }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityRouteKind {
    Root {
        ident: RootIdentifier,
    },
    Package {
        main: FilePtr,
        ident: CustomIdentifier,
    },
    ChildScope {
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
    },
    Input {
        main: FilePtr,
    },
    Generic {
        ident: CustomIdentifier,
        entity_kind: EntityKind,
    },
    ThisType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticEntityData {
    pub subscopes: &'static [(&'static str, &'static StaticEntityData)],
    pub decl: StaticEntityDecl,
}

impl EntityRoute {
    pub fn pack(main: FilePtr, ident: CustomIdentifier) -> Self {
        EntityRoute {
            kind: EntityRouteKind::Package { main, ident },
            generics: Vec::new(),
        }
    }

    pub fn ident(&self) -> Identifier {
        match self.kind {
            EntityRouteKind::Root { ident } => ident.into(),
            EntityRouteKind::Package { main, ident } => ident.into(),
            EntityRouteKind::ChildScope { parent, ident } => ident.into(),
            EntityRouteKind::Input { main } => todo!(),
            EntityRouteKind::Generic {
                ident,
                entity_kind: raw_entity_kind,
            } => todo!(),
            EntityRouteKind::ThisType => todo!(),
        }
    }

    pub fn child_scope(
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> EntityRoute {
        EntityRoute {
            kind: EntityRouteKind::ChildScope { parent, ident },
            generics,
        }
    }

    pub fn new_builtin(
        ident: RootIdentifier,
        generic_arguments: Vec<GenericArgument>,
    ) -> EntityRoute {
        EntityRoute {
            kind: EntityRouteKind::Root { ident },
            generics: generic_arguments,
        }
    }

    pub fn vec(element: GenericArgument) -> Self {
        Self::new_builtin(RootIdentifier::Vec, vec![element])
    }

    pub fn array(element: GenericArgument, size: usize) -> Self {
        Self::new_builtin(RootIdentifier::Array, vec![element, size.into()])
    }

    pub fn tuple_or_void(args: Vec<GenericArgument>) -> Self {
        EntityRoute::new_builtin(
            if args.len() > 0 {
                RootIdentifier::Tuple
            } else {
                RootIdentifier::Void
            },
            args,
        )
    }

    pub fn default_func_type(args: Vec<GenericArgument>) -> Self {
        EntityRoute::new_builtin(word::default_func_type(), args)
    }

    pub fn is_builtin(&self) -> bool {
        match self.kind {
            EntityRouteKind::Root { .. } => true,
            EntityRouteKind::Package { .. } => false,
            EntityRouteKind::ChildScope { parent, .. } => parent.is_builtin(),
            EntityRouteKind::Input { .. } => false,
            EntityRouteKind::Generic { ident, .. } => todo!(),
            EntityRouteKind::ThisType => todo!(),
        }
    }
}

impl From<RootIdentifier> for EntityRoute {
    fn from(ident: RootIdentifier) -> Self {
        Self::new_builtin(ident, Vec::new())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitySource {
    Builtin(&'static StaticEntityData),
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

impl EntitySource {
    pub fn from_file(file_id: FilePtr, token_group_index: usize) -> EntitySource {
        EntitySource::WithinModule {
            file: file_id,
            token_group_index: token_group_index,
        }
    }
}

impl From<&'static StaticEntityData> for EntitySource {
    fn from(data: &'static StaticEntityData) -> Self {
        Self::Builtin(data)
    }
}
