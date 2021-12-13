use super::*;
use common::*;
pub struct ScopeTable {
    symbols: Vec<SymbolID>,
    subscopes: Vec<ScopeID>,
}

impl ScopeTable {
    pub fn new_empty() -> ScopeTable {
        ScopeTable {
            symbols: Vec::<SymbolID>::new(),
            subscopes: Vec::<ScopeID>::new(),
        }
    }
    pub fn new(raw_subscopes: Vec<Scope>, arena: &mut ScopeArena) -> ScopeTable {
        let mut symbols = Vec::<SymbolID>::new();
        let mut subscopes = Vec::<ScopeID>::new();
        for scope in raw_subscopes {
            let mut seek_old = 0;
            while seek_old < symbols.len() {
                if symbols[seek_old] == scope.key {
                    break;
                }
                seek_old += 1;
            }
            if seek_old < symbols.len() {
                let new_scope_variant = scope.variant;
                let old = subscopes[seek_old];
                match arena[old].variant {
                    ScopeVariant::Opn(old_opn) => match old_opn {
                        Opn::Main => todo!(),
                        Opn::MemFunc => todo!(),
                        Opn::MemLazy => todo!(),
                        Opn::MemVar => todo!(),
                        Opn::Func => {
                            if let ScopeVariant::Opn(new_opn) = new_scope_variant {
                                assert!(new_opn == old_opn);
                            } else {
                                todo!()
                            }
                        }
                        Opn::Pattern => todo!(),
                    },
                    ScopeVariant::Type(_) => todo!(),
                    ScopeVariant::Module => todo!(),
                }
            } else {
                symbols.push(scope.key);
                subscopes.push(arena.alloc(scope));
            }
        }

        ScopeTable { symbols, subscopes }
        // ScopeTable { subscopes }
    }
    pub fn get(&self, arena: &[Scope], key: SymbolID) -> Option<ScopeID> {
        td!()
        // self.table.get(&key).map(|e| *e)
    }
    pub fn map(&self, arena: &[Scope], f: &dyn Fn(&Scope)) {
        td!()
        // for i in self.start.value..self.end.value {
        //   f(&arena[i])
        // }
        // for (_, scope) in &self.table {
        //   f(scope);
        // }
    }
    pub fn set_parent(&self, arena: &[Scope], parent: ScopeID) {
        td!()
        // for i in self.start.value..self.end.value {
        //   arena[i].init_parent_dfs(parent)
        // }
    }
    pub fn as_slice(&self) -> &[ScopeID] {
        self.subscopes.as_slice()
    }
}

impl Debug for ScopeTable {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        td!()
        // fmt.write_fmt(format_args!(
        //   "dict[{}]",
        //   self
        //     .table
        //     .iter()
        //     .map(|(k, v)| format!("{:?}: {:?}", k, v))
        //     .collect::<Vec<String>>()
        //     .join(", ")
        // ))
    }
}
