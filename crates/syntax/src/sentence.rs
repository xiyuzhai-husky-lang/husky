use super::*;
use common::*;

#[derive(Debug)]
pub struct Sentence {
  pub clause: Clause,
  pub body: Vec<Sentence>,
  pub i_end: usize,
}

fn get_line_group_end(source: &HuskySource, start: usize) -> usize {
  match source.indents[start] {
    file::Indent::None => panic!(),
    file::Indent::Indent(start_indent) => {
      let mut end = start + 1;
      while end < source.indents.len() {
        match source.indents[end] {
          file::Indent::None => end += 1,
          file::Indent::Indent(end_indent) => {
            if end_indent > start_indent + 2 {
              end += 1;
            } else {
              break;
            }
          }
        }
      }
      end
    }
  }
}

fn parse_sentence(
  source: &HuskySource,
  start: usize,
  sess: &mut Session,
  phrase_arena: &mut PhraseArena,
) -> Result<(Sentence, usize), ParserError> {
  match source.indents[start] {
    file::Indent::None => panic!("what"),
    file::Indent::Indent(indent) => {
      if indent % 2 != 0 {
        syntax_error!(
          file::Range {
            start: file::Position { i: start, j: 0 },
            end: file::Position {
              i: start,
              j: source.lines[start].len(),
            },
          },
          "incorrect indent, first line in a line group must have even indent".to_string(),
          IndentError
        )
      } else {
        let end = get_line_group_end(source, start);
        let sentences = parse_sentence_block(source, end, indent + 2, sess, phrase_arena)?;
        let tokens = token::lex_tokens(source, start, end, sess)?;
        let clause = Clause::parse_clause(sentences.len() > 0, tokens, phrase_arena)?;
        let i_end = if sentences.len() > 0 {
          sentences.last().unwrap().i_end
        } else {
          end
        };
        Ok((
          sentence::Sentence {
            clause,
            body: sentences,
            i_end,
          },
          i_end,
        ))
      }
    }
  }
}

#[derive(Debug)]
pub struct Clause {
  pub keyword: Keyword,
  pub phrase: PhraseID,
}
impl Clause {
  fn parse_head_clause(
    phrase_arena: &mut PhraseArena,
    stream: &mut token::output::TokenStream,
  ) -> Result<Clause, ParserError> {
    let keyword = stream.pop();
    assert!(matches!(keyword.cls, TokenVariant::Keyword(_)));
    if let TokenVariant::Keyword(kw) = &keyword.cls {
      match kw {
        Keyword::Struct
        | Keyword::PubStruct
        | Keyword::Def
        | Keyword::PubDef
        | Keyword::Pattern
        | Keyword::PubPattern
        | Keyword::If
        | Keyword::Elif
        | Keyword::Else
        | Keyword::For
        | Keyword::ForExt
        | Keyword::While
        | Keyword::DoWhile
        | Keyword::NoKeyword
        | Keyword::Switch
        | Keyword::Case
        | Keyword::Default
        | Keyword::Main => {
          assert!(!stream.is_end());
          let phrase = phrase::parse_phrase(phrase_arena, stream)?;
          assert!(stream.is_end());
          Ok(Clause {
            keyword: *kw,
            phrase,
          })
        }
        _ => todo!(),
      }
    } else {
      panic!();
    }
  }
  fn parse_body_clause(
    phrase_arena: &mut PhraseArena,
    stream: &mut token::output::TokenStream,
  ) -> Result<Clause, ParserError> {
    let keyword = stream.pop();
    assert!(matches!(keyword.cls, TokenVariant::Keyword(_)));
    if let TokenVariant::Keyword(kw) = &keyword.cls {
      match kw {
        Keyword::Use
        | Keyword::Mod
        | Keyword::PubMod
        | Keyword::Struct
        | Keyword::PubStruct
        | Keyword::Rename
        | Keyword::PubRename
        | Keyword::Enum
        | Keyword::PubEnum
        | Keyword::Func
        | Keyword::PubFunc
        | Keyword::NoKeyword
        | Keyword::Let
        | Keyword::Var
        | Keyword::Switch
        | Keyword::Case
        | Keyword::Return
        | Keyword::Break
        | Keyword::Comment => {
          assert!(!stream.is_end());
          let phrase = phrase::parse_phrase(phrase_arena, stream)?;
          assert!(stream.is_end());
          Ok(Clause {
            keyword: *kw,
            phrase,
          })
        }
        Keyword::PubDef => syntax_error!(
          keyword.range,
          "pub def should be head".to_string(),
          WrongKeyword
        ),
        _ => {
          p!(kw);
          todo!()
        }
      }
    } else {
      panic!();
    }
  }
  fn parse_clause(
    has_body: bool,
    mut stream: token::output::TokenStream,
    phrase_arena: &mut PhraseArena,
  ) -> Result<Clause, ParserError> {
    assert!(matches!(stream.top().cls, token::TokenVariant::Keyword(_)));
    if has_body {
      Self::parse_head_clause(phrase_arena, &mut stream)
    } else {
      Self::parse_body_clause(phrase_arena, &mut stream)
    }
  }
}
fn parse_sentence_block(
  source: &HuskySource,
  start: usize,
  indent: usize,
  sess: &mut Session,
  phrase_arena: &mut PhraseArena,
) -> Result<Vec<Sentence>, ParserError> {
  let mut sentences = Vec::<Sentence>::new();
  let mut i = start;
  while i < source.lines.len() && source.indents[i] == file::Indent::Indent(indent) {
    let (sentence, i_end) = parse_sentence(source, i, sess, phrase_arena)?;
    sentences.push(sentence);
    i = i_end;
  }
  if i < source.lines.len() {
    match source.indents[i] {
      file::Indent::None => Ok(sentences),
      file::Indent::Indent(indent2) => {
        if indent2 > indent {
          syntax_error!(
            file::Range {
              start: file::Position { i, j: 0 },
              end: file::Position {
                i,
                j: source.lines[i].len(),
              },
            },
            "nothing to say".to_string(),
            IndentError
          )
        } else {
          Ok(sentences)
        }
      }
    }
  } else {
    Ok(sentences)
  }
}

