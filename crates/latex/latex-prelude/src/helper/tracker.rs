use sealed::*;

#[sealed]
pub trait IsLxTrackerInput {}

pub struct LxDocumentTrackerInput;

#[sealed]
impl IsLxTrackerInput for LxDocumentTrackerInput {}

pub struct LxDocumentBodyTrackerInput;

#[sealed]
impl IsLxTrackerInput for LxDocumentBodyTrackerInput {}

pub struct LxFormulaTrackerInput;

#[sealed]
impl IsLxTrackerInput for LxFormulaTrackerInput {}
