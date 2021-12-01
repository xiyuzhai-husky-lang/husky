use common::*;

pub struct LineGroupCharStream<'lex> {
  i_end: usize,
  i: usize,
  j: usize,
  source: &'lex HuskySource,
  _top: Option<char>,
  iter: std::str::Chars<'lex>,
}
impl<'lex> LineGroupCharStream<'lex> {
  pub fn new(source: &'lex HuskySource, i_start: usize, i_end: usize) -> LineGroupCharStream {
    let mut iter = source.lines[i_start].chars();
    let _top = iter.next();
    assert!(i_end <= source.lines.len());
    LineGroupCharStream {
      source,
      i_end,
      i: i_start,
      j: 0,
      _top,
      iter,
    }
  }
  pub fn top<'b>(&'b self) -> char {
    match self._top {
      Some(c) => c,
      None => ' ',
    }
  }
  pub fn is_end(&self) -> bool {
    assert!(self.i < self.i_end);
    self.i == self.i_end - 1 && self.j == self.source.lines[self.i_end - 1].len()
  }
  pub fn get_i(&self) -> usize {
    self.i
  }
  pub fn get_j(&self) -> usize {
    self.j
  }
  // pub fn line(&self, i: usize) -> &'lex str {
  //   todo!();
  // }
  pub fn token_value_from<'b>(&'b self, j_start: usize) -> &'lex str {
    &self.source.lines[self.i][j_start..self.j]
  }
  pub fn pass(&mut self) -> () {
    match self._top {
      Some(_) => self.j += 1,
      None => {
        self.i += 1;
        self.j = 0;
        // assert!(self.i < self.i_end);
        self.iter = self.source.lines[self.i].chars();
      }
    }
    self._top = self.iter.next();
  }
  pub fn pass_to_eol(&mut self) -> () {
    self.j = self.source.lines[self.i].len();
    self._top = None;
  }
}
