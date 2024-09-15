use husky_rng_utils::XRng;
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};

pub fn rnd_codes(n: u64, max_fns: usize, error_rate: f64) -> Vec<(Vec<String>, Vec<TokenInfo>)> {
    let mut data = Vec::new();

    for seed in 0..n {
        data.push(rnd_code(seed, error_rate, max_fns));
    }
    data
}

#[derive(Serialize, Clone, Copy)]
pub struct TokenInfo {
    #[serde(serialize_with = "serialize_option_ast_kind")]
    ast_kind: Option<AstKind>,
    #[serde(serialize_with = "serialize_option_symbol_resolution")]
    symbol_resolution: Option<SymbolResolution>,
    #[serde(serialize_with = "serialize_option_type_error")]
    error: Option<TypeError>,
}

#[derive(Serialize, Clone, Copy)]
pub enum AstKind {
    FnEntityName = 1,
    ParameterType = 2,
    ParameterIdent = 3,
    FnEntityUsage = 4,
    CallLpar = 5,
}

#[derive(Serialize, Clone, Copy)]
pub enum SymbolResolution {
    Fn = 1,
    Unresolved = 2,
}

#[derive(Serialize, Clone, Copy)]
pub enum TypeError {
    Expected = 1,
}

pub fn rnd_code(seed: u64, error_rate: f64, max_fns: usize) -> (Vec<String>, Vec<TokenInfo>) {
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
    token_infos: Vec<TokenInfo>,
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
            token_infos: Vec::new(),
            error_rate,
        }
    }

    fn push_token(
        &mut self,
        token: impl Into<String>,
        ast_kind: Option<AstKind>,
        symbol_resolution: Option<SymbolResolution>,
        error: Option<TypeError>,
    ) {
        self.result.push(token.into());
        self.token_infos.push(TokenInfo {
            ast_kind,
            symbol_resolution,
            error,
        });
    }

    fn with_curly(&mut self, f: impl FnOnce(&mut Self)) {
        self.push_token("{", None, None, None);
        f(self);
        self.push_token("}", None, None, None);
    }

    fn gen_ty(&mut self, ast_kind: AstKind) -> Type {
        let ty = match self.rng.rand_range(0..3) {
            0 => Type::Bool,
            1 => Type::Int,
            2 => Type::Float,
            _ => unreachable!(),
        };
        self.push_token(ty.repr(), Some(ast_kind), None, None);
        ty
    }

    fn gen_fn(&mut self) {
        let len = self.functions.len();
        self.push_token("fn", None, None, None);
        self.push_token(
            format!("f{len}"),
            Some(AstKind::FnEntityName),
            Some(SymbolResolution::Fn),
            None,
        );
        self.push_token("(", None, None, None);
        self.push_token("a", Some(AstKind::ParameterIdent), None, None);
        self.push_token(":", None, None, None);
        let input_ty = self.gen_ty(AstKind::ParameterType);
        self.push_token(")", None, None, None);
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

                gen.push_token(
                    format!("f{callee_index}"),
                    None,
                    Some(SymbolResolution::Fn),
                    None,
                );
                gen.push_token("(", None, None, None);
                gen.push_token(
                    arg_literal,
                    None,
                    None,
                    if has_ty_error {
                        Some(TypeError::Expected)
                    } else {
                        None
                    },
                );
                gen.push_token(")", None, None, None); // Changed from Some(AstKind::Other)
                gen.push_token(";", None, None, None); // Changed from Some(AstKind::Other)
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

    fn finish(self) -> (Vec<String>, Vec<TokenInfo>) {
        (self.result, self.token_infos)
    }
}

fn serialize_option_ast_kind<S>(value: &Option<AstKind>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        None => serializer.serialize_u8(0),
        Some(kind) => serializer.serialize_u8(*kind as u8),
    }
}

fn serialize_option_symbol_resolution<S>(
    value: &Option<SymbolResolution>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        None => serializer.serialize_u8(0),
        Some(resolution) => serializer.serialize_u8(*resolution as u8),
    }
}

fn serialize_option_type_error<S>(
    value: &Option<TypeError>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        None => serializer.serialize_u8(0),
        Some(TypeError::Expected) => serializer.serialize_u8(1),
    }
}
