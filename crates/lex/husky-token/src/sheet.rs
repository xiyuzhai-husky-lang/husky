use crate::*;
use husky_text::TextPosition;
use husky_vfs::VfsError;
use salsa::DebugWithDb;

/// is eof if raw is equal to the len of all tokens
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct TokenIdx(pub(crate) usize);

impl TokenIdx {
    pub fn raw(self) -> usize {
        self.0
    }
}

impl std::ops::Add<usize> for TokenIdx {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::Sub<usize> for TokenIdx {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl std::ops::Index<TokenIdx> for TokenSheetData {
    type Output = Token;

    fn index(&self, index: TokenIdx) -> &Self::Output {
        &self.tokens[index.0]
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct TokenIdxRange {
    start: TokenIdxRangeStart,
    end: TokenIdxRangeEnd,
}

pub enum TokenIdxRangeConfig {
    IncludeBoundary,
    ExcludeBoundary,
}

impl TokenIdxRange {
    pub fn new_single(token_idx: TokenIdx) -> Self {
        Self {
            start: TokenIdxRangeStart(token_idx),
            end: TokenIdxRangeEnd(token_idx + 1),
        }
    }

    #[inline(always)]
    pub fn to(self, end: TokenIdxRangeEnd) -> Self {
        Self {
            start: self.start,
            end,
        }
    }

    #[inline(always)]
    pub fn join(self, other: TokenIdxRange) -> Self {
        self.to(other.end())
    }
}

impl std::fmt::Debug for TokenIdxRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.start.0 .0..self.end.0 .0).fmt(f)
    }
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct TokenIdxRangeStart(TokenIdx);

impl TokenIdxRangeStart {
    pub fn token_idx(self) -> TokenIdx {
        self.0
    }
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct TokenIdxRangeEnd(TokenIdx);

impl TokenIdxRangeEnd {
    pub fn new_after(token_idx: TokenIdx) -> Self {
        Self(token_idx + 1)
    }

    pub fn token_idx(self) -> TokenIdx {
        self.0
    }
}

pub trait HasTokenIdxRange {
    fn token_idx_range(&self) -> TokenIdxRange;
}

impl HasTokenIdxRange for TokenIdxRange {
    fn token_idx_range(&self) -> TokenIdxRange {
        *self
    }
}

impl<T> HasTokenIdxRange for [T]
where
    T: HasTokenIdxRange,
{
    fn token_idx_range(&self) -> TokenIdxRange {
        if self.len() == 0 {
            return TokenIdxRange {
                start: TokenIdxRangeStart(TokenIdx(0)),
                end: TokenIdxRangeEnd(TokenIdx(0)),
            };
        }
        TokenIdxRange {
            start: self.first().unwrap().token_idx_range().start,
            end: self.last().unwrap().token_idx_range().end,
        }
    }
}

impl From<(TokenIdxRangeStart, TokenIdxRangeEnd)> for TokenIdxRange {
    fn from((start, end): (TokenIdxRangeStart, TokenIdxRangeEnd)) -> Self {
        Self { start, end }
    }
}

impl TokenIdxRange {
    #[inline(always)]
    pub fn new(start: TokenIdx, end: TokenIdxRangeEnd) -> Self {
        Self {
            start: TokenIdxRangeStart(start),
            end,
        }
    }

    fn new_from_raw(start: usize, end: usize) -> Self {
        Self {
            start: TokenIdxRangeStart(TokenIdx(start)),
            end: TokenIdxRangeEnd(TokenIdx(end)),
        }
    }

    pub fn start(&self) -> TokenIdxRangeStart {
        self.start
    }

