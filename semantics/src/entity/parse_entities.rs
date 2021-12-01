use super::*;
use crate::utils::*;
use common::*;
use syntax::phrase::{Phrase, PhraseVariant};
use syntax::sentence::*;
use syntax::token::*;
use syntax::*;
#[derive(Copy, Clone)]
enum Environment<'a> {
  Module { ast: &'a AST },
  Opn,
  Type,
}
impl<'a> Debug for Environment<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Module { .. } => write!(f, "Module"),
      Self::Opn => write!(f, "Opn"),
      Self::Type => write!(f, "Type"),
    }
  }
}

fn parse_entities(
  p_a: &[Phrase],
  sentences: &Vec<Sentence>,
  env: Environment,
  arena: &mut EntityArena,
) -> Result<Vec<Entity>, ParserError> {
  let mut entities = Vec::<Entity>::new();
  for sentence in sentences {
    if let Some(entity) = parse_entity(p_a, sentence, env, arena)? {
      entities.push(entity);
    }
  }
  Ok(entities)
}

fn parse_entity(
  p_a: &[Phrase],
  sentence: &Sentence,
  env: Environment,
  arena: &mut EntityArena,
) -> Result<Option<Entity>, ParserError> {
  let clause = &sentence.clause;
  match clause.keyword {
    Keyword::Struct => Ok(Some(parse_struct(p_a, sentence, false, arena)?)),
    Keyword::PubStruct => Ok(Some(parse_struct(p_a, sentence, true, arena)?)),
    Keyword::Rename => Ok(Some(parse_rename(p_a, sentence, false)?)),
    Keyword::PubRename => Ok(Some(parse_rename(p_a, sentence, true)?)),
    Keyword::Enum => Ok(Some(parse_enum(p_a, sentence, false, arena)?)),
    Keyword::PubEnum => Ok(Some(parse_enum(p_a, sentence, true, arena)?)),
    Keyword::Def => Ok(Some(parse_func_entity(p_a, sentence, false)?)),
    Keyword::PubDef => Ok(Some(parse_func_entity(p_a, sentence, true)?)),
    Keyword::Pattern => Ok(Some(parse_pattern_entity(p_a, sentence, false)?)),
    Keyword::PubPattern => Ok(Some(parse_pattern_entity(p_a, sentence, true)?)),
    Keyword::Main => Ok(Some(Entity::new(
      SymbolID::Keyword(Keyword::Main),
      parse_entities(p_a, &sentence.body, Environment::Opn, arena)?,
      EntityVariant::Opn(Opn::Main),
      arena,
    ))),
    Keyword::NoKeyword => match env {
      Environment::Opn => {
        assert!(sentence.body.len() == 0);
        Ok(None)
      }
      Environment::Type => {
        if sentence.body.len() == 0 {
          let mem_varname = identify!(unpair!(sentence.clause.phrase, p_a).1, p_a);
          Ok(Some(Entity::new(
            mem_varname.clone(),
            vec![],
            EntityVariant::Opn(Opn::MemVar),
            arena,
          )))
        } else {
          td!();
        }
      }
      _ => {
        p!(env);
        p!(sentence.clause.phrase);
        td!();
      }
    },
    Keyword::PubMod => {
      if let Environment::Module { ast } = env {
        Ok(Some(parse_submodule(p_a, ast, sentence, true, arena)?))
      } else {
        // assert!(env == Environment::Module);
        td!()
      }
    }
    Keyword::Let
    | Keyword::Var
    | Keyword::If
    | Keyword::Elif
    | Keyword::Else
    | Keyword::Switch
    | Keyword::Case
    | Keyword::Default
    | Keyword::For
    | Keyword::ForExt
    | Keyword::While
    | Keyword::DoWhile
    | Keyword::Comment
    | Keyword::Func
    | Keyword::PubFunc
    | Keyword::Use => Ok(None),
    _ => {
      p!(clause.keyword);
      td!()
    } // print(clause.keyword.cls);
      // td;
  }
}

pub fn parse_package(ast: &AST, arena: &mut EntityArena) -> Result<EntityID, ParserError> {
  let subentities = parse_entities(
    ast.phrase_arena.as_slice(),
    &ast.sentences,
    Environment::Module { ast },
    arena,
  )?;
  let package = Entity::new(ast.key, subentities, EntityVariant::Module, arena);
  Ok(arena.alloc(package))
}

fn parse_struct(
  p_a: &[Phrase],
  sentence: &Sentence,
  visibility: bool,
  arena: &mut EntityArena,
) -> Result<Entity, ParserError> {
  match sentence.clause.phrase.variant(p_a) {
    PhraseVariant::Atom(atom) => Ok(Entity::new(
      identify!(sentence.clause.phrase, p_a),
      parse_entities(p_a, &sentence.body, Environment::Type, arena)?,
      EntityVariant::Type(Type::Struct),
      arena,
    )),
    PhraseVariant::Binary { opr, lopd, ropd } => {
      assert!(*opr == Binary::Be);
      Ok(Entity::new(
        identify!(*lopd, p_a),
        vec![],
        EntityVariant::Type(Type::Rename),
        arena,
      ))
    }
    PhraseVariant::Join { .. } => {
      let (name, mem_vars) = unpair!(sentence.clause.phrase, p_a);
      Ok(Entity::new(
        identify!(name, p_a),
        parse_inline_struct_members(p_a, mem_vars),
        EntityVariant::Type(Type::Struct),
        arena,
      ))
    }
    _ => td!(),
  }
}

