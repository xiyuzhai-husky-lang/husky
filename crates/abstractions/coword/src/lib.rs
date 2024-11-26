#[interned::interned]
pub struct Coword {
    data: String,
}

impl Coword {
    pub fn from_ref(s: &str) -> Self {
        todo!()
        // Self::new(s)
    }
}

impl std::fmt::Debug for Coword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Coword").field(self.data()).finish()
    }
}
