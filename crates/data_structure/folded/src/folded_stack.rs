pub struct FoldedStack<T> {
    list: Vec<T>,
    block_indices: Vec<usize>,
}

impl<T> FoldedStack<T> {
    pub fn new() -> FoldedStack<T> {
        Self {
            list: Vec::new(),
            block_indices: vec![0],
        }
    }
    pub fn append(&mut self, item: T) {
        self.list.push(item);
    }

    pub fn enter_fold(&mut self) {
        self.block_indices.push(self.list.len());
    }

    pub fn exit_fold(&mut self) {
        let block_index = self.block_indices.pop().unwrap();
        self.list.truncate(block_index);
    }

    pub fn locals(&self) -> &[T] {
        &self.list[(*self.block_indices.last().unwrap())..(self.list.len())]
    }

    pub fn for_all_local(&self, f: impl Fn(&T) -> bool) -> bool {
        self.locals().iter().all(f)
    }

    pub fn for_any_local(&self, f: impl Fn(&T) -> bool) -> bool {
        self.locals().iter().any(f)
    }

    pub fn find(&self, f: impl Fn(&T) -> bool) -> Option<&T> {
        self.list.iter().rev().find(|item| f(*item))
    }
}