fn parse_inline_struct_members(p_a: &[Phrase], phrase: PhraseID) -> Vec<Entity> {
  uncurl!(phrase, p_a, &mut |phrase: PhraseID, p_a: &[Phrase]| {
    Entity::new_leaf(
      identify!(unpair!(phrase, p_a).1, p_a).clone(),
      EntityVariant::Opn(Opn::MemVar),
    )
  })
}

fn parse_enum(
  p_a: &[Phrase],
  sentence: &Sentence,
  visibility: bool,
  arena: &mut EntityArena,
) -> Result<Entity, ParserError> {
  fn parse_inline_enum_items(
    phrase: PhraseID,
    p_a: &[Phrase],
    arena: &mut EntityArena,
  ) -> Vec<Entity> {
    uncurl!(phrase, p_a, &mut |phrase, p_a| Entity::new(
      identify!(phrase, p_a).clone(),
      vec![],
      EntityVariant::Opn(Opn::MemVar),
      arena
    ))
  }
  let (tyname, items) = unpair!(sentence.clause.phrase, p_a);
  Ok(Entity::new(
    identify!(tyname, p_a).clone(),
    parse_inline_enum_items(items, p_a, arena),
    EntityVariant::Type(Type::Enum),
    arena,
  ))
}

fn parse_func_entity(
  p_a: &[Phrase],
  sentence: &Sentence,
  visibility: bool,
) -> Result<Entity, ParserError> {
  let funcname = match &sentence.clause.phrase.variant(p_a) {
    PhraseVariant::Binary { opr, lopd, ropd } => match opr {
      Binary::Arrow => match &lopd.variant(p_a) {
        PhraseVariant::Binary {
          opr,
          lopd: funcname,
          ..
        } => {
          assert!(*opr == Binary::Call);
          identify!(*funcname, p_a)
        }
        _ => td!(),
      },
      Binary::Call => identify!(*lopd, p_a),
      _ => td!(),
    },
    PhraseVariant::Join { separator, phrases } => {
      identify!(phrases[0], p_a)
    }
    _ => {
      p!(&sentence.clause.phrase);
      td!()
    }
  };
  Ok(Entity::new_leaf(funcname, EntityVariant::Opn(Opn::Func)))
}

fn parse_pattern_entity(
  p_a: &[Phrase],
  sentence: &Sentence,
  visibility: bool,
) -> Result<Entity, ParserError> {
  let pattern_name = match sentence.clause.phrase.variant(p_a) {
    PhraseVariant::Binary { opr, lopd, ropd } => match opr {
      Binary::Arrow => match lopd.variant(p_a) {
        PhraseVariant::Binary {
          opr,
          lopd: pattern_name,
          ..
        } => {
          assert!(*opr == Binary::Call);
          identify!(*pattern_name, p_a)
        }
        _ => td!(),
      },
      Binary::Call => identify!(*lopd, p_a),
      _ => td!(),
    },
    PhraseVariant::Join { separator, phrases } => {
      identify!(phrases[0], p_a)
    }
    _ => {
      p!(&sentence.clause.phrase);
      td!()
    }
  };
  Ok(Entity::new_leaf(
    pattern_name.clone(),
    EntityVariant::Opn(Opn::Func),
  ))
}

fn parse_submodule(
  p_a: &[Phrase],
  parent_ast: &AST,
  sentence: &Sentence,
  visibility: bool,
  arena: &mut EntityArena,
) -> Result<Entity, ParserError> {
  fn get_submodule_filepath(parent_filepath: &PathBuf, submodule_name: &String) -> PathBuf {
    let mut result = parent_filepath.clone();
    result.pop();
    result.push(submodule_name);
    if result.exists() {
      result.push("mod.hsk");
      p!(result);
      assert!(result.exists());
      result
    } else {
      result.set_extension("hsk");
      assert!(result.exists());
      result
    }
  }

  let submodule_name = identify!(sentence.clause.phrase, p_a);
  let submodule_ast = parent_ast.get_child(submodule_name);
  Ok(Entity::new(
    submodule_name,
    parse_entities(
      submodule_ast.phrase_arena.as_slice(),
      &submodule_ast.sentences,
      Environment::Module {
        ast: &submodule_ast,
      },
      arena,
    )?,
    EntityVariant::Module,
    arena,
  ))
}

fn parse_rename(
  p_a: &[Phrase],
  sentence: &Sentence,
  visibility: bool,
) -> Result<Entity, ParserError> {
  if let PhraseVariant::Binary { opr, lopd, ropd } = sentence.clause.phrase.variant(p_a) {
    Ok(Entity::new_leaf(
      identify!(*lopd, p_a),
      EntityVariant::Type(Type::Rename),
    ))
  } else {
    td!()
    // Err(e);
  }
}
