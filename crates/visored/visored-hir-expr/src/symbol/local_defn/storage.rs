use super::*;

pub struct VdHirSymbolLocalDefnStorage {
    defn_arena: VdHirSymbolLocalDefnArena,
}
impl VdHirSymbolLocalDefnStorage {
    pub(crate) fn new_empty() -> Self {
        Self {
            defn_arena: VdHirSymbolLocalDefnArena::default(),
        }
    }

    pub fn defn_arena(&self) -> &VdHirSymbolLocalDefnArena {
        &self.defn_arena
    }
}

impl VdHirSymbolLocalDefnStorage {
    pub(crate) fn set_defns(&mut self, data: Vec<VdHirSymbolLocalDefnData>) {
        self.defn_arena = unsafe { VdHirSymbolLocalDefnArena::new_ext(data) };
    }
}
