pub struct __RegisterPrototype {
    pub copy_handler: unsafe fn(u64) -> u64,
    pub clone_handler: unsafe fn(u64) -> u64,
    pub drop_handler: unsafe fn(u64),
    pub opt_assign_handler: Option<unsafe fn(u64, u64)>,
}

pub struct __Register {
    pub qual: __RegisterRawKind,
    pub raw: u64,
    pub proto: &'static __RegisterPrototype,
}

impl __Register {
    pub unsafe fn copy(&self) -> Self {
        Self {
            qual: self.qual,
            raw: match self.qual {
                __RegisterRawKind::Data => todo!(),
                __RegisterRawKind::Box => todo!(),
                __RegisterRawKind::EvalRef => todo!(),
                __RegisterRawKind::TempRef => todo!(),
                __RegisterRawKind::TempMut => todo!(),
                __RegisterRawKind::Moved => todo!(),
            },
            proto: self.proto,
        }
    }

    pub unsafe fn downcast_ref<T>() {
        todo!()
    }

    pub unsafe fn downcast_move<T>() {
        todo!()
    }

    pub unsafe fn downcast_copy<T>() {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum __RegisterRawKind {
    Data,
    Box,
    EvalRef,
    TempRef,
    TempMut,
    Moved,
}

impl Drop for __Register {
    fn drop(&mut self) {
        match self.qual {
            __RegisterRawKind::Box => unsafe { (self.proto.drop_handler)(self.raw) },
            _ => (),
        }
    }
}
