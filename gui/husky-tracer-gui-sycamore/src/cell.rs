use std::cell::{Cell, Ref, RefCell, RefMut};

pub struct OptionCell<T>(Cell<Option<T>>);

impl<T> Default for OptionCell<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> OptionCell<T> {
    pub fn is_some(&self) -> bool {
        let t = self.0.take();
        let is_some = t.is_some();
        self.0.set(t);
        is_some
    }

    pub fn set(&self, t: T) {
        self.0.set(Some(t))
    }

    pub fn pop(&self) -> Option<T> {
        self.0.take()
    }
}

#[derive(Debug, Default)]
pub struct InformativeRefCell<T> {
    last_borrow_mut: Cell<Option<(&'static str, u32)>>,
    cell: RefCell<T>,
}

impl<T> InformativeRefCell<T> {
    pub fn new(value: T) -> InformativeRefCell<T> {
        Self {
            last_borrow_mut: Cell::new(None),
            cell: RefCell::new(value),
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
