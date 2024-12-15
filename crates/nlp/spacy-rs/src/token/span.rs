use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    start: usize,
    end: usize,
    root: usize,
}

impl<'py> FromPyObject<'py> for Span {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let start = ob.getattr("start")?.extract()?;
        let end = ob.getattr("end")?.extract()?;
        let root = ob.getattr("root")?.getattr("i")?.extract()?;
        Ok(Self { start, end, root })
    }
}
