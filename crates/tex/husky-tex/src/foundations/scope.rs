use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};

use ecow::{eco_format, EcoString};
use indexmap::IndexMap;

use crate::diag::{bail, HintedStrResult, HintedString, StrResult};
use crate::foundations::{
    ElementSchemaRef, Func, IntoTexValue, IsTexElem, NativeFunc, NativeFuncData, NativeType,
    TexModuleEvaluation, TexValue, Type,
};
use crate::util::Static;
use crate::Library;

#[doc(inline)]
pub use husky_tex_macros::category;

/// A stack of scopes.
#[derive(Debug, Default, Clone)]
pub struct TexValueAssignmentGroups<'a> {
    /// The active scope.
    pub top: TexValueAssignmentGroup,
    /// The stack of lower scopes.
    pub scopes: Vec<TexValueAssignmentGroup>,
    /// The standard library.
    pub base: Option<&'a Library>,
}

impl<'a> TexValueAssignmentGroups<'a> {
    /// Create a new, empty hierarchy of scopes.
    pub fn new(base: Option<&'a Library>) -> Self {
        Self {
            top: TexValueAssignmentGroup::new(),
            scopes: vec![],
            base,
        }
    }

    /// Enter a new scope.
    pub fn enter(&mut self) {
        self.scopes.push(std::mem::take(&mut self.top));
    }

    /// Exit the topmost scope.
    ///
    /// This panics if no scope was entered.
    pub fn exit(&mut self) {
        self.top = self.scopes.pop().expect("no pushed scope");
    }

    /// Try to access a variable immutably.
    pub fn get(&self, var: &str) -> HintedStrResult<&TexValue> {
        std::iter::once(&self.top)
            .chain(self.scopes.iter().rev())
            .chain(self.base.map(|base| base.global.scope()))
            .find_map(|scope| scope.get(var))
            .ok_or_else(|| unknown_variable(var))
    }

    /// Try to access a variable immutably in math.
    pub fn get_in_math(&self, var: &str) -> HintedStrResult<&TexValue> {
        std::iter::once(&self.top)
            .chain(self.scopes.iter().rev())
            .chain(self.base.map(|base| base.math.scope()))
            .find_map(|scope| scope.get(var))
            .ok_or_else(|| unknown_variable(var))
    }

    /// Try to access a variable mutably.
    pub fn get_mut(&mut self, var: &str) -> HintedStrResult<&mut TexValue> {
        std::iter::once(&mut self.top)
            .chain(&mut self.scopes.iter_mut().rev())
            .find_map(|scope| scope.get_mut(var))
            .ok_or_else(
                || match self.base.and_then(|base| base.global.scope().get(var)) {
                    Some(_) => eco_format!("cannot mutate a constant: {}", var).into(),
                    _ => unknown_variable(var),
                },
            )?
    }
}

/// The error message when a variable is not found.
#[cold]
fn unknown_variable(var: &str) -> HintedString {
    let mut res = HintedString {
        message: eco_format!("unknown variable: {}", var),
        hints: vec![],
    };

    if matches!(var, "none" | "auto" | "false" | "true") {
        res.hints.push(eco_format!(
            "if you meant to use a literal, try adding a hash before it"
        ));
    } else if var.contains('-') {
        res.hints.push(eco_format!(
            "if you meant to use subtraction, try adding spaces around the minus sign",
        ));
    }

    res
}

/// A map from binding names to values.
#[derive(Default, Clone)]
pub struct TexValueAssignmentGroup {
    map: IndexMap<EcoString, TexValueSlot>,
    deduplicate: bool,
    kind: Option<TexDefnKind>,
}

impl TexValueAssignmentGroup {
    /// Create a new empty scope.
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new scope with duplication prevention.
    pub fn deduplicating() -> Self {
        Self {
            deduplicate: true,
            ..Default::default()
        }
    }

    /// Enter a new category.
    pub fn category(&mut self, category: TexDefnKind) {
        self.kind = Some(category);
    }

    /// Reset the category.
    pub fn reset_group(&mut self) {
        self.kind = None;
    }

    /// Bind a value to a name.
    #[track_caller]
    pub fn define(&mut self, name: impl Into<EcoString>, value: impl IntoTexValue) {
        let name = name.into();

        #[cfg(debug_assertions)]
        if self.deduplicate && self.map.contains_key(&name) {
            panic!("duplicate definition: {name}");
        }

        self.map.insert(
            name,
            TexValueSlot::new(value.into_value(), Kind::Normal, self.kind),
        );
    }

