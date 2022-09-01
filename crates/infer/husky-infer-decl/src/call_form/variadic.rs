pub use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum VariadicParametersDecl {
    None,
    RepeatSingle { parameter: ParameterDecl },
}

impl Default for VariadicParametersDecl {
    fn default() -> Self {
        VariadicParametersDecl::None
    }
}

impl VariadicParametersDecl {
    pub(crate) fn from_static(
        db: &dyn DeclQueryGroup,
        ctx: &mut dyn AtomContext,
        static_defn: &StaticVariadicParameterDecl,
    ) -> InferResult<Self> {
        Ok(match static_defn {
            StaticVariadicParameterDecl::None => VariadicParametersDecl::None,
            StaticVariadicParameterDecl::RepeatSingle(parameter_decl) => {
                VariadicParametersDecl::RepeatSingle {
                    parameter: ParameterDecl::from_static(db, ctx, parameter_decl)?,
                }
            }
        })
    }

    pub fn is_some(&self) -> bool {
        match self {
            VariadicParametersDecl::None => false,
            VariadicParametersDecl::RepeatSingle { .. } => true,
        }
    }
}

impl Instantiable for VariadicParametersDecl {
    type Target = Self;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        match self {
            VariadicParametersDecl::None => VariadicParametersDecl::None,
            VariadicParametersDecl::RepeatSingle {
                parameter: parameter_decl,
            } => VariadicParametersDecl::RepeatSingle {
                parameter: parameter_decl.instantiate(ctx),
            },
        }
    }
}

impl Implementable for VariadicParametersDecl {
    type Target = Self;

    fn implement(&self, ctx: &ImplementationContext) -> Self::Target {
        match self {
            VariadicParametersDecl::None => VariadicParametersDecl::None,
            VariadicParametersDecl::RepeatSingle {
                parameter: parameter_decl,
            } => VariadicParametersDecl::RepeatSingle {
                parameter: parameter_decl.implement(ctx),
            },
        }
    }
}
