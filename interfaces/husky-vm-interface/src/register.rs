pub struct __RegisterPrototype {
    pub copy_handler: unsafe fn(u64) -> u64,
    pub clone_handler: unsafe fn(u64) -> u64,
    pub drop_handler: unsafe fn(u64),
    pub opt_assign_handler: Option<unsafe fn(u64, u64)>,
}

pub struct __Register {
    pub data_kind: __RegisterDataKind,
    pub data: u64,
    pub proto: &'static __RegisterPrototype,
}

impl __Register {
    pub unsafe fn copy(&self) -> Self {
        Self {
            data_kind: self.data_kind,
            data: match self.data_kind {
                __RegisterDataKind::Data => todo!(),
                __RegisterDataKind::Box => todo!(),
                __RegisterDataKind::EvalRef => todo!(),
                __RegisterDataKind::TempRef => todo!(),
                __RegisterDataKind::TempMut => todo!(),
                __RegisterDataKind::Moved => todo!(),
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
pub enum __RegisterDataKind {
    Data,
    Box,
    EvalRef,
    TempRef,
    TempMut,
    Moved,
}

impl Drop for __Register {
    fn drop(&mut self) {
        match self.data_kind {
            __RegisterDataKind::Box => unsafe { (self.proto.drop_handler)(self.data) },
            _ => (),
        }
    }
}
