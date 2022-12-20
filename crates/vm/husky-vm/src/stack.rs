use crate::*;
use arrayvec::ArrayVec;
use husky_word::Identifier;


pub const STACK_SIZE: usize = 255;

pub struct VMStack<'eval> {
    values: ArrayVec<__Register<'eval>, STACK_SIZE>,
}

impl<'eval> std::fmt::Debug for VMStack<'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VMStack")
            .field("values", &self.values)
            .finish()
    }
}

impl<'eval, T: Iterator<Item = __Register<'eval>>> From<T> for VMStack<'eval> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<'eval> VMStack<'eval> {
    pub(crate) fn try_new(
        argument_iter: impl Iterator<Item = __VMResult<__Register<'eval>>>,
    ) -> __VMResult<Self> {
        let mut values = ArrayVec::new();
        for result in argument_iter {
            values.push(result?)
        }
        Ok(Self { values })
    }

    pub(crate) fn new(argument_iter: impl Iterator<Item = __Register<'eval>>) -> Self {
        let mut values = ArrayVec::new();
        for value in argument_iter {
            values.push(value)
        }
        Self { values }
    }

    pub(crate) fn value(&self, idx: VMStackIdx) -> &__Register<'eval> {
        &self.values[idx.raw()]
    }

    pub(crate) fn value_mut(&mut self, idx: VMStackIdx) -> &mut __Register<'eval> {
        &mut self.values[idx.raw()]
    }

    pub(crate) fn push_variable(
        &mut self,
        stack_idx: VMStackIdx,
        binding: Binding,
    ) -> &mut __Register<'eval> {
        unsafe {
            let value = &mut self.values[stack_idx.raw()];
            let stack_value = value.bind(binding);
            self.push(stack_value);
        }
        self.values.last_mut().unwrap()
    }

    pub(crate) fn snapshot(&mut self, message: String) -> StackSnapshot<'eval> {
        StackSnapshot {
            message,
            values: self
                .values
                .iter_mut()
                .map(|value| value.snapshot())
                .collect(),
        }
    }

    pub(crate) fn eval(&mut self, stack_idx: VMStackIdx) -> __Register<'eval> {
        self.values[stack_idx.raw()].snapshot()
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn push(&mut self, value: __Register<'eval>) {
        assert_ne!(value.vtable as *const _, &__VOID_VTABLE as *const _);
        self.values.push(value);
    }
    pub(crate) fn pop(&mut self) -> __Register<'eval> {
        self.values.pop().unwrap()
    }

    pub(crate) fn drain<'a>(&'a mut self, k: u8) -> impl Iterator<Item = __Register<'eval>> + 'a {
        self.values.drain((self.len() - k as usize)..)
    }

    pub(crate) fn eval_top(&mut self) -> __Register<'eval> {
        self.values.last().unwrap().snapshot()
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        self.values.truncate(len)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct VariableStack {
    non_this_variables: Vec<Identifier>,
    has_this: bool,
}

impl std::fmt::Debug for VariableStack {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // f.write_str("\nVariableStack:\n")?;
        // f.write_fmt(format_args!("    has_this: {}\n", self.has_this))?;
        // f.write_str("    variables:\n")?;
        // for (i, ident) in self.non_this_variables.iter().enumerate() {
        //     f.write_fmt(format_args!("        {: <3} {}\n", i, ident.as_str()))?
        // }
        // f.write_str("\n")
    }
}

impl VariableStack {
    pub fn new(inputs: impl Iterator<Item = Identifier>, has_this: bool) -> Self {
        Self {
            non_this_variables: inputs.map(|ident| ident).collect(),
            has_this,
        }
    }

    pub fn len(&self) -> usize {
        self.non_this_variables.len()
    }

    pub fn next_stack_idx(&self) -> VMStackIdx {
        VMStackIdx::new(self.non_this_variables.len())
    }

    pub fn stack_idx(&self, ident0: Identifier) -> VMStackIdx {
        let idx = self.non_this_variables.len()
            - (1 + self
                .non_this_variables
                .iter()
                .rev()
                .position(|ident| *ident == ident0)
                .unwrap());
        VMStackIdx::new(if self.has_this { idx + 1 } else { idx })
    }

    pub fn push(&mut self, ident: Identifier) {
        self.non_this_variables.push(ident)
    }

    pub fn varname(&self, stack_idx: VMStackIdx) -> Identifier {
        self.non_this_variables[stack_idx.raw() as usize]
    }

    pub fn compare_with_vm_stack(&self, _vm_stack: &VMStack) -> String {
        todo!()
        // let mut result = String::new();
        // write!(result, "VariableStack:\n").unwrap();
        // write!(result, "    has_this: {}\n", self.has_this).unwrap();
        // if self.has_this {
        //     write!(
        //         result,
        //         "        this: {}\n",
        //         vm_stack.values[0].print_short()
        //     )
        //     .unwrap();
        // }
        // write!(result, "    variables:\n").unwrap();
        // let shift = if self.has_this { 1 } else { 0 };
        // for (i, ident) in self.non_this_variables.iter().enumerate() {
        //     write!(
        //         result,
        //         "        #{: <3} {}{: <10}{} ",
        //         i,
        //         husky_print_utils::CYAN,
        //         ident.as_str(),
        //         husky_print_utils::RESET,
        //     )
        //     .unwrap();
        //     if i + shift < vm_stack.values.len() {
        //         write!(result, "{}\n", vm_stack.values[i + shift].print_short()).unwrap()
        //     } else {
        //         write!(result, "uninitialized\n").unwrap()
        //     }
        // }

        // for i in self.non_this_variables.len()..(vm_stack.values.len() - shift) {
        //     write!(
        //         result,
        //         "        #{: <3} {}{: <10}{} ",
        //         i,
        //         husky_print_utils::RED,
        //         "$",
        //         husky_print_utils::RESET,
        //     )
        //     .unwrap();
        //     write!(result, "{}\n", vm_stack.values[i + shift].print_short()).unwrap()
        // }
        // result.push('\n');
        // result
    }
}
