use crate::token::output::TokenStream;
use crate::token::*;
use std::collections::VecDeque;
pub struct TokenReceiver {
  tokens: VecDeque<Token>,
}
fn get_left_convexity(token: &Token) -> Convexity {
  match &token.cls {
    TokenVariant::Keyword(_)
    | TokenVariant::Atom(_)
    | TokenVariant::Bra(_)
    | TokenVariant::Prefix(_) => Convexity::Convex,
    TokenVariant::Join(_)
    | TokenVariant::Ket(_)
    | TokenVariant::Suffix(_)
    | TokenVariant::Binary(_) => Convexity::Concave,
    _ => todo!(),
    //   print(token.fml);
    //   err("TODO");
  }
}
fn get_right_convexity(token: &Token) -> Convexity {
  match &token.cls {
    TokenVariant::Atom(_) | TokenVariant::Ket(_) | TokenVariant::Suffix(_) => Convexity::Convex,
    TokenVariant::Keyword(_)
    | TokenVariant::Bra(_)
    | TokenVariant::Join(_)
    | TokenVariant::Prefix(_)
    | TokenVariant::Binary(_) => Convexity::Concave,
    _ => {
      println!("cls: {:?}", &token.cls);
      todo!()
    } //   print(token.fml);
      //   err("TODO");
  }
}
fn get_queue_right_convexity(tokens: &VecDeque<Token>) -> Convexity {
  match &tokens.back() {
    None => Convexity::Concave,
    Some(token) => get_right_convexity(token),
  }
}
impl TokenReceiver {
  pub fn new() -> TokenReceiver {
    TokenReceiver {
      tokens: VecDeque::<Token>::new(),
    }
  }
  pub fn add_token(
    &mut self,
    range: file::Range,
    cls: TokenVariant,
    is_rear_attached: bool,
  ) -> Result<(), ParserError> {
    let token = Token::new(range, cls, is_rear_attached);
    if self.tokens.len() > 1 {
      self.accept_non_keyword(token)
    } else if self.tokens.len() == 1 {
      self.accept_second(token)
    } else {
      self.accept_first_keyword(token)
    }
  }
  pub fn output(self) -> TokenStream {
    TokenStream::new(self.tokens)
  }
  pub fn empty(&self) -> bool {
    self.tokens.is_empty()
  }
  pub fn len(&self) -> usize {
    self.tokens.len()
  }
  pub fn last(&self) -> &Token {
    self.tokens.back().unwrap()
  }
  pub fn right_convexity(&self) -> Convexity {
    get_queue_right_convexity(&self.tokens)
  }

  fn accept_non_keyword(&mut self, token: Token) -> Result<(), ParserError> {
    match &token.cls {
      TokenVariant::Keyword(_) => {
        syntax_error!(
          token.range,
          "can't put keyword in the middle of a line group!".to_string(),
          KeywordMisplacement
        )
        // println!("token: {:?}", token);
        // todo!();
        //   SyntaxError::KeywordMisplacement,
        //   "can't put keyword in the middle of a line group!")
      }
      _ => {
        if self.tokens.back().unwrap().is_rear_attached {
          self.accept_non_keyword_after_no_space(token)
        } else {
          self.accept_non_keyword_after_space(token)
        }
      }
    }
  }
  fn accept_non_keyword_after_no_space(&mut self, token: Token) -> Result<(), ParserError> {
    match &token.cls {
      TokenVariant::Keyword(_) => {
        panic!();
      }
      _ => {}
    };
    let prev = self.tokens.back().unwrap();
    match &prev.cls {
      TokenVariant::Atom(_) | TokenVariant::Ket(_) => match &token.cls {
        TokenVariant::Atom(atom) => match atom {
          Atom::Identifier(_) => self.push_compatible(Token::implicit_token_before(
            &token,
            TokenVariant::Binary(Binary::Combine),
          )),
          _ => todo!(),
        },
        TokenVariant::Bra(bracket) => match bracket {
          Bracket::Par => self.push_compatible(Token::implicit_token_before(
            &token,
            TokenVariant::Binary(Binary::Call),
          )),
          Bracket::Box => self.push_compatible(Token::implicit_token_before(
            &token,
            TokenVariant::Binary(Binary::Index),
          )),
          _ => todo!(),
        },
        TokenVariant::Prefix(prefix) => match prefix {
          Prefix::Shared | Prefix::NotOrExclusive => self.push_compatible(
            Token::implicit_token_before(&token, TokenVariant::Binary(Binary::Combine)),
          ),
          _ => todo!(),
        },
        TokenVariant::Binary(_)
        | TokenVariant::Join(_)
        | TokenVariant::Ket(_)
        | TokenVariant::Suffix(_) => Ok(()),
        _ => {
          todo!();
        }
      },
      TokenVariant::Bra(_) => {
        if let TokenVariant::Ket(_) = &token.cls {
          self.push_compatible(Token::implicit_token_before(
            &token,
            TokenVariant::Atom(Atom::Void),
          ))
        } else {
          Ok(())
        }
      }
      TokenVariant::Suffix(_) => {
        todo!();
        // print(token);
        // print(token.input.range);
        // err("what");
      }
      _ => Ok(()),
    }?;
    self.push_compatible(token)
  }
  fn accept_non_keyword_after_space(&mut self, token: Token) -> Result<(), ParserError> {
    if let TokenVariant::Keyword(_keyword) = &token.cls {
      syntax_error!(
        token.range,
        "expect non keyword".to_string(),
        KeywordMisplacement
      )
    } else {
      let prev = self.tokens.back().unwrap();
      match &prev.cls {
        TokenVariant::Atom(_) | TokenVariant::Ket(_) => match &token.cls {
          TokenVariant::Atom(_) | TokenVariant::Bra(_) | TokenVariant::Prefix(_) => self
            .push_compatible(Token::implicit_token_before(
              &token,
              TokenVariant::Join(Join::List),
            )),
          TokenVariant::Binary(_) | TokenVariant::Ket(_) => Ok(()),
          _ => todo!(),
        },
        _ => Ok(()),
      }?;
      self.push_compatible(token)
    }
  }

