pub struct Pool<T, const BLOCK_SIZE: usize> {
    blocks: Vec<Block<T, BLOCK_SIZE>>,
}

impl<T, const BLOCK_SIZE: usize> Default for Pool<T, BLOCK_SIZE> {
    fn default() -> Self {
        let result = Self {
            blocks: vec![Block::new()],
        };
        result
    }
}

impl<T, const BLOCK_SIZE: usize> Pool<T, BLOCK_SIZE> {
    // the pointer is guaranteed to be valid until self is dropped
    pub fn alloc(&mut self, t: T) -> *const T {
        let block = {
            if self.blocks.last().unwrap().is_full() {
                self.blocks.push(Block::new());
            }
            self.blocks.last_mut().unwrap()
        };
        block.alloc(t)
    }

    pub fn len(&self) -> usize {
        (self.blocks.len() - 1) * BLOCK_SIZE + self.blocks.last().unwrap().len()
    }
}

pub struct Block<T, const BLOCK_SIZE: usize> {
    storage: Vec<T>,
}

impl<T, const BLOCK_SIZE: usize> Block<T, BLOCK_SIZE> {
    fn new() -> Self {
        let mut storage = Vec::new();
        storage.reserve_exact(BLOCK_SIZE);
        let result = Self { storage };
        result
    }

    fn is_full(&self) -> bool {
        self.len() >= BLOCK_SIZE
    }

    fn alloc(&mut self, t: T) -> &T {
        assert!(!self.is_full());
        self.storage.push(t);
        self.storage.last().unwrap()
    }

    fn len(&self) -> usize {
        self.storage.len()
    }
}

#[test]
fn test_pool() {
    let mut pool = Pool::<i32, 2>::default();
    unsafe {
        let _a = &*pool.alloc(1);
        let _b = &*pool.alloc(1);
        let _c = &*pool.alloc(1);
        let _d = &*pool.alloc(1);
        let _e = &*pool.alloc(1);
        let _f = &*pool.alloc(1);
        assert_eq!(pool.blocks.len(), 3);
    }
}

#[test]
fn test_pool2() {
    let mut pool = Pool::<i32, 10000>::default();
    assert_eq!(std::mem::size_of::<Pool<i32, 10000>>(), 24);
    unsafe {
        let _a = &*pool.alloc(1);
        let _b = &*pool.alloc(1);
        let _c = &*pool.alloc(1);
        let _d = &*pool.alloc(1);
        let _e = &*pool.alloc(1);
        let _f = &*pool.alloc(1);
        assert_eq!(pool.blocks.len(), 1);
    }
}
