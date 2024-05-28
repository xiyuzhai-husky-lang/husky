use super::*;

#[salsa::tracked(constructor = new_inner)]
pub struct CrateDeclTokraRegion {
    #[return_ref]
    tokens_data: Vec<TokenData>,
}

#[derive(Debug, Clone, Copy)]
pub struct CrateDeclTokraRegionDataRef<'a> {
    tokens_data: &'a [TokenData],
}

impl CrateDeclTokraRegion {
    pub fn data<'a>(self, db: &'a ::salsa::Db) -> CrateDeclTokraRegionDataRef<'a> {
        CrateDeclTokraRegionDataRef {
            tokens_data: self.tokens_data(db),
        }
    }

    pub fn regional_tokens_data<'a>(self, db: &'a ::salsa::Db) -> RegionalTokensData<'a> {
        RegionalTokensData::new(self.tokens_data(db))
    }
}

pub trait HasCrateDeclTokraRegion {
    fn decl_tokra_region(self, db: &::salsa::Db) -> CrateDeclTokraRegion;
}

impl HasCrateDeclTokraRegion for CratePath {
    fn decl_tokra_region(self, db: &::salsa::Db) -> CrateDeclTokraRegion {
        todo!()
    }
}

impl<'a> CrateDeclTokraRegionDataRef<'a> {
    pub fn regional_token_stream(self) -> RegionalTokenStream<'a> {
        todo!("verses");
        RegionalTokenStream::new_decl_regional_token_stream(self.tokens_data, Some(todo!()))
    }
}
