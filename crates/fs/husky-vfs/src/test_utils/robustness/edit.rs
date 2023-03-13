#[derive(Debug)]
pub enum VfsEdit {
    InsertChar { position: usize, c: char },
    Slice(std::ops::Range<usize>),
}
