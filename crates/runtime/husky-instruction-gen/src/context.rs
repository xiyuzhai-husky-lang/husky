use husky_vm::VMStackIdx;

#[derive(Copy, Clone, Debug)]
pub(crate) enum InstructionGenContext {
    Normal,
    NewVirtualStruct { output_stack_idx: VMStackIdx },
}