    /// Define a native function through a Rust type that shadows the function.
    pub fn define_func<T: NativeFunc>(&mut self) {
        let data = T::data();
        self.define(data.name, Func::from(data));
    }

    /// Define a native function with raw function data.
    pub fn define_func_with_data(&mut self, data: &'static NativeFuncData) {
        self.define(data.name, Func::from(data));
    }

    /// Define a native type.
    pub fn define_type<T: NativeType>(&mut self) {
        let data = T::data();
        self.define(data.name, Type::from(data));
    }

    /// Define a native element.
    pub fn define_elem<T: IsTexElem>(&mut self) {
        let data = T::data();
        self.define(data.name, ElementSchemaRef::from(data));
    }

    /// Define a module.
    pub fn define_module(&mut self, module: TexModuleEvaluation) {
        self.define(module.name().clone(), module);
    }

    /// Define a captured, immutable binding.
    pub fn define_captured(&mut self, var: impl Into<EcoString>, value: impl IntoTexValue) {
        self.map.insert(
            var.into(),
            TexValueSlot::new(value.into_value(), Kind::Captured, self.kind),
        );
    }

    /// Try to access a variable immutably.
    pub fn get(&self, var: &str) -> Option<&TexValue> {
        self.map.get(var).map(TexValueSlot::read)
    }

    /// Try to access a variable mutably.
    pub fn get_mut(&mut self, var: &str) -> Option<HintedStrResult<&mut TexValue>> {
        self.map
            .get_mut(var)
            .map(TexValueSlot::write)
            .map(|res| res.map_err(HintedString::from))
    }

    /// Get the category of a definition.
    pub fn get_category(&self, var: &str) -> Option<TexDefnKind> {
        self.map.get(var)?.category
    }

    /// Iterate over all definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&EcoString, &TexValue)> {
        self.map.iter().map(|(k, v)| (k, v.read()))
    }
}

impl Debug for TexValueAssignmentGroup {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Scope ")?;
        f.debug_map()
            .entries(self.map.iter().map(|(k, v)| (k, v.read())))
            .finish()
    }
}

impl Hash for TexValueAssignmentGroup {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.map.len());
        for item in &self.map {
            item.hash(state);
        }
        self.deduplicate.hash(state);
        self.kind.hash(state);
    }
}

/// Defines the associated scope of a Rust type.
pub trait NativeScope {
    /// The constructor function for the type, if any.
    fn constructor() -> Option<&'static NativeFuncData>;

    /// Get the associated scope for the type.
    fn scope() -> TexValueAssignmentGroup;
}

/// A slot where a value is stored.
#[derive(Clone, Hash)]
struct TexValueSlot {
    /// The stored value.
    value: TexValue,
    /// The kind of slot, determines how the value can be accessed.
    kind: Kind,
    /// The category of the slot.
    category: Option<TexDefnKind>,
}

/// The different kinds of slots.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Kind {
    /// A normal, mutable binding.
    Normal,
    /// A captured copy of another variable.
    Captured,
}

impl TexValueSlot {
    /// Create a new slot.
    fn new(value: TexValue, kind: Kind, category: Option<TexDefnKind>) -> Self {
        Self {
            value,
            kind,
            category,
        }
    }

    /// Read the value.
    fn read(&self) -> &TexValue {
        &self.value
    }

    /// Try to write to the value.
    fn write(&mut self) -> StrResult<&mut TexValue> {
        match self.kind {
            Kind::Normal => Ok(&mut self.value),
            Kind::Captured => {
                bail!(
                    "variables from outside the function are \
                     read-only and cannot be modified"
                )
            }
        }
    }
}

/// A group of related definitions.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TexDefnKind(Static<TexCategoryData>);

impl TexDefnKind {
    /// Create a new category from raw data.
    pub const fn from_data(data: &'static TexCategoryData) -> Self {
        Self(Static(data))
    }

    /// The category's name.
    pub fn name(&self) -> &'static str {
        self.0.name
    }

    /// The type's title case name, for use in documentation (e.g. `String`).
    pub fn title(&self) -> &'static str {
        self.0.title
    }

    /// Documentation for the category.
    pub fn docs(&self) -> &'static str {
        self.0.docs
    }
}

impl Debug for TexDefnKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Category({})", self.name())
    }
}

/// Defines a category.
#[derive(Debug)]
pub struct TexCategoryData {
    pub name: &'static str,
    pub title: &'static str,
    pub docs: &'static str,
}
