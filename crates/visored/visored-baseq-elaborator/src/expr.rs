use floated_sequential::{db::FloaterDb, floated};
use smallvec::SmallVec;
use visored_entity_path::path::VdItemPath;
use visored_mir_expr::{
    expr::{
        application::VdMirFunc, VdMirExprArena, VdMirExprArenaRef, VdMirExprData,
        VdMirExprIdxRange, VdMirExprMap, VdMirExprOrderedMap,
    },
    symbol::local_defn::VdMirSymbolLocalDefnIdx,
};
use visored_opr::separator::VdBaseSeparator;
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;
use visored_term::term::literal::VdLiteral;

use crate::{session::VdBaseqSession, term::VdMirTermFld};

#[floated]
pub struct VdMirExprFld<'sess> {
    #[return_ref]
    data: VdMirExprFldData<'sess>,
}

impl<'sess> std::fmt::Debug for VdMirExprFld<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdMirExprFldData<'sess> {
    Literal(VdLiteral),
    Variable(VdMirSymbolLocalDefnIdx),
    Application {
        function: VdMirFunc,
        arguments: VdMirExprFlds<'sess>,
    },
    FoldingSeparatedList {
        leader: VdMirExprFld<'sess>,
        /// TODO: should we use VdBaseSeparatorSignature instead?
        followers: SmallVec<[(VdMirFunc, VdMirExprFld<'sess>); 4]>,
    },
    ChainingSeparatedList {
        leader: VdMirExprFld<'sess>,
        followers: SmallVec<[(VdMirFunc, VdMirExprFld<'sess>); 4]>,
        joined_separator_and_signature: Option<(VdBaseSeparator, VdBaseSeparatorSignature)>,
    },
    ItemPath(VdItemPath),
}

pub type VdMirExprFlds<'sess> = SmallVec<[VdMirExprFld<'sess>; 4]>;

impl<'sess> VdMirExprFld<'sess> {
    pub fn term(self, db: &'sess FloaterDb) -> VdMirTermFld<'sess> {
        todo!()
    }
}

pub fn build_expr_to_fld_map<'db, 'sess>(
    session: &'sess VdBaseqSession<'db>,
    expr_arena: VdMirExprArenaRef,
) -> VdMirExprOrderedMap<VdMirExprFld<'sess>> {
    let mut map = VdMirExprOrderedMap::<VdMirExprFld<'sess>>::default();
    for (idx, entry) in expr_arena.indexed_iter() {
        let expr_fld = build_expr_to_fld_map_step(session, entry.data(), &map);
        map.insert_next(idx, expr_fld);
    }
    map
}

fn build_expr_to_fld_map_step<'db, 'sess>(
    session: &'sess VdBaseqSession<'db>,
    expr: &VdMirExprData,
    map: &VdMirExprOrderedMap<VdMirExprFld<'sess>>,
) -> VdMirExprFld<'sess> {
    match *expr {
        VdMirExprData::Literal(vd_literal) => todo!(),
        VdMirExprData::Variable(arena_idx) => todo!(),
        VdMirExprData::Application {
            function,
            arguments,
        } => todo!(),
        VdMirExprData::FoldingSeparatedList {
            leader,
            ref followers,
        } => todo!(),
        VdMirExprData::ChainingSeparatedList {
            leader,
            ref followers,
            joined_separator_and_signature,
        } => todo!(),
        VdMirExprData::ItemPath(vd_item_path) => todo!(),
    }
}
