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

    pub(crate) fn set(&mut self, defns: Vec<VdSemSymbolLocalDefnData>) {
        debug_assert!(self.arena.is_empty());
        self.arena = unsafe { VdSemSymbolLocalDefnArena::new_ext(defns) };
    }
}
