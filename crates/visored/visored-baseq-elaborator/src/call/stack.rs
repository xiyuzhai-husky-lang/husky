use super::*;

pub struct VdBsqCallStack {
    stack: Vec<VdBsqCall>,
}

impl VdBsqCallStack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn with_call<R>(
        &mut self,
        call: impl Into<VdBsqCall>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.call_stack.enter_call(call);
        let result = f(self);
        self.call_stack.exit_call();
        result
    }
}

impl VdBsqCallStack {
    pub fn enter_call(&mut self, call: impl Into<VdBsqCall>) {
        self.stack.push(call.into());
    }

    pub fn exit_call(&mut self) {
        self.stack.pop();
    }
}
