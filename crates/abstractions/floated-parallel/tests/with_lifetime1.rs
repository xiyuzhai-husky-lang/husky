use floated_parallel::floated;

#[floated]
pub struct List<'db> {
    pub first: i32,
    pub others: List<'db>,
}

impl<'db> std::fmt::Debug for List<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
