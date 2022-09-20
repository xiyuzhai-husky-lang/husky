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

impl<V> std::ops::DerefMut for ProjAtom<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.updated = false;
        &mut self.value
    }
}
