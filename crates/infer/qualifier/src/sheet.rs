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
use word::Identifier;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedTySheet {
    pub(crate) eager_variable_qualified_tys:
        VecPairMap<(Identifier, TextRange), InferResult<EagerQualifiedTy>>,
    pub(crate) lazy_variable_qualified_tys:
        VecPairMap<(Identifier, TextRange), InferResult<LazyQualifiedTy>>,
    pub(crate) eager_expr_qualified_tys: RawExprMap<InferResult<EagerQualifiedTy>>,
    pub(crate) lazy_expr_qualified_tys: RawExprMap<InferResult<LazyQualifiedTy>>,
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

    pub fn lazy_expr_qualified_ty(&self, raw_expr_idx: RawExprIdx) -> InferResult<LazyQualifiedTy> {
        match derived_not_none!(self.lazy_expr_qualified_tys.get(raw_expr_idx))? {
            Ok(qt) => Ok(*qt),
            Err(e) => Err(e.derived()),
        }
    }

    pub fn eager_expr_qualified_ty(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerQualifiedTy> {
        match derived_not_none!(self.eager_expr_qualified_tys.get(raw_expr_idx))? {
            Ok(qt) => Ok(*qt),
            Err(e) => Err(e.derived()),
        }
    }

    pub fn eager_variable_qualified_ty(
        &self,
        varname: Identifier,
        init_range: TextRange,
    ) -> InferResult<EagerQualifiedTy> {
        match derived_not_none!(self
            .eager_variable_qualified_tys
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
    ($result: expr, $name: expr) => {
        write!(
            $result,
            "\n{}{}\n{}",
            print_utils::MAGENTA,
            $name,
            print_utils::RESET
        )
        .unwrap()
    };
}

impl TestDisplay for QualifiedTySheet {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        let ast_text = &self.contract_sheet.entity_route_sheet.ast_text;
        write_field_name!(result, "eager variable qualified types");
        self.eager_variable_qualified_tys
            .write_inherent(config.indented(), result);
        write_field_name!(result, "lazy expr qualified types");
        ast_text.write_map_inherently(&self.lazy_expr_qualified_tys, config.indented(), result);
        write_field_name!(result, "eager expr qualified types");
        ast_text.write_map_inherently(&self.eager_expr_qualified_tys, config.indented(), result);
        write_field_name!(result, "lazy expr qualified types");
        ast_text.write_map_inherently(&self.lazy_expr_qualified_tys, config.indented(), result);
    }
    // fn print_inherent(&self) -> String {
    //     let mut result = String::new();
    //     result.push_str("eager variable qualified types:\n\n");
    //     for ((ident, row), qt_result) in self.eager_variable_qualified_tys.iter() {
    //         write!(
    //             result,
    //             "    {: <4} {: <20}{:?}\n",
    //             row.0,
    //             ident.as_str(),
    //             qt_result
    //         )
    //         .unwrap()
    //     }
    //     println!("{}", &result);
    //     todo!()
    // }
}
