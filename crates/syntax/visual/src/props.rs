use vm::PrimitiveValue;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VisualProps {
    BinaryImage28 { padded_rows: [u32; 30] },
    Primitive { value: PrimitiveValue },
}
