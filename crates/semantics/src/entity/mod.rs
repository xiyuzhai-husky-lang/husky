mod entity_table;
mod entity_variant;
mod parse_entities;

pub use entity_variant::*;

use common::*;
use std::ops::Index;

#[derive(Debug)]
pub struct Entity {
  pub id: EntityID,
  pub key: SymbolID,
  pub subentities: EntityTable,
  pub variant: EntityVariant,
  pub parent: EntityID,
}
impl Entity {
  pub fn new(
    key: SymbolID,
    subentities: Vec<Entity>,
    variant: EntityVariant,
    arena: &mut EntityArena,
  ) -> Entity {
    Entity {
      id: EntityID::NotDefined,
      key,
      subentities: EntityTable::new(subentities, arena),
      variant,
      parent: EntityID::NotDefined,
    }
  }
  pub fn new_leaf(key: SymbolID, variant: EntityVariant) -> Entity {
    Entity {
      id: EntityID::NotDefined,
      key,
      subentities: EntityTable::new_empty(),
      variant,
      parent: EntityID::NotDefined,
    }
  }
  pub fn child(&self, arena: &[Entity], key: SymbolID) -> Option<EntityID> {
    self.subentities.get(arena, key)
  }
  pub fn init_parent(&mut self, arena: &[Entity]) {
    self.init_parent_dfs(arena, EntityID::NotDefined);
  }
  fn init_parent_dfs(&mut self, arena: &[Entity], parent: EntityID) {
    self.parent = parent;
    assert!(self.id != EntityID::NotDefined);
    self.subentities.set_parent(arena, self.id)
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

pub enum EntityID {
  NotDefined,
  Builtin(usize),
  UserDefined(usize),
}

impl EntityID {
  pub fn subentities<'a>(&self, arena: &'a [Entity]) -> &'a [EntityID] {
    get_entity(*self, arena).subentities.as_slice()
  }
  pub fn key<'a>(self, sess: &'a Session, arena: &[Entity]) -> &'a str {
    sess
      .get_symbol_str(get_entity(self, arena).key)
      .expect("should")
  }
  pub fn variant(self, arena: &[Entity]) -> &EntityVariant {
    &get_entity(self, arena).variant
  }
}

fn get_entity<'a>(id: EntityID, arena: &'a [Entity]) -> &'a Entity {
  match id {
    EntityID::NotDefined => panic!(),
    EntityID::Builtin(index) => &BUILTIN.entities[index],
    EntityID::UserDefined(index) => &arena[index],
  }
}

pub struct EntityArena {
  items: Vec<Entity>,
}

impl EntityArena {
  fn alloc(&mut self, mut entity: Entity) -> EntityID {
    let id = EntityID::UserDefined(self.items.len());
    entity.id = id;
    self.items.push(entity);
    id
  }
  pub fn new() -> EntityArena {
    EntityArena {
      items: Vec::<Entity>::new(),
    }
  }
  pub fn as_slice(&self) -> &[Entity] {
    self.items.as_slice()
  }
}

impl Index<EntityID> for EntityArena {
  type Output = Entity;

  fn index(&self, id: EntityID) -> &Self::Output {
    match id {
      EntityID::NotDefined => panic!(),
      EntityID::Builtin(index) => &BUILTIN.entities[index],
      EntityID::UserDefined(index) => &self.items[index],
    }
  }
}
static BUILTIN: Builtin = Builtin::new();

pub struct BuiltinIDs {
  int_type: EntityID,
  float_type: EntityID,
  bool_type: EntityID,
}
pub struct Builtin {
  ids: BuiltinIDs,
  entities: Vec<Entity>,
}
impl Builtin {
  pub const fn new() -> Builtin {
    let int_type = EntityID::Builtin(1);
    let float_type = EntityID::Builtin(1);
    let bool_type = EntityID::Builtin(1);
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

pub use entity_table::EntityTable;
pub use entity_variant::*;
pub use parse_entities::parse_package;
