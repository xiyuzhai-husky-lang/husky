mod value;

pub use value::*;

use arrayvec::ArrayVec;
use word::CustomIdentifier;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackIdx(u8);

impl StackIdx {
    pub fn this() -> StackIdx {
        Self(0)
    }

    pub fn new(raw: usize) -> VMResult<StackIdx> {
        let raw: u8 = raw.try_into().unwrap();
        Ok(StackIdx(raw))
    }

    fn raw(&self) -> usize {
        self.0 as usize
    }
}

pub const STACK_SIZE: usize = 255;

#[derive(Debug)]
pub struct VMStack<'stack, 'eval: 'stack> {
    values: ArrayVec<StackValue<'stack, 'eval>, STACK_SIZE>,
    opt_this: Option<StackValue<'stack, 'eval>>,
}

impl<'stack, 'eval: 'stack> VMStack<'stack, 'eval> {
    pub(crate) fn try_new(
        iter: impl Iterator<Item = VMResult<StackValue<'stack, 'eval>>>,
    ) -> VMResult<Self> {
        let mut values = ArrayVec::new();
        for result in iter {
            values.push(result?)
        }
        Ok(Self {
            values,
            opt_this: None,
        })
    }

    pub(crate) fn new(iter: impl Iterator<Item = StackValue<'stack, 'eval>>) -> Self {
        let mut values = ArrayVec::new();
        for value in iter {
            values.push(value)
        }
        Self {
            values,
            opt_this: None,
        }
    }

    pub(crate) fn value(&self, idx: StackIdx) -> &StackValue<'stack, 'eval> {
        &self.values[idx.raw()]
    }

    pub(crate) fn value_mut(&mut self, idx: StackIdx) -> &mut StackValue<'stack, 'eval> {
        &mut self.values[idx.raw()]
    }

    pub(crate) fn push_variable(
        &mut self,
        stack_idx: StackIdx,
        contract: EagerContract,
    ) -> &mut StackValue<'stack, 'eval> {
        //  self.stack.variable(stack_idx).bind(,  stack_idx) };
        // self.stack.push(value);
        unsafe {
            let stack_value = self.values[stack_idx.raw()].bind(contract, stack_idx);
            self.push(stack_value);
        }
        self.values.last_mut().unwrap()
    }

    pub(crate) fn snapshot(&mut self) -> StackSnapshot<'eval> {
        StackSnapshot {
            values: self
                .values
                .iter_mut()
                .map(|value| value.snapshot())
                .collect(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn push(&mut self, value: StackValue<'stack, 'eval>) {
        self.values.push(value);
    }
    pub(crate) fn pop(&mut self) -> StackValue<'stack, 'eval> {
        self.values.pop().unwrap()
    }

    pub(crate) fn drain(&mut self, k: u8) -> Vec<StackValue<'stack, 'eval>> {
        self.values.drain((self.len() - k as usize)..).collect()
    }

    pub(crate) fn topk_mut(&mut self, k: u8) -> &mut [StackValue<'stack, 'eval>] {
        let len = self.len();
        &mut self.values[(len - (k as usize))..]
    }

    pub(crate) fn top_second(&self) -> &StackValue<'stack, 'eval> {
        &self.values[self.values.len() - 2]
    }

    pub(crate) fn top(&mut self) -> &StackValue<'stack, 'eval> {
        self.values.last_mut().unwrap()
    }

    pub(crate) fn top_mut(&mut self, mode: Mode) -> &mut StackValue<'stack, 'eval> {
        self.values.last_mut().unwrap()
    }
}

impl<'stack, 'eval: 'stack> From<Vec<StackValue<'stack, 'eval>>> for VMStack<'stack, 'eval> {
    fn from(values: Vec<StackValue<'stack, 'eval>>) -> Self {
        Self::new(values.into_iter())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableStack {
    variables: Vec<CustomIdentifier>,
    has_this: bool,
}

impl VariableStack {
    pub fn new(inputs: Vec<CustomIdentifier>, has_this: bool) -> Self {
        Self {
            variables: inputs,
            has_this,
        }
    }

    pub fn stack_idx(&self, ident0: CustomIdentifier) -> StackIdx {
        let idx = self.variables.len()
            - (1 + self
                .variables
                .iter()
                .rev()
                .position(|ident| *ident == ident0)
                .unwrap());
        StackIdx::new(if self.has_this { idx + 1 } else { idx }).unwrap()
    }

    pub fn push(&mut self, ident: CustomIdentifier) {
        self.variables.push(ident)
    }
}
