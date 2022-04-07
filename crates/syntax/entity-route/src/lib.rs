mod alias;
mod allocate_unique;
mod generic;
mod kind;

pub use alias::ScopeAliasTable;
pub use allocate_unique::{
    new_scope_unique_allocator, AllocateUniqueScope, EntityRoutePtr, ScopeInterner,
};
use entity_syntax::RawTyKind;
use file::FilePtr;
pub use generic::*;
pub use kind::RawEntityKind;
use text::{TextRange, TextRanged};
use visual_syntax::BuiltinVisualizer;
use vm::{EagerContract, InputContract, RoutineFp};
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
            EntityRouteKind::Pack { main, ident } => {
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
            EntityRouteKind::Contextual { main, ident } => todo!(),
            EntityRouteKind::Generic { ident, .. } => todo!(),
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
    Pack {
        main: FilePtr,
        ident: CustomIdentifier,
    },
    ChildScope {
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
    },
    Contextual {
        main: FilePtr,
        ident: ContextualIdentifier,
    },
    Generic {
        ident: CustomIdentifier,
        raw_entity_kind: RawEntityKind,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinEntityData {
    pub subscopes: &'static [(&'static str, &'static BuiltinEntityData)],
    pub decl: BuiltinEntityDecl,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BuiltinEntityDecl {
    Func(StaticFuncDecl),
    Ty {
        raw_ty_kind: RawTyKind,
        visualizer: BuiltinVisualizer,
    },
    Vec,
    Module,
}

impl BuiltinEntityDecl {
    pub fn raw_entity_kind(&self) -> RawEntityKind {
        match self {
            BuiltinEntityDecl::Func(_) => RawEntityKind::Routine,
            BuiltinEntityDecl::Ty { raw_ty_kind, .. } => RawEntityKind::Type(*raw_ty_kind),
            BuiltinEntityDecl::Module => RawEntityKind::Module,
            BuiltinEntityDecl::Vec => RawEntityKind::Type(RawTyKind::Vec),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticFuncDecl {
    pub inputs: Vec<StaticInputSignature>,
    pub output: &'static str,
    pub compiled: RoutineFp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticInputSignature {
    pub contract: InputContract,
    pub ty: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputSignature {
    pub contract: InputContract,
    pub ty: EntityRoutePtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputPlaceholder {
    pub ident: CustomIdentifier,
    pub contract: InputContract,
    pub ranged_ty: RangedEntityRoute,
}

impl Into<InputSignature> for &InputPlaceholder {
    fn into(self) -> InputSignature {
        InputSignature {
            contract: self.contract,
            ty: self.ranged_ty.route,
        }
    }
}

impl EntityRoute {
    pub fn pack(main: FilePtr, ident: CustomIdentifier) -> Self {
        EntityRoute {
            kind: EntityRouteKind::Pack { main, ident },
            generics: Vec::new(),
        }
    }

    pub fn ident(&self) -> Identifier {
        match self.kind {
            EntityRouteKind::Root { ident } => ident.into(),
            EntityRouteKind::Pack { main, ident } => ident.into(),
            EntityRouteKind::ChildScope { parent, ident } => ident.into(),
            EntityRouteKind::Contextual { main, ident } => todo!(),
            EntityRouteKind::Generic {
                ident,
                raw_entity_kind,
            } => todo!(),
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
            EntityRouteKind::Pack { .. } => false,
            EntityRouteKind::ChildScope { parent, .. } => parent.is_builtin(),
            EntityRouteKind::Contextual { .. } => false,
            EntityRouteKind::Generic { ident, .. } => todo!(),
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
    Builtin(&'static BuiltinEntityData),
    WithinBuiltinModule,
    WithinModule {
        file: FilePtr,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file: FilePtr,
    },
    Contextual {
        main: FilePtr,
        ident: ContextualIdentifier,
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

impl From<&'static BuiltinEntityData> for EntitySource {
    fn from(data: &'static BuiltinEntityData) -> Self {
        Self::Builtin(data)
    }
}