  fn combine_keywords(&mut self, token: Token, cls: TokenVariant) {
    self.tokens.back_mut().unwrap().cls = cls;
    self.tokens.back_mut().unwrap().range = file::Range {
      start: self.tokens.back().unwrap().range.start,
      end: token.range.end,
    };
  }

  fn accept_second(&mut self, token: Token) -> Result<(), ParserError> {
    if let TokenVariant::Keyword(second_keyword) = &token.cls {
      if let TokenVariant::Keyword(first_keyword) = &self.tokens.back().unwrap().cls {
        match first_keyword {
          Keyword::Pub => match second_keyword {
            Keyword::Mod => {
              Ok(self.combine_keywords(token, TokenVariant::Keyword(Keyword::PubMod)))
            }
            Keyword::Struct => {
              Ok(self.combine_keywords(token, TokenVariant::Keyword(Keyword::PubStruct)))
            }
            Keyword::Enum => {
              Ok(self.combine_keywords(token, TokenVariant::Keyword(Keyword::PubEnum)))
            }
            Keyword::Rename => {
              Ok(self.combine_keywords(token, TokenVariant::Keyword(Keyword::PubRename)))
            }
            Keyword::Func => {
              Ok(self.combine_keywords(token, TokenVariant::Keyword(Keyword::PubFunc)))
            }
            Keyword::Def => {
              Ok(self.combine_keywords(token, TokenVariant::Keyword(Keyword::PubDef)))
            }
            Keyword::Pattern => {
              Ok(self.combine_keywords(token, TokenVariant::Keyword(Keyword::PubPattern)))
            }
            _ => todo!(),
          },
          Keyword::For => {
            if let TokenVariant::Keyword(_second_keyword) = &token.cls {
              todo!();
            }
            Ok(())
          }
          Keyword::Do => {
            if let TokenVariant::Keyword(second_keyword) = &token.cls {
              if *second_keyword == Keyword::While {
                self.combine_keywords(token, TokenVariant::Keyword(Keyword::DoWhile));
              } else {
                panic!();
              }
            }
            Ok(())
          }
          _ => self.push_compatible(token),
        }
      } else {
        self.push_compatible(token)
      }
    } else {
      self.push_compatible(token)
    }
  }
  fn accept_first_keyword(&mut self, token: Token) -> Result<(), ParserError> {
    match &token.cls {
      TokenVariant::Keyword(keyword) => match &keyword {
        Keyword::Else | Keyword::Default | Keyword::Break | Keyword::Main | Keyword::Comment => {
          assert!(self.tokens.len() == 0);
          let void_token = Token::implicit_token_after(&token, TokenVariant::Atom(Atom::Void));
          self.push_compatible(token)?;
          self.push_compatible(void_token)
        }
        _ => self.push_compatible(token),
      },
      _ => {
        self.push_compatible(Token::implicit_token_before(
          &token,
          TokenVariant::Keyword(Keyword::NoKeyword),
        ))?;
        self.push_compatible(token)
      }
    }
  }

  fn push_compatible(&mut self, token: Token) -> Result<(), ParserError> {
    fn is_compatible(a: Convexity, b: Convexity) -> bool {
      a != Convexity::None && b != Convexity::None && a != b
    }

    if !is_compatible(self.right_convexity(), get_left_convexity(&token)) {
      syntax_error!(
        token.range,
        "incompatible convexity!".to_string(),
        IncompatibleConvexity
      )
    } else {
      self.tokens.push_back(token);
      Ok(())
    }
  }
}
