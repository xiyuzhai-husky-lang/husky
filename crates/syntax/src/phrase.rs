use crate::*;
use common::*;

pub enum PhraseVariant {
  Atom(Atom),
  Binary {
    opr: Binary,
    lopd: PhraseID,
    ropd: PhraseID,
  },
  Join {
    separator: Join,
    phrases: Vec<PhraseID>,
  },
  Prefix {
    opr: Prefix,
    opd: PhraseID,
  },
  Suffix {
    opr: Suffix,
    opd: PhraseID,
  },
  Bracket {
    bracket: Bracket,
    opd: PhraseID,
  },
}
impl Debug for PhraseVariant {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PhraseVariant::Atom(_) => f.write_str("Atom"),
      PhraseVariant::Binary { opr, .. } => f.write_fmt(format_args!("Binary({})", opr.code())),
      PhraseVariant::Join { .. } => f.write_str("Join"),
      PhraseVariant::Prefix { .. } => todo!(),
      PhraseVariant::Suffix { .. } => todo!(),
      PhraseVariant::Bracket { .. } => f.write_str("Bracket"),
    }
  }
}
pub struct Phrase {
  pub range: file::Range,
  pub variant: PhraseVariant,
}
impl Phrase {
  pub fn unjoin_map<T>(
    arena: &[Phrase],
    phrase: PhraseID,
    expected_separator: Join,
    f: &dyn Fn(PhraseID) -> Result<T, ParserError>,
  ) -> Result<Vec<T>, ParserError> {
    if let PhraseVariant::Join { separator, .. } = phrase.variant(arena) {
      assert_eq!(expected_separator, *separator);
      td!()
      // phrases.iter().map(f).collect()
    } else {
      Ok(vec![f(phrase)?])
    }
  }
  // fn code(&self, arena: PhraseIDArena) -> String {
  //   match &self.variant {
  //     PhraseVariant::Atom(atom) => match atom {
  //       Atom::Void => "()".to_string(),
  //       Atom::Identifier(identifier) => identifier.clone(),
  //       Atom::Int(_) => todo!(),
  //       Atom::Float(_) => todo!(),
  //       Atom::Bool(_) => todo!(),
  //     },
  //     PhraseVariant::Binary { opr, lopd, ropd } => {
  //       fn process(symbol: String) -> String {
  //         if symbol == "" || symbol == " " {
  //           symbol
  //         } else {
  //           format!(" {} ", symbol)
  //         }
  //       }
  //       format!(
  //         "{}{}{}",
  //         arena[lopd]),
  //         process(opr.code(arena[lopd])),
  //         ropd.code(arena)
  //       )
  //     }
  //     PhraseVariant::Join { separator, phrases } => phrases
  //       .iter()
  //       .map(|phrase| phrase.code())
  //       .collect::<Vec<String>>()
  //       .join(&separator.code()),
  //     PhraseVariant::Prefix { opr, opd } => todo!(),
  //     PhraseVariant::Suffix { opr, opd } => todo!(),
  //     PhraseVariant::Bracket { bracket, opd } => match bracket {
  //       Bracket::Par => format!("({})", opd.code()),
  //       Bracket::Box => format!("[{}]", opd.code()),
  //       Bracket::Curl => format!("{{{}}}", opd.code()),
  //     },
  //   }
  // }
}
pub struct PhraseArena {
  arena: Vec<Phrase>,
}
impl PhraseArena {
  pub fn new() -> PhraseArena {
    PhraseArena { arena: vec![] }
  }
  pub fn alloc(&mut self, phrase: Phrase) -> PhraseID {
    self.arena.push(phrase);
    PhraseID {
      index: self.arena.len() - 1,
    }
  }
  pub fn as_slice(&self) -> &[Phrase] {
    self.arena.as_slice()
  }
}
impl PhraseArena {
  fn atom(&mut self, token: Token) -> PhraseID {
    if let TokenVariant::Atom(atom) = token.cls {
      self.alloc(Phrase {
        range: token.range,
        variant: PhraseVariant::Atom(atom),
      })
    } else {
      panic!();
    }
  }
  fn join(&mut self, separator: Join, phrases: Vec<PhraseID>) -> PhraseID {
    self.alloc(Phrase {
      range: file::Range::combine(
        phrases.first().unwrap().range(self.as_slice()),
        phrases.last().unwrap().range(self.as_slice()),
      ),
      variant: PhraseVariant::Join { separator, phrases },
    })
  }
  fn prefix(&mut self, start: file::Position, opr: Prefix, phrase: PhraseID) -> PhraseID {
    self.alloc(Phrase {
      range: file::Range {
        start,
        end: phrase.range(self.as_slice()).end,
      },
      variant: PhraseVariant::Prefix { opr, opd: phrase },
    })
  }
  fn suffix(&mut self, phrase: PhraseID, opr: Suffix, end: file::Position) -> PhraseID {
    self.alloc(Phrase {
      range: file::Range {
        start: phrase.range(self.as_slice()).start,
        end,
      },
      variant: PhraseVariant::Suffix { opd: phrase, opr },
    })
  }
  fn binary(&mut self, lopd: PhraseID, opr: Binary, ropd: PhraseID) -> PhraseID {
    self.alloc(Phrase {
      range: file::Range::combine(&lopd.range(self.as_slice()), &ropd.range(self.as_slice())),
      variant: PhraseVariant::Binary {
        lopd: lopd,
        opr,
        ropd: ropd,
      },
    })
  }
  fn bracket(&mut self, bra: Token, phrase: PhraseID, ket: Token) -> PhraseID {
    let range = file::Range::combine(&bra.range, &ket.range);
    if let TokenVariant::Bra(bracket) = bra.cls {
      self.alloc(Phrase {
        range,
        variant: PhraseVariant::Bracket {
          bracket,
          opd: phrase,
        },
      })
    } else {
      panic!();
    }
  }
}
#[derive(Debug, Clone, Copy)]
pub struct PhraseID {
  index: usize,
}
impl PhraseID {
  pub fn variant(self, arena: &[Phrase]) -> &PhraseVariant {
    &arena[self.index].variant
  }
  pub fn range(self, arena: &[Phrase]) -> &Range {
    &arena[self.index].range
  }
  pub fn identify(self, arena: &[Phrase]) -> SymbolID {
    if let PhraseVariant::Atom(atom) = self.variant(arena) {
      if let Atom::Identifier(identifier) = atom {
        *identifier
      } else {
        panic!()
      }
    } else {
      panic!()
    }
  }

