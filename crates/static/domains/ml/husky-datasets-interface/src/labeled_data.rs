use crate::*;

pub struct LabeledData<'eval> {
    pub sample_id: SampleId,
    pub input: __Register<'eval>,
    pub label: Label,
}
