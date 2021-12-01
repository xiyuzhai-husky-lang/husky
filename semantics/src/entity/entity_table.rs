use super::*;
use common::*;
pub struct EntityTable {
  symbols: Vec<SymbolID>,
  subentities: Vec<EntityID>,
}

impl EntityTable {
  pub fn new_empty() -> EntityTable {
    EntityTable {
      symbols: Vec::<SymbolID>::new(),
      subentities: Vec::<EntityID>::new(),
    }
  }
  pub fn new(raw_subentities: Vec<Entity>, arena: &mut EntityArena) -> EntityTable {
    let mut symbols = Vec::<SymbolID>::new();
    let mut subentities = Vec::<EntityID>::new();
    for entity in raw_subentities {
      let mut seek_old = 0;
      while seek_old < symbols.len() {
        if symbols[seek_old] == entity.key {
          break;
        }
        seek_old += 1;
      }
      if seek_old < symbols.len() {
        let new_entity_variant = entity.variant;
        let old = subentities[seek_old];
        match arena[old].variant {
          EntityVariant::Opn(old_opn) => match old_opn {
            Opn::Main => todo!(),
            Opn::MemFunc => todo!(),
            Opn::MemLazy => todo!(),
            Opn::MemVar => todo!(),
            Opn::Func => {
              if let EntityVariant::Opn(new_opn) = new_entity_variant {
                assert!(new_opn == old_opn);
              } else {
                todo!()
              }
            }
            Opn::Pattern => todo!(),
          },
          EntityVariant::Type(_) => todo!(),
          EntityVariant::Module => todo!(),
        }
      } else {
        symbols.push(entity.key);
        subentities.push(arena.alloc(entity));
      }
    }

    EntityTable {
      symbols,
      subentities,
    }
    // EntityTable { subentities }
  }
  pub fn get(&self, arena: &[Entity], key: SymbolID) -> Option<EntityID> {
    td!()
    // self.table.get(&key).map(|e| *e)
  }
  pub fn map(&self, arena: &[Entity], f: &dyn Fn(&Entity)) {
    td!()
    // for i in self.start.value..self.end.value {
    //   f(&arena[i])
    // }
    // for (_, entity) in &self.table {
    //   f(entity);
    // }
  }
  pub fn set_parent(&self, arena: &[Entity], parent: EntityID) {
    td!()
    // for i in self.start.value..self.end.value {
    //   arena[i].init_parent_dfs(parent)
    // }
  }
  pub fn as_slice(&self) -> &[EntityID] {
    self.subentities.as_slice()
  }
}

impl Debug for EntityTable {
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
