use husky_rng_utils::XRng;

pub fn rnd_code(seed: u64) -> String {
    let mut bcg = BasicCodeGenerator::new(seed);
    bcg.gen_fns(3);
    let code = bcg.finish();
    todo!("code = {code}")
}

#[test]
fn rnd_code_works() {
    rnd_code(0);
}

struct BasicCodeGenerator {
    rng: XRng,
    functions: Vec<Function>,
    result: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Bool,
    Int,
    Float,
}

impl Type {
    fn repr(self) -> &'static str {
        match self {
            Type::Bool => "Bool",
            Type::Int => "Int",
            Type::Float => "Float",
        }
    }
}

struct Function {
    input_ty: Type,
}

impl BasicCodeGenerator {
    fn new(seed: u64) -> Self {
        Self {
            rng: XRng::new(seed),
            functions: Default::default(),
            result: Default::default(),
        }
    }
}

impl BasicCodeGenerator {
    fn with_curly(&mut self, f: impl FnOnce(&mut Self)) {
        self.result += "{ ";
        f(self);
        self.result += " }";
    }

    fn gen_ty(&mut self) -> Type {
        let ty = match self.rng.randint(0..3) {
            0 => Type::Bool,
            1 => Type::Int,
            2 => Type::Float,
            _ => unreachable!(),
        };
        self.result += ty.repr();
        ty
    }

    fn gen_fn(&mut self) {
        let len = self.functions.len();
        self.result += &format!("fn f{len}(a: ");
        let input_ty = self.gen_ty();
        self.result += &format!(") {{}}");
        self.functions.push(Function { input_ty });
    }

    fn gen_fns(&mut self, n: usize) {
        for _ in 0..n {
            self.gen_fn()
        }
    }

    fn finish(self) -> String {
        self.result
    }
}
