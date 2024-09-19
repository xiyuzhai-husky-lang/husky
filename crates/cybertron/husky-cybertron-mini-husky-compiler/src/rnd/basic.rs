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

#[derive(Serialize, Clone, Copy, Debug)]
pub struct TokenInfo {
    #[serde(serialize_with = "serialize_option_ast_kind")]
    ast_kind: Option<AstKind>,
    #[serde(serialize_with = "serialize_option_symbol_resolution")]
    symbol_resolution: Option<SymbolResolution>,
    #[serde(serialize_with = "serialize_option_type_error")]
    error: Option<TypeError>,
}

#[derive(Serialize, Clone, Copy, Debug)]
pub enum AstKind {
    FnEntityName = 1,
    ParameterType = 2,
    ParameterIdent = 3,
    FnEntityUsage = 4,
    CallLpar = 5,
    FnKeyword = 6,
    LetKeyword = 7,
    CallRpar = 8,
    IntLiteral = 9,
    FloatLiteral = 10,
    BoolLiteral = 11,
    ParametersLpar = 12,
    ParametersRpar = 13,
    FnBodyLcurl = 14,
    FnBodyRcurl = 15,
    ParameterTypeColon = 16,
    StmtColon = 17,
}

#[derive(Serialize, Clone, Copy, Debug)]
pub enum SymbolResolution {
    Fn = 1,
    Unresolved = 2,
}

#[derive(Serialize, Clone, Copy, Debug)]
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
    use expect_test::{expect, Expect};

    fn t(seed: u64, error_rate: f64, max_fns: usize, expect: Expect) {
        let (tokens, token_infos) = rnd_code(seed, error_rate, max_fns);
        let tokens_str = tokens.join(" ");
        let token_infos_str = token_infos
            .iter()
            .map(|info| format!("{:?}", info))
            .collect::<Vec<_>>()
            .join("\n");
        let result = format!(
            "Tokens:\n{}\n\nToken Infos:\n{}",
            tokens_str, token_infos_str
        );
        expect.assert_eq(&result);
    }

    t(0, 0.1, 5, expect![[r#"
        Tokens:
        fn f0 ( a : Int ) { } fn f1 ( a : Float ) { f0 ( 1 ) ; } fn f2 ( a : Bool ) { f1 ( 1.1 ) ; f0 ( 1 ) ; f0 ( 1 ) ; f1 ( 1.1 ) ; } fn f3 ( a : Bool ) { f0 ( 1 ) ; f1 ( 1.1 ) ; f2 ( 1 ) ; f0 ( 1 ) ; f2 ( 1 ) ; } fn f4 ( a : Float ) { f1 ( 1.1 ) ; f3 ( true ) ; f3 ( true ) ; f2 ( true ) ; f0 ( 1 ) ; }

        Token Infos:
        TokenInfo { ast_kind: Some(FnKeyword), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityName), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(ParametersLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterIdent), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterTypeColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterType), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParametersRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnBodyLcurl), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnBodyRcurl), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnKeyword), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityName), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(ParametersLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterIdent), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterTypeColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterType), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParametersRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnBodyLcurl), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(IntLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnBodyRcurl), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnKeyword), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityName), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(ParametersLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterIdent), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterTypeColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterType), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParametersRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnBodyLcurl), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FloatLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(IntLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(IntLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FloatLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnBodyRcurl), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnKeyword), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityName), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(ParametersLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterIdent), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterTypeColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterType), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParametersRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnBodyLcurl), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(IntLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FloatLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(IntLiteral), symbol_resolution: None, error: Some(Expected) }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(IntLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(IntLiteral), symbol_resolution: None, error: Some(Expected) }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnBodyRcurl), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnKeyword), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityName), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(ParametersLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterIdent), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterTypeColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParameterType), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(ParametersRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnBodyLcurl), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FloatLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(BoolLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(BoolLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(BoolLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnEntityUsage), symbol_resolution: Some(Fn), error: None }
        TokenInfo { ast_kind: Some(CallLpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(IntLiteral), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(CallRpar), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(StmtColon), symbol_resolution: None, error: None }
        TokenInfo { ast_kind: Some(FnBodyRcurl), symbol_resolution: None, error: None }"#]]);
}

struct BasicCodeGenerator {
    rng: XRng,
    functions: Vec<Function>,
    result: Vec<String>,
    token_infos: Vec<TokenInfo>,
    error_rate: f64,
    max_calls_per_fn: usize,
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
            max_calls_per_fn: 5, // default value
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

    fn with_curly(&mut self, lcurl_kind: AstKind, rcurl_kind: AstKind, f: impl FnOnce(&mut Self)) {
        self.push_token("{", Some(lcurl_kind), None, None);
        f(self);
        self.push_token("}", Some(rcurl_kind), None, None);
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
        self.push_token("fn", Some(AstKind::FnKeyword), None, None);
        self.push_token(
            format!("f{len}"),
            Some(AstKind::FnEntityName),
            Some(SymbolResolution::Fn),
            None,
        );
        self.push_token("(", Some(AstKind::ParametersLpar), None, None);
        self.push_token("a", Some(AstKind::ParameterIdent), None, None);
        self.push_token(":", Some(AstKind::ParameterTypeColon), None, None);
        let input_ty = self.gen_ty(AstKind::ParameterType);
        self.push_token(")", Some(AstKind::ParametersRpar), None, None);
        self.with_curly(AstKind::FnBodyLcurl, AstKind::FnBodyRcurl, |gen| {
            if len > 0 {
                let num_calls = gen.rng.rand_range(1..=gen.max_calls_per_fn);
                for _ in 0..num_calls {
                    gen.gen_fn_call(len);
                }
            }
        });
        self.functions.push(Function { input_ty });
    }

    fn gen_fn_call(&mut self, len: usize) {
        let callee_index = self.rng.rand_range(0..len);
        let callee = &self.functions[callee_index];

        let has_ty_error = self.rng.randf64() < self.error_rate;
        let (arg_literal, literal_kind) = if has_ty_error {
            match callee.input_ty {
                Type::Bool => ("1", AstKind::IntLiteral),
                Type::Int => ("1.1", AstKind::FloatLiteral),
                Type::Float => ("true", AstKind::BoolLiteral),
            }
        } else {
            match callee.input_ty {
                Type::Bool => ("true", AstKind::BoolLiteral),
                Type::Int => ("1", AstKind::IntLiteral),
                Type::Float => ("1.1", AstKind::FloatLiteral),
            }
        };

        self.push_token(
            format!("f{callee_index}"),
            Some(AstKind::FnEntityUsage),
            Some(SymbolResolution::Fn),
            None,
        );
        self.push_token("(", Some(AstKind::CallLpar), None, None);
        self.push_token(
            arg_literal,
            Some(literal_kind),
            None,
            if has_ty_error {
                Some(TypeError::Expected)
            } else {
                None
            },
        );
        self.push_token(")", Some(AstKind::CallRpar), None, None);
        self.push_token(";", Some(AstKind::StmtColon), None, None);
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
