use crate::*;

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
                            TokenKind::Decorator(_) => Break,
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

#[derive(Debug, PartialEq, Eq)]
pub struct TokenGroupSheet {
    tokens: Vec<Token>,
    group_starts: Vec<usize>,
}

impl TokenGroupSheet {
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn iter<'a>(&'a self) -> TokenGroupIter<'a> {
        TokenGroupIter::new(&self.tokens, &self.group_starts)
    }

    pub fn new(tokens: Vec<Token>) -> TokenGroupSheet {
        TokenGroupSheet {
            group_starts: produce_group_starts(&tokens),
            tokens,
        }
    }
}
