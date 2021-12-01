use super::*;
use common::*;
use std::fmt::Write;

pub struct Project {
  entity_arena: EntityArena,
  package: EntityID,
}
impl Project {
  pub fn new(ast: &AST) -> Result<Project, ParserError> {
    let mut project = Project {
      entity_arena: EntityArena::new(),
      package: EntityID::NotDefined,
    };
    project.package = parse_package(ast, &mut project.entity_arena)?;
    Ok(project)
  }
  pub fn resolve_vector(&mut self, _element: &Entity) -> Entity {
    td!()
  }
}
impl Project {
  pub fn show(&self, sess: &Session) -> ShowCase {
    let mut result = String::new();
    write!(
      &mut result,
      "
Project {{
  package:
{}
}}",
      &Self::show_tree(sess, self.entity_arena.as_slice(), self.package, 2)
    )
    .unwrap();
    ShowCase::new(result)
  }
  fn show_tree(sess: &Session, arena: &[Entity], entity: EntityID, indent: usize) -> String {
    let mut result = String::new();
    for i in 0..indent {
      result.push(' ');
    }
    write!(
      result,
      "{}{:?}{}: {}\n",
      RED,
      entity.variant(arena),
      RESET,
      entity.key(sess, arena)
    )
    .expect("success");
    for subentity in entity.subentities(arena) {
      result.push_str(&Self::show_tree(sess, arena, *subentity, indent + 2));
    }
    result
  }
}
