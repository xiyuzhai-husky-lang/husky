use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_lib_rs_content(&mut self) {
        let package = self.package();
        self.write("#![allow(warnings)]\n");
        self.write("pub mod __init__;\n");
        self.write("use __husky_root::*;\n\n");
        self.write(
            r#"
// ad hoc
fn __input<'a, 'eval:'a>(__ctx: &'a __EvalContext<'eval>) -> &'a domains::ml::datasets::cv::mnist::BinaryImage28 {
    unsafe { __evaluator(__ctx) }
        .eval_input
        .any_ref()
        .__downcast_ref()
}

"#,
        );
        self.gen_mod_rs_content(&package.subentities);
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }
}
