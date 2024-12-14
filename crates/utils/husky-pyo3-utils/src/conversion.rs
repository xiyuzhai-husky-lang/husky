use pyo3::PyResult;

pub trait FromPyObjectBoundX<'a, 'py> {}

pub trait PyAnyMethodsX<'py> {
    fn extract<'a, T>(&self) -> PyResult<T>
    where
        T: FromPyObjectBoundX<'a, 'py>;
}
