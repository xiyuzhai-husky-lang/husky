//! `NameDefinition` keeps information about the element we want to search references for.
//! The element is represented by `NameKind`. It's located inside some `container` and
//! has a `visibility`, which defines a search scope.
//! Note that the reference search is possible for not all of the classified items.

// FIXME: this badly needs rename/rewrite (matklad, 2020-02-06).

use arrayvec::ArrayVec;
use hir::{
    Adt, AsAssocItem, AssocItem, BuiltinType, Const, Field, Function, GenericParam, HasVisibility,
    Impl, ItemInNs, Label, Local, MacroDef, Module, ModuleDef, Name, PathResolution, Semantics,
    Static, Trait, TypeAlias, Variant, Visibility,
};
use stdx::impl_from;
use syntax::{ast, SyntaxKind, SyntaxNode, SyntaxToken};

use crate::{helpers::try_resolve_derive_input, IdeDatabase};

// FIXME: a more precise name would probably be `Symbol`?
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Definition {
    Macro(MacroDef),
    Field(Field),
    Module(Module),
    Function(Function),
    Adt(Adt),
    Variant(Variant),
    Const(Const),
    Static(Static),
    Trait(Trait),
    TypeAlias(TypeAlias),
    BuiltinType(BuiltinType),
    SelfType(Impl),
    Local(Local),
    GenericParam(GenericParam),
    Label(Label),
}

impl Definition {
    pub fn from_token(
        sema: &Semantics<IdeDatabase>,
        token: &SyntaxToken,
    ) -> ArrayVec<Definition, 2> {
        todo!()
    }

    pub fn from_node(sema: &Semantics<IdeDatabase>, node: &SyntaxNode) -> ArrayVec<Definition, 2> {
        todo!()
    }

    pub fn module(&self, db: &IdeDatabase) -> Option<Module> {
        todo!()
    }

    pub fn visibility(&self, db: &IdeDatabase) -> Option<Visibility> {
        todo!()
    }

    pub fn name(&self, db: &IdeDatabase) -> Option<Name> {
        todo!()
    }
}

/// On a first blush, a single `ast::Name` defines a single definition at some
/// scope. That is, that, by just looking at the syntactical category, we can
/// unambiguously define the semantic category.
///
/// Sadly, that's not 100% true, there are special cases. To make sure that
/// callers handle all the special cases correctly via exhaustive matching, we
/// add a [`NameClass`] enum which lists all of them!
///
/// A model special case is `None` constant in pattern.
#[derive(Debug)]
pub enum NameClass {
    Definition(Definition),
    /// `None` in `if let None = Some(82) {}`.
    /// Syntactically, it is a name, but semantically it is a reference.
    ConstReference(Definition),
    /// `field` in `if let Foo { field } = foo`. Here, `ast::Name` both introduces
    /// a definition into a local scope, and refers to an existing definition.
    PatFieldShorthand {
        local_def: Local,
        field_ref: Field,
    },
}

impl NameClass {
    /// `Definition` defined by this name.
    pub fn defined(self) -> Option<Definition> {
        let res = match self {
            NameClass::Definition(it) => it,
            NameClass::ConstReference(_) => return None,
            NameClass::PatFieldShorthand {
                local_def,
                field_ref: _,
            } => Definition::Local(local_def),
        };
        Some(res)
    }

    pub fn classify(sema: &Semantics<IdeDatabase>, name: &ast::Name) -> Option<NameClass> {
        todo!()
    }
}

/// This is similar to [`NameClass`], but works for [`ast::NameRef`] rather than
/// for [`ast::Name`]. Similarly, what looks like a reference in syntax is a
/// reference most of the time, but there are a couple of annoying exceptions.
///
/// A model special case is field shorthand syntax, which uses a single
/// reference to point to two different defs.
#[derive(Debug)]
pub enum NameRefClass {
    Definition(Definition),
    FieldShorthand { local_ref: Local, field_ref: Field },
}

impl NameRefClass {
    // Note: we don't have unit-tests for this rather important function.
    // It is primarily exercised via goto definition tests in `ide`.
    pub fn classify(
        sema: &Semantics<IdeDatabase>,
        name_ref: &ast::NameRef,
    ) -> Option<NameRefClass> {
        todo!()
    }
}

impl_from!(
    Field, Module, Function, Adt, Variant, Const, Static, Trait, TypeAlias, BuiltinType, Local,
    GenericParam, Label
    for Definition
);

impl From<Impl> for Definition {
    fn from(impl_: Impl) -> Self {
        Definition::SelfType(impl_)
    }
}

impl AsAssocItem for Definition {
    fn as_assoc_item(self, db: &dyn hir::db::HirDatabase) -> Option<AssocItem> {
        todo!()
    }
}

impl From<AssocItem> for Definition {
    fn from(assoc_item: AssocItem) -> Self {
        todo!()
    }
}

impl From<PathResolution> for Definition {
    fn from(path_resolution: PathResolution) -> Self {
        todo!()
    }
}

impl From<ModuleDef> for Definition {
    fn from(def: ModuleDef) -> Self {
        todo!()
    }
}

impl From<Definition> for Option<ItemInNs> {
    fn from(def: Definition) -> Self {
        todo!()
    }
}
