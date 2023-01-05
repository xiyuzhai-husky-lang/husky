use husky_text::TextPosition;

use crate::*;

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct TokenIdx(pub(crate) usize);

impl TokenIdx {
    pub fn raw(self) -> usize {
        self.0
    }
}

impl std::ops::Index<TokenIdx> for TokenSheetData {
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
pub struct RangedTokenSheet {
    token_sheet: TokenSheet,
    // external
    token_ranges: Vec<TextRange>,
    comments: Vec<Comment>,
}

#[salsa::tracked(jar = TokenJar)]
pub struct TokenSheet {
    #[return_ref]
    pub data: TokenSheetData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenSheetData {
    tokens: Vec<Token>,
    group_starts: Vec<usize>,
    indents: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Comment {
    kind: CommentKind,
    next_token_idx: TokenIdx,
    range: TextRange,
}

impl Comment {
    pub(crate) fn new(kind: CommentKind, next_token_idx: TokenIdx, range: TextRange) -> Self {
        Self {
            kind,
            next_token_idx,
            range,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommentKind {
    Todo,
}

impl RangedTokenSheet {
    pub fn len(&self) -> usize {
        self.token_ranges.len()
    }

    pub fn tokens<'a>(&self, db: &'a dyn TokenDb) -> &'a [Token] {
        &self.token_sheet.data(db).tokens
    }

    pub fn ranged_token_iter<'a>(
        &'a self,
        db: &'a dyn TokenDb,
    ) -> impl Iterator<Item = (&'a TextRange, &'a Token)> + 'a {
        let tokens = self.tokens(db);
        (0..tokens.len())
            .into_iter()
            .map(|i| (&self.token_ranges[i], &tokens[i]))
    }

    pub fn token_index_iter<'a>(&'a self) -> impl Iterator<Item = TokenIdx> + 'a {
        (0..self.token_ranges.len())
            .into_iter()
            .map(|i| TokenIdx(i))
    }

    // todo: test this
    pub fn search_token_by_position(&self, pos: TextPosition) -> Option<TokenIdx> {
        let index = match self
            .token_ranges
            .binary_search_by(|range1| range1.start.cmp(&pos))
        {
            Ok(i) => i,
            Err(e) => (e > 0).then(|| e - 1)?,
        };
        assert!(self.token_ranges[index].start <= pos);
        assert!(self.token_ranges[index + 1].start > pos);
        (self.token_ranges[index].end > pos).then(|| TokenIdx(index))
    }

    pub fn tokens_text_range(&self, token_idx_range: TokenIdxRange) -> TextRange {
        let start = token_idx_range.start.0;
        let end = token_idx_range.end.0;
        self.token_ranges[start..end].text_range()
    }

    // todo: change this to pub(crate)
    pub fn token_sheet(&self) -> TokenSheet {
        self.token_sheet
    }

    pub fn token_ranges(&self) -> &[TextRange] {
        self.token_ranges.as_ref()
    }
}

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn token_sheet(
    db: &dyn TokenDb,
    module_path: ModulePath,
) -> VfsResult<RangedTokenSheet> {
    Ok(tokenize::tokenize(db, db.module_content(module_path)?))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenGroupIdx(usize);

pub struct TokenGroupIter<'a> {
    tokens: &'a [Token],
    line_group_starts: &'a [usize],
    indents: &'a [u32],
    current: usize,
}

impl<'a> TokenGroupIter<'a> {
    pub(crate) fn new(
        tokens: &'a [Token],
        line_group_starts: &'a [usize],
        indents: &'a [u32],
    ) -> Self {
        Self {
            tokens,
            line_group_starts,
            indents,
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
                base: start,
                tokens: &self.tokens[start..end],
                indent: self.indents[idx],
            },
        ))
    }

    pub fn peek_token_group_of_exact_indent_with_its_first_token(
        &self,
        indent: u32,
    ) -> Option<(TokenGroupIdx, TokenGroup<'a>, &'a Token)> {
        let (idx, token_group) = self.peek()?;
        if token_group.indent() != indent {
            return None;
        }
        let first_noncomment = token_group.first();
        Some((idx, token_group, first_noncomment))
    }

    pub fn next_token_group_of_equal_or_more_indent_with_its_first_token(
        &mut self,
        indent: u32,
    ) -> Option<(TokenGroupIdx, TokenGroup<'a>, &'a Token)> {
        let (idx, token_group) = self.peek()?;
        if token_group.indent() >= indent {
            self.current += 1;
            let first_noncomment = token_group.first();
            Some((idx, token_group, first_noncomment))
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
    base: usize,
    tokens: &'a [Token],
    indent: u32,
}

impl<'a> TokenGroup<'a> {
    pub fn first(&self) -> &'a Token {
        self.tokens.first().unwrap()
    }

    pub fn last(&self) -> &'a Token {
        self.tokens.last().unwrap()
    }

    pub fn indent(&self) -> u32 {
        self.indent
    }
}

