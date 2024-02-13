use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct SnippetTokraRegion {
    #[return_ref]
    tokens: Vec<TokenData>,
}

#[derive(Debug, Clone, Copy)]
pub struct SnippetTokraRegionDataRef<'a> {
    tokens: &'a [TokenData],
}

impl<'a> std::ops::Index<RegionalTokenIdx> for SnippetTokraRegionDataRef<'a> {
    type Output = TokenData;

    fn index(&self, idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens[idx.index()]
    }
}
