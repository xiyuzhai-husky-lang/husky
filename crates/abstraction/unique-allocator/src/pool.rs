use std::mem::MaybeUninit;

pub struct Pool<T, const BLOCK_SIZE: usize> {
    blocks: Vec<Block<T, BLOCK_SIZE>>,
}

impl<T, const BLOCK_SIZE: usize> Default for Pool<T, BLOCK_SIZE> {
    fn default() -> Self {
        Self {
            blocks: vec![Block::new()],
        }
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
}

pub struct Block<T, const BLOCK_SIZE: usize> {
    storage: Box<[MaybeUninit<T>; BLOCK_SIZE]>,
    len: usize,
}

impl<T, const BLOCK_SIZE: usize> Block<T, BLOCK_SIZE> {
    fn new() -> Self {
        Self {
            storage: Box::new(unsafe { MaybeUninit::uninit().assume_init() }),
            len: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.len >= BLOCK_SIZE
    }

    fn alloc(&mut self, t: T) -> &T {
        assert!(!self.is_full());
        let res = self.storage[self.len].write(t);
        self.len += 1;
        res
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
