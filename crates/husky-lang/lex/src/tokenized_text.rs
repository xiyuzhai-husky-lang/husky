use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedText {
    tokens: Vec<Token>,
    line_groups: Vec<TokenGroup>,
    errors: Vec<LexError>,
}

impl TokenizedText {
    pub fn new(
        mut line_groups: Vec<TokenGroup>,
        tokens: Vec<Token>,
        errors: Vec<LexError>,
    ) -> TokenizedText {
        set_folding_ends(&mut line_groups);
        TokenizedText {
            tokens,
            line_groups,
            errors,
        }
    }

    pub fn folded_iter(&self, start: usize) -> TokenGroupFoldedIter {
        TokenGroupFoldedIter {
            text: &self,
            next_raw: start,
            indent: self.line_groups[start].indent,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenGroup {
    pub(crate) indent: Indent,
    pub(crate) tokens: Range,
    pub(crate) folding_end: usize,
}

impl TokenizedText {
    pub(crate) fn lex(db: &dyn LexQuery, text: &str) -> Self {
        let mut token_scanner = TokenScanner::new(db);
        text.lines()
            .enumerate()
            .for_each(|(i, line)| token_scanner.scan(i, line));
        token_scanner.into()
    }
}

fn set_folding_ends(line_groups: &mut [TokenGroup]) {
    for i in 0..line_groups.len() {
        let mut j = i + 1;
        line_groups[i].folding_end = loop {
            if j < line_groups.len() {
                if line_groups[j].indent <= line_groups[i].indent {
                    break j;
                }
                j += 1;
            } else {
                break line_groups.len();
            }
        }
    }
}

pub struct TokenGroupFoldedIter<'a> {
    text: &'a TokenizedText,
    next_raw: usize,
    indent: Indent,
}

impl<'a> Iterator for TokenGroupFoldedIter<'a> {
    type Item = &'a [Token];

    fn next(&mut self) -> Option<&'a [Token]> {
        if self.next_raw >= self.text.line_groups.len() {
            return None;
        }
        if self.text.line_groups[self.next_raw].indent < self.indent {
            return None;
        }
        let line_group = &self.text.line_groups[self.next_raw];
        self.next_raw = line_group.folding_end;
        Some(&self.text.tokens[line_group.tokens.clone()])
    }
}
