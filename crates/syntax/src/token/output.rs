use super::*;
use common::file;
use std::collections::VecDeque;
#[derive(Debug)]
pub struct TokenStream {
  pub end: file::Range,
  tokens: VecDeque<Token>,
}
impl TokenStream {
  pub fn new(tokens: VecDeque<Token>) -> TokenStream {
    fn get_range_after(range: &file::Range) -> file::Range {
      file::Range {
        start: range.end,
        end: file::Position {
          i: range.end.i,
          j: range.end.j + 1,
        },
      }
    }
    TokenStream {
      end: get_range_after(&tokens.back().unwrap().range),
      tokens,
    }
  }
  pub fn top<'b>(&'b self) -> &Token {
    self.tokens.front().unwrap()
  }

  pub fn pop<'b>(&'b mut self) -> Token {
    self.tokens.pop_front().unwrap()
  }

  pub fn is_end(&self) -> bool {
    self.tokens.is_empty()
  }
  pub fn len(&self) -> usize {
    self.tokens.len()
  }
}