pub fn parse_sentences(
  source: &HuskySource,
  sess: &mut Session,
  phrase_arena: &mut PhraseArena,
) -> Result<Vec<Sentence>, ParserError> {
  match parse_sentence_block(source, 0, 0, sess, phrase_arena) {
    Ok(sentences) => Ok(sentences),
    Err(e) => {
      println!(
        "{}Syntax error emitted from: {}{}:{}{}{}",
        RED, GREEN, e.call_stack[0].file, YELLOW, e.call_stack[0].line, RESET
      );
      let mut result: String = "".to_string();
      result += CYAN;
      result += source.filepath.as_os_str().to_str().unwrap();
      result += "\n\n";
      result += RESET;
      result += &format_parser_error(source, &e);
      println!("{}\n{}{:?}{}", result, RED, &e, RESET);
      Err(e)
    }
  }
}
const RESET: &str = "\x1B[0m";
// const BLACK: &str = "\x1B[30m";
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const YELLOW: &str = "\x1B[33m";
// const BLUE: &str = "\x1B[34m";
// const MAGENTA: &str = "\x1B[35m";
const CYAN: &str = "\x1B[36m";
// const WHITE: &str = "\x1B[37m";
// const BOLD_BLACK: &str = "\033[1m\033[30m";
pub fn format_parser_error(source: &HuskySource, e: &ParserError) -> String {
  let mut result: String = "".to_string();
  let mut i = std::cmp::max(0, e.range.start.i - 2);
  while i < source.lines.len() && i <= e.range.end.i + 2 {
    let line = &source.lines[i];
    let ln = i.to_string() + ": ";
    result += &ln;
    // let shift=result.len()
    result += &line;
    result += "\n";
    if i >= e.range.start.i && i <= e.range.end.i {
      result += &String::from_utf8(vec![b' '; ln.len()]).unwrap();
      result += RED;
      let mut j = 0;
      if i == e.range.start.i {
        while j < e.range.start.j {
          j += 1;
          result += " ";
        }
      }
      while j < source.lines[i].len() && !(i == e.range.end.i && j >= e.range.end.j) {
        j += 1;
        result += "^";
      }
      result += "\u{2191}";
      result += RESET;
      result += "\n";
    }
    i += 2;
  }
  result
}
