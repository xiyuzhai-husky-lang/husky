use shifted_unsigned_int::ShiftedU32;

pub struct FileId(ShiftedU32);
impl FileId {
    pub(crate) fn new(file: u32) -> Self {
        Self(file.into())
    }
}
