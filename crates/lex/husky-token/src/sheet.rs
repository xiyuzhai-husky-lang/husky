use crate::*;
use husky_text::TextPosition;
use husky_vfs::VfsError;
use salsa::DebugWithDb;

impl std::ops::Index<TokenIdx> for TokenSheetData {
    type Output = TokenData;

    fn index(&self, idx: TokenIdx) -> &Self::Output {
        &self.tokens[idx.index()]
    }
}

pub enum TokenIdxRangeConfig {
    IncludeBoundary,
    ExcludeBoundary,
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
#[salsa::debug_with_db(db = TokenDb)]
pub struct TokenSheetData {
    tokens: Vec<TokenData>,
    token_group_bases: Vec<TokenGroupStart>,
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

    pub fn tokens<'a>(&self, db: &'a dyn TokenDb) -> &'a [TokenData] {
        &self.token_sheet.data(db).tokens
    }

    pub fn ranged_token_iter<'a>(
        &'a self,
        db: &'a dyn TokenDb,
    ) -> impl Iterator<Item = (&'a TextRange, &'a TokenData)> + 'a {
        let tokens = self.tokens(db);
        (0..tokens.len())
            .into_iter()
            .map(|i| (&self.token_ranges[i], &tokens[i]))
    }

    pub fn indexed_ranged_token_iter<'a>(
        &'a self,
        db: &'a dyn TokenDb,
    ) -> impl Iterator<Item = (TokenIdx, &'a TextRange, &'a TokenData)> + 'a {
        let tokens = self.tokens(db);
        (0..tokens.len())
            .into_iter()
            .map(|i| (TokenIdx::from_index(i), &self.token_ranges[i], &tokens[i]))
    }

    pub fn token_index_iter<'a>(&'a self) -> impl Iterator<Item = TokenIdx> + 'a {
        (0..self.token_ranges.len())
            .into_iter()
            .map(|i| TokenIdx::from_index(i))
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
        (self.token_ranges[index].end > pos).then(|| TokenIdx::from_index(index))
    }

    pub fn tokens_text_range(&self, token_idx_range: TokenIdxRange) -> TextRange {
        let start = token_idx_range.start().index();
        let end = token_idx_range.end().index();
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
    tokens: &'a [TokenData],
    line_group_starts: &'a [TokenGroupStart],
    indents: &'a [u32],
    current: usize,
}

impl<'a> TokenGroupIter<'a> {
    pub(crate) fn new(
        tokens: &'a [TokenData],
        line_group_starts: &'a [TokenGroupStart],
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
            .map(|end| end.index())
            .unwrap_or(self.tokens.len());
        Some((
            TokenGroupIdx(idx),
            TokenGroup {
                base: start,
                tokens: &self.tokens[start.index()..end],
                indent: self.indents[idx],
            },
        ))
    }

    pub fn peek_token_group_of_exact_indent_with_its_first_token(
        &self,
        indent: u32,
    ) -> Option<(TokenGroupIdx, TokenGroup<'a>, TokenData)> {
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
    ) -> Option<(TokenGroupIdx, TokenGroup<'a>, TokenData)> {
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
    base: TokenGroupStart,
    tokens: &'a [TokenData],
    indent: u32,
}

impl<'a> TokenGroup<'a> {
    pub fn first(&self) -> TokenData {
        *self.tokens.first().unwrap()
    }

    pub fn second(&self) -> Option<TokenData> {
        self.tokens.get(1).copied()
    }

    pub fn last(&self) -> TokenData {
        *self.tokens.last().unwrap()
    }

