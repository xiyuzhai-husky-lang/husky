use std::cell::{Cell, Ref, RefCell as StdRefCell, RefMut};

#[derive(Debug, Default)]
pub struct RefCell<T> {
    last_borrow_mut: Cell<Option<(&'static str, u32)>>,
    cell: StdRefCell<T>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> RefCell<T> {
        Self {
            last_borrow_mut: Cell::new(None),
            cell: StdRefCell::new(value),
        }
    }
    pub fn borrow(&self, file: &'static str, line: u32) -> Ref<'_, T> {
        let old = self.last_borrow_mut.get();
        match self.cell.try_borrow() {
            Ok(borrow_ref) => borrow_ref,
            Err(_) => {
                log::info!("last borrowed at {}:{}", old.unwrap().0, old.unwrap().1);
                log::info!("borrowed at {}:{}", file, line);
                panic!()
            }
        }
    }

    pub fn replace(&self, value: T) -> T {
        self.cell.replace(value)
    }

    pub fn borrow_mut(&self, file: &'static str, line: u32) -> RefMut<'_, T> {
        let old = self.last_borrow_mut.replace(Some((file, line)));
        match self.cell.try_borrow_mut() {
            Ok(borrow_mut) => borrow_mut,
            Err(e) => {
                log::info!("last borrowed at {}:{}", old.unwrap().0, old.unwrap().1);
                panic!()
            }
        }
    }
}
