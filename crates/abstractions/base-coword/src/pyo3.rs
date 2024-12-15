use super::*;
use ::pyo3::prelude::*;
use eterned::attach::with_attached_eterner_db;

impl<'py> FromPyObject<'py> for BaseCoword {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        with_attached_eterner_db(|db| {
            let s: &str = ob.extract()?;
            Ok(BaseCoword::from_ref(s, db))
        })
        .expect("no database attached")
    }
}
