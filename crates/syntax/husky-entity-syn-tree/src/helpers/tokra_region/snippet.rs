use super::*;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct SnippetTokraRegion {
    #[return_ref]
    tokens: Vec<Token>,
}

pub struct SnippetTokraRegionData<'a> {
    tokens: &'a [Token],
}

impl<'a> std::ops::Index<RegionalTokenIdx> for SnippetTokraRegionData<'a> {
    type Output = Token;

    fn index(&self, idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens[idx.index()]
    }
}