    pub fn end(&self) -> TokenIdxRangeEnd {
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

#[salsa::tracked(db = TokenDb, jar = TokenJar)]
pub struct TokenSheet {
    #[return_ref]
    pub(crate) data: TokenSheetData,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct TokenSheetData {
    tokens: Vec<Token>,
    group_starts: Vec<usize>,
    indents: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Comment {
    kind: CommentKind,
    number_of_preceding_tokens: usize,
    range: TextRange,
}

impl Comment {
    pub(crate) fn new(
        kind: CommentKind,
        number_of_preceding_tokens: usize,
        range: TextRange,
    ) -> Self {
        Self {
            kind,
            number_of_preceding_tokens,
            range,
        }
    }

    pub fn number_of_preceding_tokens(&self) -> usize {
        self.number_of_preceding_tokens
    }

    pub fn range(&self) -> TextRange {
        self.range
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

    pub fn indexed_ranged_token_iter<'a>(
        &'a self,
        db: &'a dyn TokenDb,
    ) -> impl Iterator<Item = (TokenIdx, &'a TextRange, &'a Token)> + 'a {
        let tokens = self.tokens(db);
        (0..tokens.len())
            .into_iter()
            .map(|i| (TokenIdx(i), &self.token_ranges[i], &tokens[i]))
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
        (self.token_ranges[index].end > pos).then(|| TokenIdx(index))
    }

    pub fn tokens_text_range(&self, token_idx_range: TokenIdxRange) -> TextRange {
        let start = token_idx_range.start.0 .0;
        let end = token_idx_range.end.0 .0;
        self.token_ranges[start..end].text_range()
    }

    pub fn token_sheet_data<'a>(&self, db: &'a dyn TokenDb) -> &'a TokenSheetData {
        self.token_sheet.data(db)
    }

    pub fn token_text_ranges(&self) -> &[TextRange] {
        self.token_ranges.as_ref()
    }

    pub(crate) fn token_sheet(&self) -> TokenSheet {
        self.token_sheet
    }

    pub fn comments(&self) -> &[Comment] {
        self.comments.as_ref()
    }
}

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn ranged_token_sheet(
    db: &dyn TokenDb,
    module_path: ModulePath,
) -> VfsResult<RangedTokenSheet> {
    Ok(tokenize::tokenize(db, module_path.text(db)?))
}

#[salsa::tracked(jar = TokenJar)]
pub(crate) fn token_sheet(db: &dyn TokenDb, module_path: ModulePath) -> VfsResult<TokenSheet> {
    Ok(ranged_token_sheet(db, module_path).as_ref()?.token_sheet)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenGroupIdx(usize);

impl std::fmt::Display for TokenGroupIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

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

    pub fn state(&self) -> TokenGroupIdx {
        TokenGroupIdx(self.current)
    }

    pub fn rollback(&mut self, state: TokenGroupIdx) {
        self.current = state.0
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
            .copied()
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
    ) -> Option<(TokenGroupIdx, TokenGroup<'a>, Token)> {
        let (idx, token_group) = self.peek()?;
        if token_group.indent() != indent {
            return None;
        }
        let first_noncomment = token_group.first();
        Some((idx, token_group, first_noncomment))
    }

    pub fn next_token_group_of_no_less_indent_with_its_first_token(
        &mut self,
        indent: u32,
    ) -> Option<(TokenGroupIdx, TokenGroup<'a>, Token)> {
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
    pub fn first(&self) -> Token {
        *self.tokens.first().unwrap()
    }

    pub fn second(&self) -> Option<Token> {
        self.tokens.get(1).copied()
    }

    pub fn last(&self) -> Token {
        *self.tokens.last().unwrap()
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
        let line0_start = line_starts[i];
        let line0_indent = token_ranges[line0_start].start.col.0;
        line_group_starts.push(line0_start);
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
                let flag = if line_indent1 > line0_indent {
                    // detect an indentation
                    match tokens[line_start1 - 1] {
                        Token::Keyword(Keyword::End(_))
                        | Token::Punctuation(Punctuation::COLON) => Break,
                        Token::Punctuation(
                            Punctuation::LPAR
                            | Punctuation::LBOX
                            | Punctuation::LCURL
                            | Punctuation::LA_OR_LT,
                        ) => Continue,
                        _ => match line_start_token {
                            Token::Keyword(
                                Keyword::Pronoun(_)
                                | Keyword::Modifier(_)
                                | Keyword::End(_)
                                | Keyword::Pub,
                            ) => Continue,
                            Token::Keyword(kw) => Break,
                            _ => Continue,
                        },
                    }
                } else {
                    if line_indent1 == line0_indent {
                        match line_start_token {
                            Token::Punctuation(
                                Punctuation::RPAR
                                | Punctuation::RBOX
                                | Punctuation::RCURL
                                | Punctuation::RA_OR_GT,
                            ) => Continue,
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

    pub fn token_text_range(&self, token_idx: TokenIdx) -> TextRange {
        if token_idx.0 < self.token_ranges.len() {
            self.token_ranges[token_idx.0]
        } else {
            // eof
            self.token_ranges.last().copied().unwrap_or_default()
        }
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
            .copied()
            .unwrap_or(self.tokens.len());
        TokenIdxRange::new_from_raw(start, end)
    }

    pub fn tokens(&self) -> &[Token] {
        self.tokens.as_ref()
    }

    pub fn token_group_idx(&self, token_idx: TokenIdx) -> TokenGroupIdx {
        assert!(self.group_starts[0] == 0);
        TokenGroupIdx(match self.group_starts.binary_search(&token_idx.raw()) {
            Ok(i) => i,
            Err(i) => {
                assert!(i > 0);
                i - 1
            }
        })
    }
}

impl std::ops::Index<TokenGroupIdx> for TokenSheetData {
    type Output = [Token];

    fn index(&self, index: TokenGroupIdx) -> &Self::Output {
        let start = self.group_starts[index.0];
        let end = self
            .group_starts
            .get(index.0 + 1)
            .copied()
            .unwrap_or(self.tokens.len());
        &self.tokens[start..end]
    }
}
