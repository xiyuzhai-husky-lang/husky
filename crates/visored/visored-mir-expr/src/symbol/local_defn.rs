pub mod storage;

use crate::*;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_math_letter::letter::LxMathLetter;
use visored_sem_expr::symbol::local_defn::{
    storage::VdSemSymbolLocalDefnStorage, VdSemSymbolLocalDefnBody, VdSemSymbolLocalDefnData,
    VdSemSymbolLocalDefnHead, VdSemSymbolLocalDefnIdx,
};

pub struct VdHirSymbolLocalDefnData {
    head: VdHirSymbolLocalDefnHead,
    body: VdHirSymbolLocalDefnBody,
}

pub enum VdHirSymbolLocalDefnHead {
    Letter(LxMathLetter),
}

pub enum VdHirSymbolLocalDefnBody {
    Assigned,
    Placeholder,
}

pub type VdHirSymbolLocalDefnArena = Arena<VdHirSymbolLocalDefnData>;
pub type VdHirSymbolLocalDefnArenaRef<'a> = ArenaRef<'a, VdHirSymbolLocalDefnData>;
pub type VdHirSymbolLocalDefnIdx = ArenaIdx<VdHirSymbolLocalDefnData>;
pub type VdHirSymbolLocalDefnIdxRange = ArenaIdxRange<VdHirSymbolLocalDefnData>;
pub type VdHirSymbolLocalDefnMap<T> = ArenaMap<VdHirSymbolLocalDefnData, T>;
pub type VdHirSymbolLocalDefnOrderedMap<T> = ArenaOrderedMap<VdHirSymbolLocalDefnData, T>;

impl VdHirSymbolLocalDefnData {
    pub fn head(&self) -> &VdHirSymbolLocalDefnHead {
        &self.head
    }

    pub fn body(&self) -> &VdHirSymbolLocalDefnBody {
        &self.body
    }
}

impl<'a> VdHirExprBuilder<'a> {
    pub(crate) fn build_symbol_local_defns(
        &mut self,
        sem_symbol_local_defn_storage: &VdSemSymbolLocalDefnStorage,
    ) {
        let mut defns = vec![];
        for defn in sem_symbol_local_defn_storage.defn_arena() {
            defns.push(self.build_symbol_local_defn(defn));
        }
        self.alloc_symbol_local_defns(defns);
    }

    fn build_symbol_local_defn(
        &mut self,
        defn: &VdSemSymbolLocalDefnData,
    ) -> VdHirSymbolLocalDefnData {
        VdHirSymbolLocalDefnData {
            head: defn.head().to_vd_hir(self),
            body: defn.body().to_vd_hir(self),
        }
    }
}

impl ToVdHir<VdHirSymbolLocalDefnIdx> for VdSemSymbolLocalDefnIdx {
    fn to_vd_hir(self, builder: &mut VdHirExprBuilder) -> VdHirSymbolLocalDefnIdx {
        // INVARIANCE: the index is always the same as the syn index
        unsafe { VdHirSymbolLocalDefnIdx::new_ext(self.index()) }
    }
}

impl ToVdHir<VdHirSymbolLocalDefnHead> for &VdSemSymbolLocalDefnHead {
    fn to_vd_hir(self, builder: &mut VdHirExprBuilder) -> VdHirSymbolLocalDefnHead {
        match *self {
            VdSemSymbolLocalDefnHead::Letter(_, letter) => VdHirSymbolLocalDefnHead::Letter(letter),
        }
    }
}

impl ToVdHir<VdHirSymbolLocalDefnBody> for &VdSemSymbolLocalDefnBody {
    fn to_vd_hir(self, builder: &mut VdHirExprBuilder) -> VdHirSymbolLocalDefnBody {
        match self {
            VdSemSymbolLocalDefnBody::Assigned => VdHirSymbolLocalDefnBody::Assigned,
            VdSemSymbolLocalDefnBody::Placeholder => VdHirSymbolLocalDefnBody::Placeholder,
        }
    }
}
