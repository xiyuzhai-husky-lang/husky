use super::*;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct DefnTokenRegion {
    #[id]
    syn_node_path: ItemSynNodePath,
    #[return_ref]
    tokens: Vec<Token>,
}

impl DefnTokenRegion {
    pub fn data<'a>(self, db: &'a dyn EntitySynTreeDb) -> DefnTokenRegionData<'a> {
        DefnTokenRegionData {
            tokens: self.tokens(db),
        }
    }
}

pub struct DefnTokenRegionData<'a> {
    tokens: &'a [Token],
}

impl<'a> std::ops::Index<RegionalTokenIdx> for DefnTokenRegionData<'a> {
    type Output = Token;

    fn index(&self, index: RegionalTokenIdx) -> &Self::Output {
        &self.tokens[index.0.get() as usize - 1]
    }
}

pub(super) fn defn_token_region(
    syn_node_path: ItemSynNodePath,
    db: &dyn EntitySynTreeDb,
) -> DefnTokenRegion {
    match syn_node_path {
        ItemSynNodePath::Submodule(_) => todo!(),
        ItemSynNodePath::MajorItem(_) => todo!(),
        ItemSynNodePath::TypeVariant(_) => todo!(),
        ItemSynNodePath::ImplBlock(_) => todo!(),
        ItemSynNodePath::AssociatedItem(_) => todo!(),
        ItemSynNodePath::Decr(_) => todo!(),
    }
}
