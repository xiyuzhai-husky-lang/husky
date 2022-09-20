use crate::ProjMakeChangeM;

#[derive(Default)]
pub struct ProjAtom<V> {
    value: V,
    updated: bool,
}

impl<V> std::ops::Deref for ProjAtom<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> ProjAtom<V> {
    pub fn set(&mut self, new_value: V) -> ProjMakeChangeM<Self, ()> {
        self.updated = false;
        self.value = new_value;
        ProjMakeChangeM::Ok {
            cont: (),
            phantom_state: std::marker::PhantomData,
        }
    }
    pub fn update(&mut self, f: impl FnOnce(&mut V)) -> ProjMakeChangeM<Self, ()> {
        self.updated = false;
        f(&mut self.value);
        ProjMakeChangeM::Ok {
            cont: (),
            phantom_state: std::marker::PhantomData,
        }
    }
}
