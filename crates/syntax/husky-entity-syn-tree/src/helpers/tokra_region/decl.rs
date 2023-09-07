use super::*;
use husky_token::TokenGroupIdx;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct DeclTokraRegion {
    #[id]
    syn_node_path: ItemSynNodePath,
    #[return_ref]
    tokens: Vec<Token>,
}

impl DeclTokraRegion {
    pub fn from_singe_token_group(
        syn_node_path: ItemSynNodePath,
        token_group_idx: TokenGroupIdx,
        db: &dyn EntitySynTreeDb,
    ) -> Self {
        let module_path = syn_node_path.module_path(db);
        let token_sheet = db
            .token_sheet_data(module_path)
            .expect("todo: modules should be guaranteed to be valid");
        let tokens = token_sheet[token_group_idx].to_vec();
        DeclTokraRegion::new_inner(db, syn_node_path, tokens)
    }

    pub fn data<'a>(self, db: &'a dyn EntitySynTreeDb) -> DeclTokraRegionData<'a> {
        DeclTokraRegionData {
            tokens: self.tokens(db),
        }
    }
}

pub struct DeclTokraRegionData<'a> {
    tokens: &'a [Token],
}

impl<'a> std::ops::Index<RegionalTokenIdx> for DeclTokraRegionData<'a> {
    type Output = Token;

    fn index(&self, index: RegionalTokenIdx) -> &Self::Output {
        &self.tokens[index.0.get() as usize - 1]
    }
}

pub(super) fn decl_tokra_region(
    syn_node_path: ItemSynNodePath,
    db: &dyn EntitySynTreeDb,
) -> DeclTokraRegion {
    todo!()
}
