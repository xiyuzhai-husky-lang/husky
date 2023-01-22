use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EnglishTokenSheet {
    tokens: Vec<EnglishToken>,
    paragraph_starts: Vec<usize>,
}

impl EnglishTokenSheet {
    pub fn new(tokens: Vec<EnglishToken>) -> Self {
        Self {
            paragraph_starts: produce_paragraph_starts(&tokens),
            tokens,
        }
    }

    pub fn paragraphs(&self) -> impl Iterator<Item = &[EnglishToken]> {
        (0..self.paragraph_starts.len()).map(|paragraph_index| self.paragraph(paragraph_index))
    }

    fn paragraph(&self, paragraph_index: usize) -> &[EnglishToken] {
        let start = self.paragraph_starts[paragraph_index];
        let end = if paragraph_index + 1 < self.paragraph_starts.len() {
            self.paragraph_starts[paragraph_index + 1]
        } else {
            self.tokens.len()
        };
        &self.tokens[start..end]
    }
}
