use crate::*;
use arrayvec::ArrayVec;
use check_utils::should_eq;
use map_collect::MapCollect;
use print_utils::{emsg_once, p};
use std::fmt::Write;
use word::CustomIdentifier;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VMStackIdx(u8);

impl std::fmt::Debug for VMStackIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("StackIdx({})", self.0))
    }
}

impl VMStackIdx {
    pub fn this() -> VMStackIdx {
        Self(0)
    }

    pub fn new(raw: usize) -> VMCompileResult<VMStackIdx> {
        let raw: u8 = raw.try_into().unwrap();
        Ok(VMStackIdx(raw))
    }

    pub(crate) fn raw(&self) -> usize {
        self.0 as usize
    }
}

pub const STACK_SIZE: usize = 255;

pub struct VMStack<'vm, 'eval: 'vm> {
    values: ArrayVec<VMValue<'vm, 'eval>, STACK_SIZE>,
}

impl<'vm, 'eval> std::fmt::Debug for VMStack<'vm, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VMStack")
            .field("values", &self.values)
            .finish()
    }
}

impl<'vm, 'eval: 'vm, T: Iterator<Item = VMValue<'vm, 'eval>>> From<T> for VMStack<'vm, 'eval> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<'vm, 'eval: 'vm> VMStack<'vm, 'eval> {
    pub(crate) fn try_new(
        argument_iter: impl Iterator<Item = VMRuntimeResult<VMValue<'vm, 'eval>>>,
    ) -> VMRuntimeResult<Self> {
        let mut values = ArrayVec::new();
        for result in argument_iter {
            values.push(result?)
        }
        Ok(Self { values })
    }

    pub(crate) fn new(argument_iter: impl Iterator<Item = VMValue<'vm, 'eval>>) -> Self {
        let mut values = ArrayVec::new();
        for value in argument_iter {
            values.push(value)
        }
        Self { values }
    }

    pub(crate) fn value(&self, idx: VMStackIdx) -> &VMValue<'vm, 'eval> {
        &self.values[idx.raw()]
    }

    pub(crate) fn value_mut(&mut self, idx: VMStackIdx) -> &mut VMValue<'vm, 'eval> {
        &mut self.values[idx.raw()]
    }

    pub(crate) fn push_variable(
        &mut self,
        stack_idx: VMStackIdx,
        binding: Binding,
    ) -> &mut VMValue<'vm, 'eval> {
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

    pub(crate) fn eval(&mut self, stack_idx: VMStackIdx) -> EvalValue<'eval> {
        self.values[stack_idx.raw()].eval()
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn push(&mut self, value: VMValue<'vm, 'eval>) {
        self.values.push(value);
    }
    pub(crate) fn pop(&mut self) -> VMValue<'vm, 'eval> {
        self.values.pop().unwrap()
    }

    pub(crate) fn drain<'a>(&'a mut self, k: u8) -> impl Iterator<Item = VMValue<'vm, 'eval>> + 'a {
        self.values.drain((self.len() - k as usize)..)
    }

    pub(crate) fn eval_top(&mut self) -> EvalValue<'eval> {
        self.values.last().unwrap().eval()
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        self.values.truncate(len)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct VariableStack {
    non_this_variables: Vec<CustomIdentifier>,
    has_this: bool,
}

impl std::fmt::Debug for VariableStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\nVariableStack:\n")?;
        f.write_fmt(format_args!("    has_this: {}\n", self.has_this))?;
        f.write_str("    variables:\n")?;
        for (i, ident) in self.non_this_variables.iter().enumerate() {
            f.write_fmt(format_args!("        {: <3} {}\n", i, ident.as_str()))?
        }
        f.write_str("\n")
    }
}

impl VariableStack {
    pub fn new(inputs: impl Iterator<Item = CustomIdentifier>, has_this: bool) -> Self {
        Self {
            non_this_variables: inputs.map(|ident| ident).collect(),
            has_this,
        }
    }

    pub fn len(&self) -> usize {
        self.non_this_variables.len()
    }

    pub fn stack_idx(&self, ident0: CustomIdentifier) -> VMStackIdx {
        let idx = self.non_this_variables.len()
            - (1 + self
                .non_this_variables
                .iter()
                .rev()
                .position(|ident| *ident == ident0)
                .unwrap());
        VMStackIdx::new(if self.has_this { idx + 1 } else { idx }).unwrap()
    }

    pub fn push(&mut self, ident: CustomIdentifier) {
        self.non_this_variables.push(ident)
    }

    pub fn varname(&self, stack_idx: VMStackIdx) -> CustomIdentifier {
        self.non_this_variables[stack_idx.0 as usize]
    }

    pub fn compare_with_vm_stack(&self, vm_stack: &VMStack) -> String {
        let mut result = String::new();
        write!(result, "VariableStack:\n");
        write!(result, "    has_this: {}\n", self.has_this);
        if self.has_this {
            write!(
                result,
                "        this: {}\n",
                vm_stack.values[0].print_short()
            );
        }
        write!(result, "    variables:\n");
        let shift = if self.has_this { 1 } else { 0 };
        for (i, ident) in self.non_this_variables.iter().enumerate() {
            write!(
                result,
                "        #{: <3} {}{: <10}{} ",
                i,
                print_utils::CYAN,
                ident.as_str(),
                print_utils::RESET,
            );
            if i + shift < vm_stack.values.len() {
                write!(result, "{}\n", vm_stack.values[i + shift].print_short()).unwrap()
            } else {
                write!(result, "uninitialized\n").unwrap()
            }
        }

        for i in self.non_this_variables.len()..(vm_stack.values.len() - shift) {
            write!(
                result,
                "        #{: <3} {}{: <10}{} ",
                i,
                print_utils::RED,
                "$",
                print_utils::RESET,
            )
            .unwrap();
            write!(result, "{}\n", vm_stack.values[i + shift].print_short()).unwrap()
        }
        result.push('\n');
        result
    }
}
