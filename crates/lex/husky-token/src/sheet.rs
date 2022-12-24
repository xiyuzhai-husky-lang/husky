use crate::*;

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct TokenIdx(pub(crate) usize);

impl std::ops::Index<TokenIdx> for TokenSheet {
    type Output = Token;

    fn index(&self, index: TokenIdx) -> &Self::Output {
        &self.tokens[index.0]
    }
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct TokenIdxRange {
    start: TokenIdx,
    end: TokenIdx,
}

impl TokenIdxRange {
    pub(crate) fn new(start: TokenIdx, end: TokenIdx) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> TokenIdx {
        self.start
    }

    pub fn end(&self) -> TokenIdx {
        self.end
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenSheet {
    tokens: Vec<Token>,
    group_starts: Vec<usize>,
}

impl TokenSheet {
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn token_sheet(db: &dyn TokenDb, module_path: ModulePath) -> VfsResult<TokenSheet> {
    Ok(TokenSheet::new(tokenize::tokenize(
        db,
        db.module_content(module_path)?,
    )))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenGroupIdx(usize);

pub struct TokenGroupIter<'a> {
    tokens: &'a [Token],
    line_group_starts: &'a [usize],
    current: usize,
}

impl<'a> TokenGroupIter<'a> {
    pub(crate) fn new(tokens: &'a [Token], line_group_starts: &'a [usize]) -> Self {
        Self {
            tokens,
            line_group_starts,
            current: 0,
        }
    }

    fn peek(&self) -> Option<(TokenGroupIdx, TokenGroup<'a>)> {
        if self.current >= self.line_group_starts.len() {
            return None;
        }
        let idx = self.current;
        let start = self.line_group_starts[idx];
        let end = self
            .line_group_starts
            .get(self.current + 1)
            .map(|end| *end)
            .unwrap_or(self.tokens.len());
        Some((
            TokenGroupIdx(idx),
            TokenGroup {
                tokens: &self.tokens[start..end],
            },
        ))
    }

    pub fn peek_with_exact_indent(&self, indent: u32) -> Option<(TokenGroupIdx, TokenGroup<'a>)> {
        let (idx, token_group) = self.peek()?;
        if token_group.indent() != indent {
            return None;
        }
        Some((idx, token_group))
    }

    pub fn next_with_equal_or_more_indent(
        &mut self,
        indent: u32,
    ) -> Option<(TokenGroupIdx, TokenGroup<'a>)> {
        let (idx, group) = self.peek()?;
        if group.indent() >= indent {
            self.current += 1;
            Some((idx, group))
        } else {
            None
        }
    }
}

impl<'a> Iterator for TokenGroupIter<'a> {
    type Item = (TokenGroupIdx, TokenGroup<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.peek()?;
        self.current += 1;
        Some(item)
    }
}

pub struct TokenGroup<'a> {
    tokens: &'a [Token],
}

impl<'a> TokenGroup<'a> {
    pub fn first(&self) -> &'a Token {
        self.tokens.first().unwrap()
    }

    pub fn last(&self) -> &'a Token {
        self.tokens.last().unwrap()
    }

    pub fn indent(&self) -> u32 {
        self.first().range.start.col.0
    }
}

pub(crate) fn produce_group_starts(tokens: &[Token]) -> Vec<usize> {
    let line_starts = produce_line_starts(tokens);
    let mut i = 0;
    let mut line_group_starts = vec![];
    while i < line_starts.len() {
        let line_start0 = line_starts[i];
        let line_indent0 = tokens[line_start0].range.start.col.0;
        line_group_starts.push(line_start0);
        i = {
            let mut j = i + 1;
            while j < line_starts.len() {
                let line_start1 = line_starts[j];
                let line_start_token = &tokens[line_start1];
                let line_indent1 = line_start_token.range.start.col.0;
                enum ControlFlow {
                    Break,
                    Continue,
                }
                use ControlFlow::*;
                let flag = if line_indent1 > line_indent0 {
                    // detect an indentation
                    match tokens[line_start1 - 1].kind {
                        TokenKind::Keyword(Keyword::End(_))
                        | TokenKind::Special(SpecialToken::Colon) => Break,
                        _ => match line_start_token.kind {
                            TokenKind::Attr(_) => Break,
                            TokenKind::Keyword(kw) => match kw {
                                Keyword::Liason(_) | Keyword::End(_) => Continue,
                                _ => Break,
                            },
                            _ => Continue,
                        },
                    }
                } else {
                    if line_indent1 == line_indent0 {
                        match line_start_token.kind {
                            TokenKind::Special(SpecialToken::Ket(_)) => Continue,
                            _ => Break,
                        }
                    } else {
                        Break
                    }
                };
                match flag {
                    Break => break,
                    Continue => j += 1,
                }
            }
            j
        }
    }
    line_group_starts
}

fn produce_line_starts(tokens: &[Token]) -> Vec<usize> {
    (0..tokens.len())
        .filter_map(|line_start| {
            if line_start == 0 {
                Some(0)
            } else if tokens[line_start - 1].range.end.line < tokens[line_start].range.start.line {
                Some(line_start)
            } else {
                None
            }
        })
        .collect()
}

impl TokenSheet {
    pub(crate) fn new(tokens: Vec<Token>) -> TokenSheet {
        TokenSheet {
            group_starts: produce_group_starts(&tokens),
            tokens,
        }
    }

    pub fn token_group_iter<'a>(&'a self) -> TokenGroupIter<'a> {
        TokenGroupIter::new(&self.tokens, &self.group_starts)
    }

    pub fn group_start(&self, token_group_idx: TokenGroupIdx) -> usize {
        self.group_starts[token_group_idx.0]
    }
}

impl std::ops::Index<TokenGroupIdx> for TokenSheet {
    type Output = [Token];

    fn index(&self, index: TokenGroupIdx) -> &Self::Output {
        let start = self.group_starts[index.0];
        let end = self
            .group_starts
            .get(index.0 + 1)
            .map(|end| *end)
            .unwrap_or(self.tokens.len());
        &self.tokens[start..end]
    }
}