  pub fn unpair(
    &self,
    phrases: &[Phrase],
    expected_separator: Join,
    _file: &str,
    _line: u32,
  ) -> (PhraseID, PhraseID) {
    if let PhraseVariant::Join { separator, phrases } = self.variant(phrases) {
      // p!(phrases);
      // p!(phrases.len());
      assert_eq!(expected_separator, *separator);
      assert!(phrases.len() == 2);
      (phrases[0], phrases[1])
    } else {
      td!();
      // p!(phrase.phrase);
      // println!("origin: {}{}:{}{}{}", GREEN, file, YELLOW, line, RESET);
      // td!()
    }
  }
  pub fn unbracket_map<T>(
    self,
    arena: &[Phrase],
    bracket_to_match: Bracket,
    f: &mut dyn FnMut(PhraseID, &[Phrase]) -> T,
  ) -> Vec<T> {
    match self.variant(arena) {
      PhraseVariant::Bracket { bracket, opd } => {
        assert!(*bracket == bracket_to_match);
        match opd.variant(arena) {
          PhraseVariant::Join { separator, phrases } => {
            if *separator == Join::Comma {
              let mut result: Vec<T> = vec![];
              for phrase in phrases {
                result.push(f(*phrase, arena));
              }
              result
              // PhraseChampIter::new(champ.arena, phrases).map(f).collect()
            } else {
              vec![f(*opd, arena)]
            }
          }
          PhraseVariant::Atom(atom) => {
            if matches!(atom, Atom::Void) {
              vec![]
            } else {
              vec![f(*opd, arena)]
            }
          }
          _ => vec![f(*opd, arena)],
        }
      }
      _ => td!(),
    }
  }
}

