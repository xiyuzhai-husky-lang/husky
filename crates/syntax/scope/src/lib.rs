mod alias;
mod allocate_unique;
mod kind;

pub use alias::ScopeAliasTable;
pub use allocate_unique::{
    new_scope_unique_allocator, AllocateUniqueScope, ScopePtr, UniqueScopeAllocator,
};
use file::FilePtr;
pub use kind::{ScopeKind, TyKind};
use text::{TextRange, TextRanged};
use vm::{Compiled, EagerContract, InputContract};
use word::{BuiltinIdentifier, CustomIdentifier, Identifier, ImplicitIdentifier};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    pub route: ScopeRoute,
    pub generics: Vec<GenericArgument>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RangedScope {
    pub scope: ScopePtr,
    pub range: TextRange,
}

impl TextRanged for RangedScope {
    fn text_range_ref(&self) -> &TextRange {
        &self.range
    }
}

impl std::fmt::Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.route {
            ScopeRoute::Builtin { ident } => ident.fmt(f)?,
            ScopeRoute::Package { main, ident } => {
                // f.write_str("[package=")?;
                // main.fmt(f)?;
                // f.write_str("]")?;
                // ident.fmt(f)?
                f.write_str("package")?
            }
            ScopeRoute::ChildScope { parent, ident } => {
                parent.fmt(f)?;
                f.write_str("::")?;
                ident.fmt(f)?
            }
            ScopeRoute::Implicit { main, ident } => todo!(),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericArgument {
    Const(usize),
    Scope(ScopePtr),
}

impl From<usize> for GenericArgument {
    fn from(size: usize) -> Self {
        Self::Const(size)
    }
}

impl From<ScopePtr> for GenericArgument {
    fn from(scope: ScopePtr) -> Self {
        GenericArgument::Scope(scope)
    }
}

impl From<BuiltinIdentifier> for ScopeRoute {
    fn from(ident: BuiltinIdentifier) -> Self {
        Self::Builtin { ident }
    }
}

impl From<&BuiltinIdentifier> for ScopeRoute {
    fn from(ident: &BuiltinIdentifier) -> Self {
        Self::Builtin { ident: *ident }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeRoute {
    Builtin {
        ident: BuiltinIdentifier,
    },
    Package {
        main: FilePtr,
        ident: CustomIdentifier,
    },
    ChildScope {
        parent: ScopePtr,
        ident: CustomIdentifier,
    },
    Implicit {
        main: FilePtr,
        ident: ImplicitIdentifier,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinScopeData {
    pub scope_kind: ScopeKind,
    pub subscopes: &'static [(&'static str, &'static BuiltinScopeData)],
    pub signature: StaticScopeSignature,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StaticScopeSignature {
    Func(StaticFuncSignature),
    Ty(StaticTySignature),
    Module,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticFuncSignature {
    pub inputs: Vec<StaticInputSignature>,
    pub output: &'static str,
    pub compiled: Option<Compiled>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticTySignature {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticInputSignature {
    pub contract: InputContract,
    pub ty: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputSignature {
    pub contract: InputContract,
    pub ty: ScopePtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputPlaceholder {
    pub ident: CustomIdentifier,
    pub contract: InputContract,
    pub ranged_ty: RangedScope,
}

impl Into<InputSignature> for &InputPlaceholder {
    fn into(self) -> InputSignature {
        InputSignature {
            contract: self.contract,
            ty: self.ranged_ty.scope,
        }
    }
}

impl Scope {
    pub fn package(main: FilePtr, ident: CustomIdentifier) -> Self {
        Scope {
            route: ScopeRoute::Package { main, ident },
            generics: Vec::new(),
        }
    }
    pub fn child_scope(
        parent: ScopePtr,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> Scope {
        Scope {
            route: ScopeRoute::ChildScope { parent, ident },
            generics,
        }
    }

    pub fn new_builtin(ident: BuiltinIdentifier, generic_arguments: Vec<GenericArgument>) -> Scope {
        Scope {
            route: ScopeRoute::Builtin { ident },
            generics: generic_arguments,
        }
    }

    pub fn vec(element: GenericArgument) -> Self {
        Self::new_builtin(BuiltinIdentifier::Vector, vec![element])
    }

    pub fn array(element: GenericArgument, size: usize) -> Self {
        Self::new_builtin(BuiltinIdentifier::Array, vec![element, size.into()])
    }

    pub fn tuple_or_void(args: Vec<GenericArgument>) -> Self {
        Scope::new_builtin(
            if args.len() > 0 {
                BuiltinIdentifier::Tuple
            } else {
                BuiltinIdentifier::Void
            },
            args,
        )
    }

    pub fn default_func_type(args: Vec<GenericArgument>) -> Self {
        Scope::new_builtin(word::default_func_type(), args)
    }

    pub fn is_builtin(&self) -> bool {
        match self.route {
            ScopeRoute::Builtin { .. } => true,
            ScopeRoute::Package { .. } => false,
            ScopeRoute::ChildScope { parent, .. } => parent.is_builtin(),
            ScopeRoute::Implicit { .. } => false,
        }
    }
}

impl From<BuiltinIdentifier> for Scope {
    fn from(ident: BuiltinIdentifier) -> Self {
        Self::new_builtin(ident, Vec::new())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeSource {
    Builtin(&'static BuiltinScopeData),
    WithinBuiltinModule,
    WithinModule {
        file: FilePtr,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file: FilePtr,
    },
}

impl ScopeSource {
    pub fn from_file(file_id: FilePtr, token_group_index: usize) -> ScopeSource {
        ScopeSource::WithinModule {
            file: file_id,
            token_group_index: token_group_index,
        }
    }
}

impl From<&'static BuiltinScopeData> for ScopeSource {
    fn from(data: &'static BuiltinScopeData) -> Self {
        Self::Builtin(data)
    }
}
