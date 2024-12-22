use pyo3::{ffi::c_str, prelude::*};
use std::{ffi::CStr, sync::Mutex};

const PYTHON_CODE: &CStr = c_str!(
    r#"
import sglang as sgl
import os

llm = sgl.Engine(model_path=os.path.expandvars("$HOME/.llms/models/Qwen2-7B-Instruct"))
sampling_params = {"temperature": 0.8, "top_p": 0.95}

def generate_text_batch(prompts):
    return llm.generate(prompts, sampling_params)
"#
);

struct SglangModule {
    module: Py<PyModule>,
}

impl SglangModule {
    fn new<'py>(py: Python<'py>) -> Self {
        Self {
            module: PyModule::from_code(
                py,
                PYTHON_CODE,
                c_str!("sglang_wrapper.py"),
                c_str!("sglang_wrapper"),
            )
            .unwrap()
            .unbind(),
        }
    }
}

impl SglangModule {
    pub fn generate_text_batch<'py>(&self, py: Python<'py>, prompts: Vec<String>) -> Vec<String> {
        let module = &*self.module.bind(py);
        module
            .getattr("generate_text_batch")
            .unwrap()
            .call1((prompts,))
            .unwrap()
            .extract()
            .unwrap()
    }
}

lazy_static::lazy_static! {
    static ref SGLANG_MODULE: SglangModule = Python::with_gil(|py| SglangModule::new(py));
}

pub fn generate_text_batch(prompts: Vec<String>) -> Vec<String> {
    Python::with_gil(|py| SGLANG_MODULE.generate_text_batch(py, prompts))
}
