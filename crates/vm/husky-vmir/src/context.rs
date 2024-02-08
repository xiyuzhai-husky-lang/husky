use crate::VMStackIdx;

#[derive(Copy, Clone, Debug)]
pub(crate) enum VmirGenContext {
    Normal,
    NewVirtualStruct { output_stack_idx: VMStackIdx },
}
