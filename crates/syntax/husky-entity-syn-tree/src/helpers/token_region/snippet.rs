use super::*;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct SnippetTokenRegion {
    #[return_ref]
    tokens: Vec<Token>,
}

pub struct SnippetTokenRegionData<'a> {
    tokens: &'a [Token],
}

impl<'a> std::ops::Index<RegionalTokenIdx> for SnippetTokenRegionData<'a> {
    type Output = Token;

    fn index(&self, index: RegionalTokenIdx) -> &Self::Output {
        &self.tokens[index.0.get() as usize - 1]
    }
}
