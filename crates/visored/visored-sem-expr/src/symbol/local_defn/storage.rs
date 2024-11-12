use super::*;

pub struct VdSemSymbolLocalDefnStorage {
    arena: VdSemSymbolLocalDefnArena,
}

impl std::ops::Deref for VdSemSymbolLocalDefnStorage {
    type Target = VdSemSymbolLocalDefnArena;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}

impl VdSemSymbolLocalDefnStorage {
    pub(crate) fn new_empty() -> Self {
        Self {
            arena: Default::default(),
        }
    }

    pub(crate) fn set_defns(&mut self, defns: Vec<VdSemSymbolLocalDefnData>) {
        debug_assert!(self.arena.is_empty());
        self.arena = unsafe { VdSemSymbolLocalDefnArena::new_ext(defns) };
    }

    pub(crate) fn set_local_defn_ty(&mut self, local_defn: VdSemSymbolLocalDefnIdx, ty: VdZfcType) {
        self.arena.update(local_defn, |data| data.set_ty(ty));
    }
}
