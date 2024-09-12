use husky_rng_utils::XRng;

pub fn rnd_code(seed: u64) -> (Vec<String>, Vec<usize>) {
    let mut bcg = BasicCodeGenerator::new(seed);
    bcg.gen_fns(3);
    let (code, errors) = bcg.finish();
    todo!("code = {:?}, errors = {:?}", code, errors);
}

#[test]
fn rnd_code_works() {
    rnd_code(0);
}

struct BasicCodeGenerator {
    rng: XRng,
    functions: Vec<Function>,
    result: Vec<String>,
    errors: Vec<usize>,
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
            result: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn push_token(&mut self, token: impl Into<String>, has_ty_error: bool) {
        let position = self.result.len();
        self.result.push(token.into());
        if has_ty_error {
            self.errors.push(position);
        }
    }

    fn with_curly(&mut self, f: impl FnOnce(&mut Self)) {
        self.push_token("{", false);
        f(self);
        self.push_token("}", false);
    }

    fn gen_ty(&mut self) -> Type {
        let ty = match self.rng.randint(0..3) {
            0 => Type::Bool,
            1 => Type::Int,
            2 => Type::Float,
            _ => unreachable!(),
        };
        self.push_token(ty.repr(), false);
        ty
    }

    fn gen_fn(&mut self) {
        let len = self.functions.len();
        self.push_token("fn", false);
        self.push_token(format!("f{len}"), false);
        self.push_token("(", false);
        self.push_token("a", false);
        self.push_token(":", false);
        let input_ty = self.gen_ty();
        self.push_token(")", false);
        self.push_token("{", false);
        self.push_token("}", false);
        self.functions.push(Function { input_ty });
    }

    fn gen_fns(&mut self, n: usize) {
        for _ in 0..n {
            self.gen_fn()
        }
    }

    fn finish(self) -> (Vec<String>, Vec<usize>) {
        (self.result, self.errors)
    }
}
