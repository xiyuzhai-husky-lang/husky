use super::*;
use common::*;
use ovld_matcher::*;
use semantics::utils::*;
use semantics::*;
use syntax::phrase::{Phrase, PhraseVariant};
use syntax::sentence::*;
use syntax::token::*;
use syntax::*;
#[derive(Debug, Copy, Clone)]
enum Environment {
  Module { ast: &'repl AST },
  Opn,
  Type,
}

pub fn parse_ovld_matchers(
  ast: &AST,
  module: &Entity,
  root: &mut Project,
) -> Result<OvldMatchers, ParserError> {
  let mut ovld_matchers = HashMap::<*const Entity, OvldMatcher>::new();
  for sentence in filter_and_group(&ast.sentences) {
    dfs(sentence, &mut ovld_matchers, module, root);
  }
  for child in &ast.children {
    let submodule = throw_on_error!(module.child(&child.key), "expect a submodule".to_string());
    parse_ovld_matchers(&child, &submodule, root);
  }
  Ok(ovld_matchers)
}

fn resolve_entity(
  phrase: PhraseID,
  module: &Entity,
  root: &mut Project,
) -> Result<&'repl Entity, ParserError> {
  match &phrase.variant {
    PhraseVariant::Atom(_) => Ok(throw_on_error!(
      module.child(identify!(phrase)),
      "expect a type".to_string()
    )),
    PhraseVariant::Binary { opr, lopd, ropd } => match (opr) {
      Binary::Combine => {
        let tmpl_arguments = unbox!(lopd, &|phrase| td!());
        if tmpl_arguments.len() == 0 {
          let element = throw_on_error!(
            resolve_entity(ropd, module, root),
            "expect element type".to_string()
          );
          root.resolve_vector(element);
          td!()
        } else {
          td!()
        }
      }
      _ => {
        p!(opr);
        td!()
      }
    },
    PhraseVariant::Join { separator, phrases } => {
      if *separator == Join::ModuleQualifier {
        td!()
      } else {
        semantic_error!(
          phrase.range,
          "expect a type".to_string(),
          TypeResolveFailure
        )
        // p!(phrase);/4333
        // p!(separator);
        // // can't resolve entity
        // // don't put return varname anymore
        // td!()
      }
    }
    PhraseVariant::Prefix { opr, opd } => td!(),
    PhraseVariant::Suffix { opr, opd } => td!(),
    PhraseVariant::Bracket { bracket, opd } => td!(),
  }
}

fn filter_and_group<'b>(sentences: &'b Vec<Sentence>) -> Vec<Vec<&'b Sentence>> {
  let mut sentence_groups = Vec::<Vec<&'b Sentence>>::new();
  let mut iter = sentences.iter();
  loop {
    match iter.next() {
      Some(sentence) => match sentence.clause.keyword {
        Keyword::Pub => panic!(),
        Keyword::PubDef
        | Keyword::Def
        | Keyword::Pattern
        | Keyword::PubPattern
        | Keyword::Enum
        | Keyword::PubEnum => sentence_groups.push(vec![sentence]),
        Keyword::Func | Keyword::PubFunc => {
          let next_sentence = iter.next().expect("can't end now");
          assert!(next_sentence.clause.keyword == Keyword::Def);
          sentence_groups.push(vec![sentence, next_sentence])
        }
        Keyword::Template => td!(),
        Keyword::PubTemplate => td!(),
        Keyword::NoKeyword => td!(),
        _ => (),
      },
      None => break sentence_groups,
    }
  }
}

