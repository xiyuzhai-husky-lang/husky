use super::*;
use common::*;
use std::fmt::Write;

pub struct Project {
    scope_arena: ScopeArena,
    package: ScopeID,
}
impl Project {
    pub fn new(ast: &AST) -> Result<Project, ParserError> {
        let mut project = Project {
            scope_arena: ScopeArena::new(),
            package: ScopeID::NotDefined,
        };
        project.package = parse_package(ast, &mut project.scope_arena)?;
        Ok(project)
    }
    pub fn resolve_vector(&mut self, _element: &Scope) -> Scope {
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
            &Self::show_tree(sess, self.scope_arena.as_slice(), self.package, 2)
        )
        .unwrap();
        ShowCase::new(result)
    }
    fn show_tree(sess: &Session, arena: &[Scope], scope: ScopeID, indent: usize) -> String {
        let mut result = String::new();
        for i in 0..indent {
            result.push(' ');
        }
        write!(
            result,
            "{}{:?}{}: {}\n",
            RED,
            scope.variant(arena),
            RESET,
            scope.key(sess, arena)
        )
        .expect("success");
        for subscope in scope.subscopes(arena) {
            result.push_str(&Self::show_tree(sess, arena, *subscope, indent + 2));
        }
        result
    }
}
