use husky_rng_utils::XRng;

pub fn rnd_codes(n: u64, max_fns: usize, error_rate: f64) -> String {
    let mut combined_result = String::new();

    for seed in 0..n {
        let (code, errors) = rnd_code(seed, error_rate, max_fns);

        let code_string = code.join(" ");
        let errors_string = errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        combined_result.push_str(&format!("{}\n{}\n", code_string, errors_string));
    }

    combined_result
}

pub fn rnd_code(seed: u64, error_rate: f64, max_fns: usize) -> (Vec<String>, Vec<usize>) {
    let mut bcg = BasicCodeGenerator::new(seed, error_rate);
    bcg.gen_fns(max_fns);
    bcg.finish()
}

#[test]
fn rnd_code_works() {
    rnd_code(0, 0.1, 5);
}

struct BasicCodeGenerator {
    rng: XRng,
    functions: Vec<Function>,
    result: Vec<String>,
    errors: Vec<usize>,
    error_rate: f64,
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

    fn random_literal(self) -> &'static str {
        match self {
            Type::Bool => "true",
            Type::Int => "1",
            Type::Float => "1.1",
        }
    }
}

struct Function {
    input_ty: Type,
}

impl BasicCodeGenerator {
    fn new(seed: u64, error_rate: f64) -> Self {
        Self {
            rng: XRng::new(seed),
            functions: Default::default(),
            result: Vec::new(),
            errors: Vec::new(),
            error_rate,
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
        let ty = match self.rng.rand_range(0..3) {
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
        self.with_curly(|gen| {
            if len > 0 {
                // Generate a call to a previously defined function
                let callee_index = gen.rng.rand_range(0..len);
                let callee = &gen.functions[callee_index];

                // Randomly decide if the call should have a type error
                let has_ty_error = gen.rng.randf64() < gen.error_rate;
                let arg_literal = if has_ty_error {
                    // Pick a literal with a different type
                    match callee.input_ty {
                        Type::Bool => "1",     // Int instead of Bool
                        Type::Int => "1.1",    // Float instead of Int
                        Type::Float => "true", // Bool instead of Float
                    }
                } else {
                    callee.input_ty.random_literal()
                };

                gen.push_token(format!("f{callee_index}"), false);
                gen.push_token("(", false);
                gen.push_token(arg_literal, has_ty_error);
                gen.push_token(")", false);
                gen.push_token(";", false);
            }
        });
        self.functions.push(Function { input_ty });
    }

    fn gen_fns(&mut self, max_fns: usize) {
        let num_fns = self.rng.rand_range(3..=max_fns.max(3));
        for _ in 0..num_fns {
            self.gen_fn()
        }
    }

    fn finish(self) -> (Vec<String>, Vec<usize>) {
        (self.result, self.errors)
    }
}
