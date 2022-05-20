use crate::*;
use arrayvec::ArrayVec;
use check_utils::should_eq;
use map_collect::MapCollect;
use print_utils::{emsg_once, p};
use std::fmt::Write;
use word::CustomIdentifier;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StackIdx(u8);

impl std::fmt::Debug for StackIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("StackIdx({})", self.0))
    }
}

impl StackIdx {
    pub fn this() -> StackIdx {
        Self(0)
    }

    pub fn new(raw: usize) -> VMCompileResult<StackIdx> {
        let raw: u8 = raw.try_into().unwrap();
        Ok(StackIdx(raw))
    }

    pub(crate) fn raw(&self) -> usize {
        self.0 as usize
    }
}

pub const STACK_SIZE: usize = 255;

pub struct VMStack<'stack, 'eval: 'stack> {
    values: ArrayVec<StackValue<'stack, 'eval>, STACK_SIZE>,
    opt_this: Option<StackValue<'stack, 'eval>>,
}

impl<'stack, 'eval> std::fmt::Debug for VMStack<'stack, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VMStack")
            .field("values", &self.values)
            .field("opt_this", &self.opt_this)
            .finish()
    }
}

impl<'stack, 'eval: 'stack> VMStack<'stack, 'eval> {
    pub(crate) fn try_new(
        iter: impl Iterator<Item = VMRuntimeResult<StackValue<'stack, 'eval>>>,
    ) -> VMRuntimeResult<Self> {
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
            let value = &mut self.values[stack_idx.raw()];
            let stack_value = value.bind(binding, stack_idx);
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

    pub(crate) fn eval(&mut self, stack_idx: StackIdx) -> EvalValue<'eval> {
        self.values[stack_idx.raw()].eval()
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

    // pub(crate) fn top_second(&self) -> &StackValue<'stack, 'eval> {
    //     &self.values[self.values.len() - 2]
    // }

    // pub(crate) fn top(&mut self) -> &StackValue<'stack, 'eval> {
    //     self.values.last_mut().unwrap()
    // }

    pub(crate) fn eval_top(&mut self) -> EvalValue<'eval> {
        self.values.last().unwrap().eval()
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        self.values.truncate(len)
    }
}

impl<'stack, 'eval: 'stack> From<Vec<StackValue<'stack, 'eval>>> for VMStack<'stack, 'eval> {
    fn from(values: Vec<StackValue<'stack, 'eval>>) -> Self {
        Self::new(values.into_iter())
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct VariableStack {
    variables: Vec<CustomIdentifier>,
    has_this: bool,
}

impl std::fmt::Debug for VariableStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\nVariableStack:\n")?;
        f.write_fmt(format_args!("    has_this: {}\n", self.has_this))?;
        f.write_str("    variables:\n")?;
        for (i, ident) in self.variables.iter().enumerate() {
            f.write_fmt(format_args!("        {: <3} {}\n", i, ident.as_str()))?
        }
        f.write_str("\n")
    }
}

impl VariableStack {
    pub fn new(inputs: impl Iterator<Item = CustomIdentifier>, has_this: bool) -> Self {
        Self {
            variables: inputs.map(|ident| ident).collect(),
            has_this,
        }
    }

    pub fn len(&self) -> usize {
        self.variables.len()
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

    pub fn varname(&self, stack_idx: StackIdx) -> CustomIdentifier {
        self.variables[stack_idx.0 as usize]
    }

    pub fn compare_with_vm_stack(&self, vm_stack: &VMStack) -> String {
        let mut result = String::new();
        should_eq!(vm_stack.opt_this.is_some(), self.has_this);
        write!(result, "VariableStack:\n");
        write!(result, "    has_this: {}\n", self.has_this);
        write!(result, "    variables:\n");
        for (i, ident) in self.variables.iter().enumerate() {
            write!(
                result,
                "        #{: <3} {}{: <10}{} ",
                i,
                print_utils::CYAN,
                ident.as_str(),
                print_utils::RESET,
            );
            if i < vm_stack.values.len() {
                write!(result, "{}\n", vm_stack.values[i].print_short()).unwrap()
            } else {
                write!(result, "uninitialized\n").unwrap()
            }
        }

        for i in self.variables.len()..vm_stack.values.len() {
            write!(
                result,
                "        #{: <3} {}{: <10}{} ",
                i,
                print_utils::RED,
                "$",
                print_utils::RESET,
            )
            .unwrap();
            write!(result, "{}\n", vm_stack.values[i].print_short()).unwrap()
        }
        result.push('\n');
        result
    }
}