pub(crate) fn produce_group_starts(tokens: &[Token], token_ranges: &[TextRange]) -> Vec<usize> {
    let line_starts = produce_line_starts(token_ranges);
    let mut i = 0;
    let mut line_group_starts = vec![];
    while i < line_starts.len() {
        let line_start0 = line_starts[i];
        let line_indent0 = token_ranges[line_start0].start.col.0;
        line_group_starts.push(line_start0);
        i = {
            let mut j = i + 1;
            while j < line_starts.len() {
                let line_start1 = line_starts[j];
                let line_start_token = &tokens[line_start1];
                let line_indent1 = token_ranges[line_start1].start.col.0;
                enum ControlFlow {
                    Break,
                    Continue,
                }
                use ControlFlow::*;
                let flag = if line_indent1 > line_indent0 {
                    // detect an indentation
                    match tokens[line_start1 - 1] {
                        Token::Keyword(Keyword::End(_))
                        | Token::Punctuation(Punctuation::Colon) => Break,
                        _ => match line_start_token {
                            Token::Attr(_) => Break,
                            Token::Keyword(kw) => match kw {
                                Keyword::Liason(_) | Keyword::End(_) => Continue,
                                _ => Break,
                            },
                            _ => Continue,
                        },
                    }
                } else {
                    if line_indent1 == line_indent0 {
                        match line_start_token {
                            Token::Punctuation(Punctuation::Ket(_)) => Continue,
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

fn produce_line_starts(token_ranges: &[TextRange]) -> Vec<usize> {
    (0..token_ranges.len())
        .filter_map(|line_start| {
            if line_start == 0 {
                Some(0)
            } else if token_ranges[line_start - 1].end.line < token_ranges[line_start].start.line {
                Some(line_start)
            } else {
                None
            }
        })
        .collect()
}

fn produce_indents(group_starts: &[usize], token_ranges: &[TextRange]) -> Vec<u32> {
    group_starts
        .iter()
        .map(|i| token_ranges[*i].start.j())
        .collect()
}

impl RangedTokenSheet {
    pub fn new(
        db: &dyn TokenDb,
        tokens: Vec<Token>,
        token_ranges: Vec<TextRange>,
        comments: Vec<Comment>,
    ) -> RangedTokenSheet {
        let group_starts = produce_group_starts(&tokens, &token_ranges);
        let indents = produce_indents(&group_starts, &token_ranges);
        RangedTokenSheet {
            token_sheet: TokenSheet::new(
                db,
                TokenSheetData {
                    group_starts,
                    tokens,
                    indents,
                },
            ),
            token_ranges,
            comments,
        }
    }

    pub fn token_range(&self, token_idx: TokenIdx) -> TextRange {
        self.token_ranges[token_idx.0]
    }
}

impl TokenSheetData {
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn token_group_iter<'a>(&'a self) -> TokenGroupIter<'a> {
        TokenGroupIter::new(&self.tokens, &self.group_starts, &self.indents)
    }

    pub fn group_start(&self, token_group_idx: TokenGroupIdx) -> usize {
        self.group_starts[token_group_idx.0]
    }

    pub fn token_group_token_idx_range(&self, token_group_idx: TokenGroupIdx) -> TokenIdxRange {
        let start = self.group_starts[token_group_idx.0];
        let end = self
            .group_starts
            .get(token_group_idx.0 + 1)
            .map(|end| *end)
            .unwrap_or(self.tokens.len());
        TokenIdxRange {
            start: TokenIdx(start),
            end: TokenIdx(end),
        }
    }

    pub fn tokens(&self) -> &[Token] {
        self.tokens.as_ref()
    }
}

impl std::ops::Index<TokenGroupIdx> for TokenSheetData {
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
