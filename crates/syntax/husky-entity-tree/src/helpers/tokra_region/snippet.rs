use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct SnippetTokraRegion {
    #[return_ref]
    tokens: Vec<TokenData>,
}

pub struct SnippetTokraRegionData<'a> {
    tokens: &'a [TokenData],
}

impl<'a> std::ops::Index<RegionalTokenIdx> for SnippetTokraRegionData<'a> {
    type Output = TokenData;

    fn index(&self, idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens[idx.index()]
    }
}
