#[const_trait]
pub trait ProvideTypeContext {
    #[inline(always)]
    fn ty_context(&self) -> TypeContext;
}

pub struct TypeContext {
    expect_applicable_or_callable: bool,
}

impl const Default for TypeContext {
    fn default() -> Self {
        Self {
            expect_applicable_or_callable: false,
        }
    }
}

impl TypeContext {
    #[inline(always)]
    pub const fn new_expect_applicable_or_callable() -> Self {
        Self {
            expect_applicable_or_callable: true,
        }
    }

    pub(crate) fn expect_applicable_or_callable(&self) -> bool {
        self.expect_applicable_or_callable
    }
}
