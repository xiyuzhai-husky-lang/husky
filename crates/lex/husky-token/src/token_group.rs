use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenGroupStart(TokenIdx);

impl TokenGroupStart {
    pub fn from_index(index: usize) -> Self {
        Self(TokenIdx::from_index(index))
    }

    pub fn token_idx(self) -> TokenIdx {
        self.0
    }

    pub fn index(self) -> usize {
        self.0.index()
    }
}

/// 0-based
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TokenGroupRelativeTokenIndex(usize);

impl TokenGroupRelativeTokenIndex {
    pub(crate) fn new(base: TokenGroupStart, token_idx: TokenIdx) -> Self {
        debug_assert!(base.token_idx() <= token_idx);
        Self(token_idx.index() - base.index())
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }

    pub(crate) fn token_idx(self, base: TokenGroupStart) -> TokenIdx {
        base.0 + self.0
    }
}

impl std::ops::AddAssign<usize> for TokenGroupRelativeTokenIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}

impl std::ops::SubAssign<usize> for TokenGroupRelativeTokenIndex {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs
    }
}

pub enum TokenGroupProductionState {
    None,
    Match,
    MatchVertical,
}

enum ControlFlow {
    Break,
    Continue,
}
use husky_print_utils::p;
use ControlFlow::*;

// todo: move this to a root module called group
pub(crate) fn produce_token_group_starts(
    tokens_data: &[TokenData],
    token_ranges: &[TextRange],
) -> Vec<TokenGroupStart> {
    let line_starts = produce_line_starts(token_ranges);
    let mut i = 0;
    let mut token_group_starts = vec![];
    let mut state = TokenGroupProductionState::None;
    while i < line_starts.len() {
        let line0_start = line_starts[i];
        let line0_indent = token_ranges[line0_start].start.col.0;
        token_group_starts.push(TokenGroupStart::from_index(line0_start));
        state = match tokens_data[line0_start] {
            TokenData::Keyword(Keyword::Stmt(StmtKeyword::Match)) => {
                TokenGroupProductionState::Match
            }
            TokenData::Punctuation(Punctuation::VERTICAL) => match state {
                TokenGroupProductionState::None => TokenGroupProductionState::None,
                TokenGroupProductionState::Match | TokenGroupProductionState::MatchVertical => {
                    TokenGroupProductionState::MatchVertical
                }
            },
            _ => TokenGroupProductionState::None,
        };
        i = {
            let mut j = i + 1;
            while j < line_starts.len() {
                let line1_start = line_starts[j];
                let line1_start_token_data = &tokens_data[line1_start];
                let line1_indent = token_ranges[line1_start].start.col.0;
                let control_flow = if line1_indent > line0_indent {
                    // detected an indentation
                    match tokens_data[line1_start - 1] {
                        TokenData::Keyword(Keyword::End(_))
                        | TokenData::Punctuation(Punctuation::EQ)
                        | TokenData::Punctuation(Punctuation::HEAVY_ARROW)
                        | TokenData::Punctuation(Punctuation::COLON) => Break,
                        TokenData::Punctuation(
                            Punctuation::LPAR
                            | Punctuation::LBOX
                            | Punctuation::LCURL
                            | Punctuation::LA_OR_LT,
                        ) => Continue,
                        _ => match line1_start_token_data {
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
                } else if line1_indent == line0_indent {
                    match line1_start_token_data {
                        TokenData::Punctuation(
                            Punctuation::RPAR
                            | Punctuation::RBOX
                            | Punctuation::RCURL
                            | Punctuation::RA_OR_GT,
                        ) => Continue,
                        TokenData::Punctuation(Punctuation::VERTICAL) => match state {
                            TokenGroupProductionState::None | TokenGroupProductionState::Match => {
                                Break
                            }
                            TokenGroupProductionState::MatchVertical => {
                                if tokens_data[line_starts[j - 1]..line1_start]
                                    .contains(&TokenData::Punctuation(Punctuation::HEAVY_ARROW))
                                {
                                    Break
                                } else {
                                    Continue
                                }
                            }
                        },
                        _ => Break,
                    }
                } else {
                    Break
                };
                match control_flow {
                    Break => break,
                    Continue => j += 1,
                }
            }
            j
        }
    }
    token_group_starts
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