impl Debug for Phrase {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Phrase")
      .field("variant", &self.variant)
      // .field("code", &self.code())
      .finish()
  }
}

pub fn parse_phrase(
  allocator: &mut PhraseArena,
  stream: &mut output::TokenStream,
) -> Result<PhraseID, ParserError> {
  if stream.is_end() {
    syntax_error!(stream.end, "unexpected end".to_string(), IndentError)
    //   throw_syntax_error(
    //       stream.end, SyntaxErrorClass::UnexpectedEnd, "unexpected end");
  } else {
    let mut oprs: Vec<Token> = vec![Token::implicit_token_before(
      stream.top(),
      TokenVariant::PhraseStart,
    )];
    oprs.reserve(stream.len());
    let mut opds: Vec<PhraseID> = vec![];
    opds.reserve(stream.len());
    while !stream.is_end() {
      let next = stream.pop();
      // let prev_opr = oprs.last().unwrap();
      match &next.cls {
        TokenVariant::Atom(_) => opds.push(allocator.atom(next)),
        TokenVariant::Binary(_) => {
          while next.precedence <= oprs.last().unwrap().precedence {
            pop(allocator, &mut oprs, &mut opds);
          }
          oprs.push(next);
        }
        TokenVariant::Join(_) => {
          while next.precedence < oprs.last().unwrap().precedence {
            pop(allocator, &mut oprs, &mut opds);
          }
          oprs.push(next);
        }
        TokenVariant::Bra(_) => oprs.push(next),
        TokenVariant::Ket(ket) => {
          while next.precedence < oprs.last().unwrap().precedence {
            pop(allocator, &mut oprs, &mut opds);
          }
          if let TokenVariant::Bra(last_ket) = oprs.last().unwrap().cls {
            assert!(last_ket == *ket);
            //   throw_syntax_error(next.input,
            //                      SyntaxErrorClass::BracketNotMatching,
            //                      "bracket not matching");
          } else {
            assert!(false);
          }
          assert!(opds.len() > 0);
          let bracketed_phrase = opds.pop().unwrap();
          opds.push(allocator.bracket(oprs.pop().unwrap(), bracketed_phrase, next));
        }
        TokenVariant::Prefix(_) => oprs.push(next),
        TokenVariant::Suffix(suffix) => {
          while next.precedence < oprs.last().unwrap().precedence {
            pop(allocator, &mut oprs, &mut opds);
          }
          assert!(opds.len() > 0);
          let suffix_phrase = allocator.suffix(opds.pop().unwrap(), *suffix, next.range.end);
          opds.push(suffix_phrase);
        }
        _ => todo!(),
      }
    }
    while oprs.len() > 1 {
      pop(allocator, &mut oprs, &mut opds);
    }
    assert!(opds.len() == 1);
    Ok(opds.pop().unwrap())
  }
}

fn pop(allocator: &mut PhraseArena, oprs: &mut Vec<Token>, opds: &mut Vec<PhraseID>) {
  let opr = oprs.pop().unwrap();
  match &opr.cls {
    TokenVariant::Join(separator) => {
      let mut number_of_oprs = 1;
      loop {
        if let TokenVariant::Join(last_separator) = oprs.last().unwrap().cls {
          if last_separator == *separator {
            oprs.pop();
            number_of_oprs += 1;
          } else {
            break;
          }
        } else {
          break;
        }
      }
      let number_of_opds = number_of_oprs + 1;
      let phrases: Vec<PhraseID> = opds.split_off(opds.len() - number_of_opds);
      opds.push(allocator.join(*separator, phrases));
    }
    TokenVariant::Prefix(prefix) => {
      let opd = opds.pop().unwrap();
      opds.push(allocator.prefix(opr.range.start, *prefix, opd));
    }
    TokenVariant::Binary(binary) => {
      let ropd = opds.pop().unwrap();
      let lopd = opds.pop().unwrap();
      opds.push(allocator.binary(lopd, *binary, ropd));
    }
    _ => todo!(),
    // print(oprs.last().fml);
    // throw_syntax_error(oprs.last().input,
    //                    SyntaxErrorClass::UnexpectedEnd,
    //                    "TODO: oprs.last()");
    // err("TODO");
  }
}
