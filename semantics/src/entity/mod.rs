mod parse_entities;
mod scope_table;
mod scope_variant;

pub use scope_variant::*;

use common::*;
use std::ops::Index;

#[derive(Debug)]
pub struct Scope {
    pub id: ScopeID,
    pub key: SymbolID,
    pub subscopes: ScopeTable,
    pub variant: ScopeVariant,
    pub parent: ScopeID,
}
impl Scope {
    pub fn new(
        key: SymbolID,
        subscopes: Vec<Scope>,
        variant: ScopeVariant,
        arena: &mut ScopeArena,
    ) -> Scope {
        Scope {
            id: ScopeID::NotDefined,
            key,
            subscopes: ScopeTable::new(subscopes, arena),
            variant,
            parent: ScopeID::NotDefined,
        }
    }
    pub fn new_leaf(key: SymbolID, variant: ScopeVariant) -> Scope {
        Scope {
            id: ScopeID::NotDefined,
            key,
            subscopes: ScopeTable::new_empty(),
            variant,
            parent: ScopeID::NotDefined,
        }
    }
    pub fn child(&self, arena: &[Scope], key: SymbolID) -> Option<ScopeID> {
        self.subscopes.get(arena, key)
    }
    pub fn init_parent(&mut self, arena: &[Scope]) {
        self.init_parent_dfs(arena, ScopeID::NotDefined);
    }
    fn init_parent_dfs(&mut self, arena: &[Scope], parent: ScopeID) {
        self.parent = parent;
        assert!(self.id != ScopeID::NotDefined);
        self.subscopes.set_parent(arena, self.id)
        // match self.variant {
        //   _ => {
        //     Module
        //     p!(self.variant);
        //     td!()
        //   }
        // }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]

pub enum ScopeID {
    NotDefined,
    Builtin(usize),
    UserDefined(usize),
}

impl ScopeID {
    pub fn subscopes<'a>(&self, arena: &'a [Scope]) -> &'a [ScopeID] {
        get_scope(*self, arena).subscopes.as_slice()
    }
    pub fn key<'a>(self, sess: &'a Session, arena: &[Scope]) -> &'a str {
        sess.get_symbol_str(get_scope(self, arena).key)
            .expect("should")
    }
    pub fn variant(self, arena: &[Scope]) -> &ScopeVariant {
        &get_scope(self, arena).variant
    }
}

fn get_scope<'a>(id: ScopeID, arena: &'a [Scope]) -> &'a Scope {
    match id {
        ScopeID::NotDefined => panic!(),
        ScopeID::Builtin(index) => &BUILTIN.entities[index],
        ScopeID::UserDefined(index) => &arena[index],
    }
}

pub struct ScopeArena {
    items: Vec<Scope>,
}

impl ScopeArena {
    fn alloc(&mut self, mut scope: Scope) -> ScopeID {
        let id = ScopeID::UserDefined(self.items.len());
        scope.id = id;
        self.items.push(scope);
        id
    }
    pub fn new() -> ScopeArena {
        ScopeArena {
            items: Vec::<Scope>::new(),
        }
    }
    pub fn as_slice(&self) -> &[Scope] {
        self.items.as_slice()
    }
}

impl Index<ScopeID> for ScopeArena {
    type Output = Scope;

    fn index(&self, id: ScopeID) -> &Self::Output {
        match id {
            ScopeID::NotDefined => panic!(),
            ScopeID::Builtin(index) => &BUILTIN.entities[index],
            ScopeID::UserDefined(index) => &self.items[index],
        }
    }
}
static BUILTIN: Builtin = Builtin::new();

pub struct BuiltinIDs {
    int_type: ScopeID,
    float_type: ScopeID,
    bool_type: ScopeID,
}
pub struct Builtin {
    ids: BuiltinIDs,
    entities: Vec<Scope>,
}
impl Builtin {
    pub const fn new() -> Builtin {
        let int_type = ScopeID::Builtin(1);
        let float_type = ScopeID::Builtin(1);
        let bool_type = ScopeID::Builtin(1);
        Builtin {
            ids: BuiltinIDs {
                int_type,
                float_type,
                bool_type,
            },
            entities: vec![],
        }
    }
}

pub use parse_entities::parse_package;
pub use scope_table::ScopeTable;
pub use scope_variant::*;
