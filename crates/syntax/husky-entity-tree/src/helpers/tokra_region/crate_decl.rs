use super::*;
use husky_ast::range::AstTokenIdxRangeSheet;
use husky_item_defn_ast::ItemDefnAstArena;

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

struct CrateDeclTokraRegionBuilder<'a> {
    ast_sheet: &'a AstSheet,
    ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
    defn_ast_arena: ItemDefnAstArena,
    root_body: AstIdxRange,
    regional_token_verse_idx_base: RegionalTokenVerseIdxBase,
    regional_token_idx_base: RegionalTokenIdxBase,
    token_sheet_data: &'a TokenSheetData,
    ast_idx_map: Vec<AstIdx>,
    regional_token_idx_range_map: Vec<RegionalTokenIdxRange>,
    tokens_data: Vec<TokenData>,
    db: &'a ::salsa::Db,
}
