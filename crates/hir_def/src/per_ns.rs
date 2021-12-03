//! In rust, it is possible to have a value, a type and a macro with the same
//! name without conflicts.
//!
//! `PerNs` (per namespace) captures this.

use hir_expand::MacroDefId;

use crate::{item_scope::ItemInNamespace, visibility::Visibility, ModuleDefId};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PerNamespace {
    pub types: Option<(ModuleDefId, Visibility)>,
    pub values: Option<(ModuleDefId, Visibility)>,
}

impl Default for PerNamespace {
    fn default() -> Self {
        PerNamespace {
            types: None,
            values: None,
        }
    }
}

impl PerNamespace {
    pub fn none() -> PerNamespace {
        PerNamespace {
            types: None,
            values: None,
        }
    }

    pub fn values(t: ModuleDefId, v: Visibility) -> PerNamespace {
        PerNamespace {
            types: None,
            values: Some((t, v)),
        }
    }

    pub fn types(t: ModuleDefId, v: Visibility) -> PerNamespace {
        PerNamespace {
            types: Some((t, v)),
            values: None,
        }
    }

    pub fn both(types: ModuleDefId, values: ModuleDefId, v: Visibility) -> PerNamespace {
        PerNamespace {
            types: Some((types, v)),
            values: Some((values, v)),
        }
    }

    pub fn macros(macro_: MacroDefId, v: Visibility) -> PerNamespace {
        PerNamespace {
            types: None,
            values: None,
        }
    }

    pub fn is_none(&self) -> bool {
        self.types.is_none() && self.values.is_none()
    }

    pub fn take_types(self) -> Option<ModuleDefId> {
        self.types.map(|it| it.0)
    }

    pub fn take_types_vis(self) -> Option<(ModuleDefId, Visibility)> {
        self.types
    }

    pub fn take_values(self) -> Option<ModuleDefId> {
        self.values.map(|it| it.0)
    }

    pub fn filter_visibility(self, mut f: impl FnMut(Visibility) -> bool) -> PerNamespace {
        let _p = profile::span("PerNs::filter_visibility");
        PerNamespace {
            types: self.types.filter(|(_, v)| f(*v)),
            values: self.values.filter(|(_, v)| f(*v)),
        }
    }

    pub fn with_visibility(self, vis: Visibility) -> PerNamespace {
        PerNamespace {
            types: self.types.map(|(it, _)| (it, vis)),
            values: self.values.map(|(it, _)| (it, vis)),
        }
    }

    pub fn or(self, other: PerNamespace) -> PerNamespace {
        PerNamespace {
            types: self.types.or(other.types),
            values: self.values.or(other.values),
        }
    }

    pub fn iter_items(self) -> impl Iterator<Item = ItemInNamespace> {
        let _p = profile::span("PerNs::iter_items");
        self.types
            .map(|it| ItemInNamespace::Types(it.0))
            .into_iter()
            .chain(
                self.values
                    .map(|it| ItemInNamespace::Values(it.0))
                    .into_iter(),
            )
    }
}
