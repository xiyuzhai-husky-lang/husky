use std::sync::Arc;

use arena::map::{ArenaKeyQuery, ArenaMap};
use ast::RawExprMap;
use infer_contract::ContractSheet;
use infer_error::{derived_not_none, InferError, InferErrorVariant};
use print_utils::{p, ps};
use std::fmt::Write;
use test_utils::{TestDisplay, TestDisplayConfig};
use text::{Row, TextRange};
use vec_map::VecPairMap;
use word::{CustomIdentifier, Identifier};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedTySheet {
    pub(crate) eager_variable_qualified_tys:
        VecPairMap<(CustomIdentifier, TextRange), InferResult<EagerVariableQualifiedTy>>,
    pub(crate) lazy_variable_qualified_tys:
        VecPairMap<(CustomIdentifier, TextRange), InferResult<LazyVariableQualifiedTy>>,
    pub(crate) eager_expr_qualified_tys: RawExprMap<InferResult<EagerValueQualifiedTy>>,
    pub(crate) lazy_expr_qualified_tys: RawExprMap<InferResult<LazyValueQualifiedTy>>,
    pub(crate) contract_sheet: Arc<ContractSheet>,
    pub(crate) extra_errors: Vec<InferError>,
}

impl QualifiedTySheet {
    pub fn new(contract_sheet: Arc<ContractSheet>) -> Self {
        let arena = &contract_sheet.entity_route_sheet.ast_text.arena;
        Self {
            eager_variable_qualified_tys: Default::default(),
            lazy_variable_qualified_tys: Default::default(),
            eager_expr_qualified_tys: ArenaMap::new(arena),
            lazy_expr_qualified_tys: ArenaMap::new(arena),
            contract_sheet,
            extra_errors: Vec::new(),
        }
    }

    pub fn lazy_expr_qualified_ty(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<LazyValueQualifiedTy> {
        match derived_not_none!(self.lazy_expr_qualified_tys.get(raw_expr_idx))? {
            Ok(qt) => Ok(*qt),
            Err(e) => Err(e.derived()),
        }
    }

    pub fn eager_expr_qualified_ty(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerValueQualifiedTy> {
        match derived_not_none!(self.eager_expr_qualified_tys.get(raw_expr_idx))? {
            Ok(qt) => Ok(*qt),
            Err(e) => Err(e.derived()),
        }
    }

    pub fn eager_variable_qualified_ty(
        &self,
        varname: CustomIdentifier,
        init_range: TextRange,
    ) -> InferResult<EagerVariableQualifiedTy> {
        match derived_not_none!(self
            .eager_variable_qualified_tys
            .get_entry((varname, init_range)))?
        .1
        {
            Ok(qt) => Ok(qt),
            Err(ref e) => Err(e.derived()),
        }
    }

    pub fn lazy_variable_qualified_ty(
        &self,
        varname: CustomIdentifier,
        init_range: TextRange,
    ) -> InferResult<LazyVariableQualifiedTy> {
        match derived_not_none!(self
            .lazy_variable_qualified_tys
            .get_entry((varname, init_range)))?
        .1
        {
            Ok(qt) => Ok(qt),
            Err(ref e) => Err(e.derived()),
        }
    }

    pub fn original_errors(&self) -> Vec<&InferError> {
        let mut errors = Vec::new();
        for (_, result) in self.eager_expr_qualified_tys.iter() {
            match result {
                Ok(_) => (),
                Err(e) => match e.variant {
                    InferErrorVariant::Derived { .. } => (),
                    InferErrorVariant::Original { .. } => errors.push(e),
                },
            }
        }
        for (_, result) in self.lazy_expr_qualified_tys.iter() {
            match result {
                Ok(_) => (),
                Err(e) => match e.variant {
                    InferErrorVariant::Derived { .. } => (),
                    InferErrorVariant::Original { .. } => errors.push(e),
                },
            }
        }
        for result in self.eager_expr_qualified_tys.values() {
            match result {
                Ok(_) => (),
                Err(e) => match e.variant {
                    InferErrorVariant::Derived { .. } => (),
                    InferErrorVariant::Original { .. } => errors.push(e),
                },
            }
        }
        for result in self.lazy_expr_qualified_tys.values() {
            match result {
                Ok(_) => (),
                Err(e) => match e.variant {
                    InferErrorVariant::Derived { .. } => (),
                    InferErrorVariant::Original { .. } => errors.push(e),
                },
            }
        }
        for e in &self.extra_errors {
            match e.variant {
                InferErrorVariant::Derived { .. } => (),
                InferErrorVariant::Original { .. } => errors.push(e),
            }
        }
        errors
    }
}

macro_rules! write_field_name {
    ($result: expr, $name: expr, $config: expr) => {
        if $config.colored {
            write!(
                $result,
                "\n{}{}:\n{}",
                print_utils::MAGENTA,
                $name,
                print_utils::RESET
            )
            .unwrap()
        } else {
            write!($result, "\n{}:\n", $name).unwrap()
        }
    };
}

impl TestDisplay for QualifiedTySheet {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        let ast_text = &self.contract_sheet.entity_route_sheet.ast_text;
        write_field_name!(result, "eager variable", config);
        self.eager_variable_qualified_tys
            .write_inherent(config.indented(), result);
        write_field_name!(result, "lazy expr", config);
        ast_text.write_map_inherently(&self.lazy_expr_qualified_tys, config.indented(), result);
        write_field_name!(result, "eager expr", config);
        ast_text.write_map_inherently(&self.eager_expr_qualified_tys, config.indented(), result);
        write_field_name!(result, "lazy expr", config);
        ast_text.write_map_inherently(&self.lazy_expr_qualified_tys, config.indented(), result);
    }
}
