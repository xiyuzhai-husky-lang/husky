use vec_array::VecArray;

pub struct Pool<T, const N: usize> {
    blocks: Vec<VecArray<T, N>>,
}

impl<T, const N: usize> Default for Pool<T, N> {
    fn default() -> Self {
        Self { blocks: vec![] }
    }
}

impl<T, const N: usize> Pool<T, N> {
    pub fn alloc(&mut self, t: T) -> *const T {
        let last_block = match self.blocks.last_mut() {
            Some(last_block) => {
                if last_block.is_full() {
                    self.blocks.push(VecArray::new());
                    self.blocks.last_mut().unwrap()
                } else {
                    last_block
                }
            }
            None => {
                self.blocks.push(VecArray::new());
                self.blocks.last_mut().unwrap()
            }
        };
        last_block.push(t);
        last_block.last().unwrap()
    }

    pub fn len(&self) -> usize {
        if self.blocks.is_empty() {
            0
        } else {
            (self.blocks.len() - 1) * N + self.blocks.last().unwrap().len()
        }
    }
}

impl<T, const N: usize> std::ops::Index<u32> for Pool<T, N> {
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        let index = index as usize;
        let block_index = index / N;
        let block = &self.blocks[block_index];
        let item_index = index % N;
        &block[item_index]
    }
}