fn dfs(
  sentence_group: Vec<&'repl Sentence>,
  ovld_matchers: &mut OvldMatchers,
  module: &'repl Entity,
  root: &'repl mut Project,
) -> Result<(), ParserError> {
  match sentence_group.len() {
    1 => match sentence_group[0].clause.keyword {
      _ => td!(),
    },
    2 => match sentence_group[0].clause.keyword {
      Keyword::Func | Keyword::PubFunc => {
        let func = match &sentence_group[1].clause.phrase.variant {
          PhraseVariant::Atom(_) => td!(),
          PhraseVariant::Binary { opr, lopd, ropd } => {
            assert!(*opr == Binary::Call);
            throw_on_error!(
              resolve_entity(lopd, module, root),
              "expect call in second line".to_string()
            )
          }
          PhraseVariant::Join { separator, phrases } => td!(),
          PhraseVariant::Prefix { opr, opd } => td!(),
          PhraseVariant::Suffix { opr, opd } => td!(),
          PhraseVariant::Bracket { bracket, opd } => td!(),
        };
        let (inputs, output) = match &sentence_group[0].clause.phrase.variant {
          PhraseVariant::Binary { opr, lopd, ropd } => (
            unjoin!(
              lopd,
              Join::List,
              &|phrase: &'repl Phrase| -> Result<&'repl Entity, ParserError> {
                resolve_entity(phrase, module, root)
              },
              "list of types".to_string()
            ),
            throw_on_error!(resolve_entity(ropd, module, root), "whatever".to_string()),
          ),
          _ => td!(),
        };
        let pfunc: *const semantics::entity::Entity = &*func;
        let ovld_matcher = ovld_matchers.entry(pfunc).or_insert(OvldMatcher::new());
        ovld_matcher.add_ovld(Ovld { inputs, output });
        Ok(())
      }
      _ => {
        p!(sentence_group[0].clause.keyword);
        td!()
      }
    },
    _ => td!(),
  }
  // match &sentence.clause.phrase.variant {
  //   PhraseVariant::Atom(_) => td!(),
  //   PhraseVariant::Binary { opr, lopd, ropd } => td!(),
  //   PhraseVariant::Join { separator, phrases } => td!(),
  //   PhraseVariant::Prefix { opr, opd } => td!(),
  //   PhraseVariant::Suffix { opr, opd } => td!(),
  //   PhraseVariant::Bracket { bracket, opd } => td!(),
  // }
  // match &sentence.clause.keyword {
  //   Keyword::Pub => panic!(),
  //   Keyword::PubDef => td!(),
  //   Keyword::Def => add_func_ovld(&sentence.clause, ovld_matchers, &|phrase| td!()),
  //   Keyword::Pattern => td!(),
  //   Keyword::PubPattern => td!(),
  //   Keyword::Enum => td!(),
  //   Keyword::PubEnum => td!(),
  //   Keyword::Template => td!(),
  //   Keyword::PubTemplate => td!(),
  //   Keyword::NoKeyword => td!(),
  //   Keyword::Use
  //   | Keyword::Include
  //   | Keyword::Mod
  //   | Keyword::PubMod
  //   | Keyword::Func
  //   | Keyword::PubFunc
  //   | Keyword::Struct
  //   | Keyword::PubStruct
  //   | Keyword::Props
  //   | Keyword::PubProps
  //   | Keyword::Let
  //   | Keyword::Var
  //   | Keyword::Main
  //   | Keyword::If
  //   | Keyword::Elif
  //   | Keyword::Else
  //   | Keyword::Switch
  //   | Keyword::Case
  //   | Keyword::Default
  //   | Keyword::For
  //   | Keyword::Ext
  //   | Keyword::ForExt
  //   | Keyword::While
  //   | Keyword::Do
  //   | Keyword::DoWhile
  //   | Keyword::Break
  //   | Keyword::Return
  //   | Keyword::Comment
  //   | Keyword::NoKeyword => (),
  // }
}

fn add_func_ovld(
  clause: &Clause,
  ovld_matchers: &mut OvldMatchers,
  resolve_entity: &Fn(PhraseID) -> &Entity,
) {
  match clause.keyword {
    Keyword::Def => td!(),
    Keyword::PubDef => td!(),
    Keyword::Func => td!(),
    Keyword::PubFunc => td!(),
    _ => td!(),
  }
  match &clause.phrase.variant {
    PhraseVariant::Binary { opr, lopd, ropd } => match opr {
      Binary::Arrow => match &lopd.variant {
        PhraseVariant::Binary {
          opr,
          lopd: funcname,
          ropd: inputs,
        } => {
          let func = resolve_entity(funcname);
          td!()
        }
        _ => td!(),
      },
      Binary::Call => {
        let func = resolve_entity(lopd);
        td!()
      }
      _ => td!(),
    },
    PhraseVariant::Join { separator, phrases } => {
      // let func = resolve_entity(phrases[0]);
      td!()
    }
    _ => {
      p!(&clause.phrase);
      td!()
    }
  }
  // ovld_matchers.get(k)
  td!()
}
