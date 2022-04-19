mod alias;
mod alloc;
mod kind;

pub use alias::ScopeAliasTable;
pub use alloc::{
    new_entity_route_interner, AllocateUniqueScope, EntityRouteInterner, EntityRoutePtr,
};
pub use entity_kind::EntityKind;
use file::FilePtr;
use static_decl::{StaticEntityDecl, CLONE_TRAIT_DECL};
use static_defn::StaticEntityDefn;
use text::{TextRange, TextRanged};
use word::{CustomIdentifier, Identifier, RootIdentifier};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EntityRoute {
    pub kind: EntityRouteKind,
    pub generic_arguments: Vec<GenericArgument>,
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
            EntityRouteKind::Root { ident } => f.write_str(&ident)?,
            EntityRouteKind::Package { ident, .. } => f.write_str(&ident)?,
            EntityRouteKind::Child { parent, ident } => {
                parent.fmt(f)?;
                f.write_str("::")?;
                f.write_str(&ident)?
            }
            EntityRouteKind::Input { .. } => f.write_str("input")?,
            EntityRouteKind::Generic { ident, .. } => f.write_str(&ident)?,
            EntityRouteKind::ThisType => f.write_str("This")?,
            EntityRouteKind::TraitMember {
                ty: parent,
                trai,
                ident,
            } => todo!(),
        };
        if self.generic_arguments.len() > 0 {
            f.write_str("<")?;
            for (i, generic) in self.generic_arguments.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                match generic {
                    GenericArgument::Const(_) => todo!(),
                    GenericArgument::EntityRoute(scope) => scope.fmt(f)?,
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
    EntityRoute(EntityRoutePtr),
}

impl GenericArgument {
    pub fn as_scope(&self) -> EntityRoutePtr {
        match self {
            GenericArgument::Const(_) => panic!(),
            GenericArgument::EntityRoute(scope) => *scope,
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
        GenericArgument::EntityRoute(scope)
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
    Child {
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
    },
    TraitMember {
        ty: EntityRoutePtr,
        trai: EntityRoutePtr,
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

impl EntityRoute {
    pub fn pack(main: FilePtr, ident: CustomIdentifier) -> Self {
        EntityRoute {
            kind: EntityRouteKind::Package { main, ident },
            generic_arguments: Vec::new(),
        }
    }

    pub fn ident(&self) -> Identifier {
        match self.kind {
            EntityRouteKind::Root { ident } => ident.into(),
            EntityRouteKind::Package { ident, .. } => ident.into(),
            EntityRouteKind::Child { ident, .. } => ident.into(),
            EntityRouteKind::Input { .. } => todo!(),
            EntityRouteKind::Generic { ident, .. } => ident.into(),
            EntityRouteKind::ThisType => todo!(),
            EntityRouteKind::TraitMember { ident, .. } => ident.into(),
        }
    }

    pub fn child_route(
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> EntityRoute {
        EntityRoute {
            kind: EntityRouteKind::Child { parent, ident },
            generic_arguments: generics,
        }
    }

    pub fn new_builtin(
        ident: RootIdentifier,
        generic_arguments: Vec<GenericArgument>,
    ) -> EntityRoute {
        EntityRoute {
            kind: EntityRouteKind::Root { ident },
            generic_arguments,
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
            EntityRouteKind::Child { parent, .. } => parent.is_builtin(),
            EntityRouteKind::Input { .. } => false,
            EntityRouteKind::Generic { .. } => todo!(),
            EntityRouteKind::ThisType => todo!(),
            EntityRouteKind::TraitMember {
                ty: parent,
                trai,
                ident,
            } => todo!(),
        }
    }

    pub fn parent(&self) -> EntityRoutePtr {
        match self.kind {
            EntityRouteKind::Root { .. }
            | EntityRouteKind::Input { .. }
            | EntityRouteKind::Package { .. }
            | EntityRouteKind::Generic { .. }
            | EntityRouteKind::ThisType => panic!(),
            EntityRouteKind::Child { parent, .. } => parent,
            EntityRouteKind::TraitMember { ty: parent, .. } => parent,
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
    StaticModuleItem(&'static StaticEntityDefn),
    StaticTypeMember,
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

impl From<&'static StaticEntityDefn> for EntitySource {
    fn from(data: &'static StaticEntityDefn) -> Self {
        Self::StaticModuleItem(data)
    }
}