    pub fn indent(&self) -> u32 {
        self.indent
    }
}

// todo: move this to a root module called group
pub(crate) fn produce_token_group_starts(
    tokens: &[TokenData],
    token_ranges: &[TextRange],
) -> Vec<TokenGroupStart> {
    let line_starts = produce_line_starts(token_ranges);
    let mut i = 0;
    let mut line_group_starts = vec![];
    while i < line_starts.len() {
        let line0_start = line_starts[i];
        let line0_indent = token_ranges[line0_start].start.col.0;
        line_group_starts.push(TokenGroupStart::from_index(line0_start));
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
                        TokenData::Keyword(Keyword::End(_))
                        | TokenData::Punctuation(Punctuation::EQ)
                        | TokenData::Punctuation(Punctuation::COLON) => Break,
                        TokenData::Punctuation(
                            Punctuation::LPAR
                            | Punctuation::LBOX
                            | Punctuation::LCURL
                            | Punctuation::LA_OR_LT,
                        ) => Continue,
                        _ => match line_start_token {
                            TokenData::Keyword(
                                Keyword::Pronoun(_)
                                | Keyword::Modifier(_)
                                | Keyword::End(_)
                                | Keyword::Pub,
                            ) => Continue,
                            TokenData::Keyword(kw) => Break,
                            _ => Continue,
                        },
                    }
                } else {
                    if line_indent1 == line0_indent {
                        match line_start_token {
                            TokenData::Punctuation(
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

fn produce_indents(token_group_starts: &[TokenGroupStart], token_ranges: &[TextRange]) -> Vec<u32> {
    token_group_starts
        .iter()
        .map(|i| token_ranges[i.index()].start.j())
        .collect()
}

impl RangedTokenSheet {
    pub fn new(
        db: &dyn TokenDb,
        tokens: Vec<TokenData>,
        token_ranges: Vec<TextRange>,
        comments: Vec<Comment>,
    ) -> RangedTokenSheet {
        let group_starts = produce_token_group_starts(&tokens, &token_ranges);
        let indents = produce_indents(&group_starts, &token_ranges);
        RangedTokenSheet {
            token_sheet: TokenSheet::new(
                db,
                TokenSheetData {
                    token_group_bases: group_starts,
                    tokens,
                    indents,
                },
            ),
            token_ranges,
            comments,
        }
    }

    pub fn token_idx_text_range(&self, token_idx: TokenIdx) -> TextRange {
        debug_assert!(token_idx.index() < self.token_ranges.len());
        self.token_ranges[token_idx.index()]
    }

    pub fn token_idx_range_text_range(&self, token_idx_range: TokenIdxRange) -> TextRange {
        debug_assert!(token_idx_range.end().token_idx() > token_idx_range.start().token_idx());
        let text_range_start = self.token_ranges[token_idx_range.start().index()].start;
        let text_range_end = self.token_ranges[(token_idx_range.end().index() - 1) as usize].end;
        (text_range_start..text_range_end).into()
    }

    pub fn token_stream_state_text_range(&self, token_stream_state: TokenStreamState) -> TextRange {
        match token_stream_state.drained() {
            true => {
                let next_token_idx_index = token_stream_state.next_token_idx().index();
                match next_token_idx_index {
                    0 => todo!(),
                    _ => self.token_ranges[(next_token_idx_index - 1) as usize].right_after(),
                }
            }
            false => self.token_ranges[token_stream_state.next_token_idx().index()],
        }
    }
}

impl TokenSheetData {
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn token_group_iter<'a>(&'a self) -> TokenGroupIter<'a> {
        TokenGroupIter::new(&self.tokens, &self.token_group_bases, &self.indents)
    }

    pub fn token_group_base(&self, token_group_idx: TokenGroupIdx) -> TokenGroupStart {
        self.token_group_bases[token_group_idx.0]
    }

    pub fn token_group_token_idx_range(&self, token_group_idx: TokenGroupIdx) -> TokenIdxRange {
        let start = self.token_group_bases[token_group_idx.0].index();
        let end = self
            .token_group_bases
            .get(token_group_idx.0 + 1)
            .map(|&end| end.index())
            .unwrap_or(self.tokens.len());
        TokenIdxRange::from_indices(start, end)
    }

    pub fn tokens(&self) -> &[TokenData] {
        self.tokens.as_ref()
    }

    pub fn token_group_idx(&self, token_idx: TokenIdx) -> TokenGroupIdx {
        assert!(self.token_group_bases[0].index() == 0);
        TokenGroupIdx(
            match self
                .token_group_bases
                .binary_search_by(|base| base.token_idx().cmp(&token_idx))
            {
                Ok(i) => i,
                Err(i) => {
                    assert!(i > 0);
                    i - 1
                }
            },
        )
    }
}

impl std::ops::Index<TokenGroupIdx> for TokenSheetData {
    type Output = [TokenData];

    fn index(&self, index: TokenGroupIdx) -> &Self::Output {
        let start = self.token_group_bases[index.0].index();
        let end = self
            .token_group_bases
            .get(index.0 + 1)
            .map(|&end| end.index())
            .unwrap_or(self.tokens.len());
        &self.tokens[start..end]
    }
}
