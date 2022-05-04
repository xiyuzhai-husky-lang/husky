mod value;

use map_collect::MapCollect;
use print_utils::{msg_once, p};
pub use value::*;

use arrayvec::ArrayVec;
use word::CustomIdentifier;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StackIdx(u8);

impl StackIdx {
    pub fn this() -> StackIdx {
        Self(0)
    }

    pub fn new(raw: usize) -> VMResult<StackIdx> {
        let raw: u8 = raw.try_into().unwrap();
        Ok(StackIdx(raw))
    }

    pub(crate) fn raw(&self) -> usize {
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
        binding: Binding,
    ) -> &mut StackValue<'stack, 'eval> {
        unsafe {
            let stack_value = self.values[stack_idx.raw()].bind(binding, stack_idx);
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

    pub(crate) fn snapshot_value(&mut self, stack_idx: StackIdx) -> StackValueSnapshot<'eval> {
        self.values[stack_idx.raw()].snapshot()
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

    pub(crate) fn top_second(&self) -> &StackValue<'stack, 'eval> {
        &self.values[self.values.len() - 2]
    }

    pub(crate) fn top(&mut self) -> &StackValue<'stack, 'eval> {
        self.values.last_mut().unwrap()
    }

    pub(crate) fn top_snapshot(&mut self) -> StackValueSnapshot<'eval> {
        self.values.last_mut().unwrap().snapshot()
    }
}

impl<'stack, 'eval: 'stack> From<Vec<StackValue<'stack, 'eval>>> for VMStack<'stack, 'eval> {
    fn from(values: Vec<StackValue<'stack, 'eval>>) -> Self {
        Self::new(values.into_iter())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableStack {
    variables: Vec<Option<CustomIdentifier>>,
    has_this: bool,
}

impl VariableStack {
    pub fn new(inputs: impl Iterator<Item = CustomIdentifier>, has_this: bool) -> Self {
        Self {
            variables: inputs.map(|ident| Some(ident)).collect(),
            has_this,
        }
    }

    pub fn stack_idx(&self, ident0: CustomIdentifier) -> StackIdx {
        let idx = self.variables.len()
            - (1 + self
                .variables
                .iter()
                .rev()
                .position(|ident| *ident == Some(ident0))
                .unwrap());
        StackIdx::new(if self.has_this { idx + 1 } else { idx }).unwrap()
    }

    pub fn push(&mut self, opt_ident: Option<CustomIdentifier>) {
        self.variables.push(opt_ident)
    }

    pub fn varname(&self, stack_idx: StackIdx) -> CustomIdentifier {
        self.variables[stack_idx.0 as usize].unwrap()
    }
}
