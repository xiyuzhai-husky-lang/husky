use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StackLocationIdx(ShiftedU32);

#[derive(Default)]
pub struct StackLocationRegistry {
    next: ShiftedU32,
}

impl StackLocationRegistry {
    pub fn issue_new(&mut self) -> StackLocationIdx {
        let next = self.next;
        self.next += 1;
        StackLocationIdx(next)
    }
}
