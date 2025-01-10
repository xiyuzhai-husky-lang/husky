use super::*;
use crate::elabm::ElabM;

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
        f: impl FnOnce(&mut VdBsqElaboratorInner<'db, 'sess>) -> R,
    ) -> R
    where
        'db: 'sess,
    {
        self.call_stack.enter_call(call);
        let result = f(self);
        self.call_stack.exit_call();
        result
    }
}

pub(crate) fn with_call<'db, 'sess, R>(
    call: impl Into<VdBsqCall>,
    m: impl ElabM<'db, 'sess, R>,
) -> impl ElabM<'db, 'sess, R>
where
    'db: 'sess,
{
    |elaborator: &mut Er<'db, 'sess>, f: &dyn Fn(&mut Er<'db, 'sess>, R) -> _| {
        elaborator.call_stack.enter_call(call);
        let result = m.eval(elaborator, f);
        elaborator.call_stack.exit_call();
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
