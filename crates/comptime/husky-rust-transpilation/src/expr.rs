mod closure;
mod html;
pub(crate) mod pattern;
pub(crate) mod precedence;
pub(crate) mod role;
mod stmt;

pub(crate) use self::precedence::{RustPrecedence, RustPrecedenceRange};

use self::role::HirEagerExprRole;
use crate::{binding::RustBinding, *};
use binding::RustBindings;
use either::*;
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::{
    assoc_item::AssocItemPath,
    major_item::{ty::PreludeTypePath, MajorItemPath},
    ItemPath, PrincipalEntityPath,
};
use husky_hir_eager_expr::{
    coercion::HirEagerCoercion, emit_note_on_hir_eager_expr_codespan,
    variable::runtime::HirEagerRuntimeVariableName, HirEagerCondition, HirEagerElifBranch,
    HirEagerElseBranch, HirEagerExprData, HirEagerExprEntry, HirEagerExprIdx, HirEagerExprRegion,
    HirEagerIfBranch, HirEagerPatternData, HirEagerPatternIdx, HirEagerRitchieArgument,
    HirEagerStmtData, HirEagerStmtIdx, HirEagerStmtIdxRange,
};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    instantiation::HirTermSymbolicVariableResolution, place_contract_site::HirPlaceContractSite,
    quary::HirQuary, ritchie::HirContract, HirTemplateVariable, HirTemplateVariableClass, HirType,
};
use husky_opr::BinaryClosedOpr;
use salsa::DebugWithDb;
use smallvec::SmallVec;

impl<'db> TranspileToRustWith<HirEagerExprRegion> for (HirEagerExprIdx, HirEagerExprRole<'db>) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let (expr, role) = self;
        let expr_entry = expr.entry(builder.hir_eager_expr_arena());
        let data = expr_entry.data();
        let place_contract_site = expr_entry.place_contract_site();
        let db = builder.db;
        let innermost_precedence = RustPrecedence::from_expr(data);
        let bindings = RustBindings::new(expr_entry, role);
        let innermost_precedence_range = RustPrecedenceRange::innermost(&bindings, role);
        let outermost_precedence_range = RustPrecedenceRange::outermost(role);
        let needs_outermost_extra_pars = bindings
            .outermost()
            .map(|binding| match binding {
                RustBinding::Deref
                | RustBinding::DerefMut
                | RustBinding::DerefCustomed
                | RustBinding::Reref
                | RustBinding::RerefMut => {
                    !outermost_precedence_range.include(RustPrecedence::Prefix)
                }
                RustBinding::Deleash
                | RustBinding::Releash
                | RustBinding::SelfValue
                | RustBinding::WrapInSome => false,
            })
            .unwrap_or(false);
        if needs_outermost_extra_pars {
            builder.lpar();
        }
        builder.transpile_bindings(bindings.clone(), |builder| {
            if !innermost_precedence_range.include(innermost_precedence) {
                builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                    transpile_hir_eager_expr_to_rust(
                        data,
                        place_contract_site,
                        innermost_precedence,
                        builder,
                    )
                })
            } else {
                transpile_hir_eager_expr_to_rust(
                    data,
                    place_contract_site,
                    innermost_precedence,
                    builder,
                )
            }
        });
        if needs_outermost_extra_pars {
            builder.rpar();
        }
    }
}

// `precedence` is guaranteed to equal `hir_eager_expr_precedence(data)`
// passed to avoid recomputing it
fn transpile_hir_eager_expr_to_rust(
    data: &HirEagerExprData,
    place_contract_site: &HirPlaceContractSite,
    precedence: RustPrecedence,
    builder: &mut RustTranspilationBuilder<HirEagerExprRegion>,
) {
    let subexpr_geq = || HirEagerExprRole::subexpr(RustPrecedenceRange::Geq(precedence));
    let subexpr_greater = || HirEagerExprRole::subexpr(RustPrecedenceRange::Greater(precedence));
    let db = builder.db();
    match *data {
        HirEagerExprData::Literal(term_literal) => term_literal.transpile_to_rust(builder),
        HirEagerExprData::PrincipalEntityPath(principal_entity_path) => {
            principal_entity_path.transpile_to_rust(builder);
            match principal_entity_path {
                PrincipalEntityPath::Module(_) => unreachable!(),
                PrincipalEntityPath::MajorItem(MajorItemPath::Form(path)) => match path.kind(db) {
                    MajorFormKind::Ritchie(_) => (),
                    MajorFormKind::TypeAlias => (),
                    MajorFormKind::TypeVar => (),
                    MajorFormKind::Val | MajorFormKind::StaticVar => {
                        builder.delimited(RustDelimiter::Par, |_| ())
                    }
                    MajorFormKind::StaticMut => todo!(),
                    MajorFormKind::Compterm => (),
                    MajorFormKind::Conceptual => todo!(),
                },
                PrincipalEntityPath::TypeVariant(_) => (),
                PrincipalEntityPath::MajorItem(_) => (),
            }
        }
        HirEagerExprData::ComptimeVariable { ident, .. } => ident.transpile_to_rust(builder),
        HirEagerExprData::RuntimeVariable(variable) => variable.transpile_to_rust(builder),
        HirEagerExprData::Binary { lopd, opr, ropd } => match opr {
            HirBinaryOpr::Closed(BinaryClosedOpr::RemEuclid) => {
                (lopd, HirEagerExprRole::simple_self_argument()).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                builder.rem_eulid();
                builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                    (ropd, HirEagerExprRole::subexpr_with_any_precedence_range())
                        .transpile_to_rust(builder)
                })
            }
            HirBinaryOpr::Closed(BinaryClosedOpr::Power) => {
                // ad hoc
                (lopd, HirEagerExprRole::simple_self_argument()).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                builder.pow();
                builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                    (ropd, HirEagerExprRole::subexpr_with_any_precedence_range())
                        .transpile_to_rust(builder)
                })
            }
            HirBinaryOpr::Assign | HirBinaryOpr::AssignClosed(_) | HirBinaryOpr::AssignShift(_) => {
                (lopd, HirEagerExprRole::assign_self_argument()).transpile_to_rust(builder);
                opr.transpile_to_rust(builder);
                (ropd, subexpr_greater()).transpile_to_rust(builder)
            }
            _ => {
                (lopd, subexpr_geq()).transpile_to_rust(builder);
                opr.transpile_to_rust(builder);
                (ropd, subexpr_greater()).transpile_to_rust(builder)
            }
        },
        HirEagerExprData::Be { src: _, pattern: _ } => builder.macro_name(RustMacroName::Matches),
        HirEagerExprData::Prefix { opr, opd } => {
            match opr {
                HirPrefixOpr::NotInt => builder.delimited(RustDelimiter::Par, |builder| {
                    (
                        opd,
                        HirEagerExprRole::subexpr(RustPrecedenceRange::Geq(
                            RustPrecedence::EqComparison,
                        )),
                    )
                        .transpile_to_rust(builder);
                    builder.eq_zero()
                }),
                _ => {
                    // todo: check some details
                    opr.transpile_to_rust(builder);
                    (opd, subexpr_geq()).transpile_to_rust(builder)
                }
            }
        }
        HirEagerExprData::Suffix { opd, opr } => match opr {
            HirSuffixOpr::Incr | HirSuffixOpr::Decr => {
                (opd, HirEagerExprRole::assign_self_argument()).transpile_to_rust(builder);
                opr.transpile_to_rust(builder)
            }
        },
        HirEagerExprData::Unveil {
            opd,
            return_ty,
            ref instantiation,
            ..
        } => {
            builder.macro_name(RustMacroName::Unveil);
            builder.delimited(RustDelimiter::Par, |builder| {
                return_ty.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                (opd, subexpr_geq()).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                let runtime_constants: SmallVec<[HirTermSymbolicVariableResolution; 2]> =
                    instantiation
                        .symbol_map()
                        .iter()
                        .filter_map(|&(symbol, resolution)| match symbol {
                            HirTemplateVariable::Compterm(symbol) => (symbol.index(db).class()
                                == HirTemplateVariableClass::Poly)
                                .then_some(resolution),
                            _ => None,
                        })
                        .collect();
                builder.delimited_comma_list_with_last_comma(RustDelimiter::Par, runtime_constants)
            })
        }
        HirEagerExprData::Unwrap { opd } => {
            (opd, HirEagerExprRole::simple_self_argument()).transpile_to_rust(builder);
            builder.call_unwrap()
        }
        HirEagerExprData::TypeConstructorCall {
            path,
            instantiation: _,
            ref arguments,
        } => {
            builder.struct_ty_constructor_path(path);
            builder.delimited_comma_list(RustDelimiter::Par, arguments)
        }
        HirEagerExprData::TypeVariantConstructorCall {
            path,
            instantiation: _,
            ref arguments,
        } => {
            path.transpile_to_rust(builder);
            builder.delimited_comma_list(RustDelimiter::Par, arguments)
        }
        HirEagerExprData::FunctionRitchieCall {
            path,
            instantiation: _,
            ref arguments,
        } => {
            path.transpile_to_rust(builder);
            builder.delimited_comma_list(RustDelimiter::Par, arguments)
        }
        HirEagerExprData::AssocFunctionRitchieCall {
            path,
            ref arguments,
            ..
        } => {
            path.transpile_to_rust(builder);
            builder.delimited_comma_list(RustDelimiter::Par, arguments)
        }
        HirEagerExprData::PropsStructField {
            self_argument,
            self_ty,
            ident,
            ref indirections,
            ..
        } => {
            (
                self_argument,
                HirEagerExprRole::self_argument_with_indirections(indirections),
            )
                .transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            ident.transpile_to_rust(builder)
        }
        HirEagerExprData::MemoizedField {
            self_argument,
            self_argument_ty,
            ident,
            ref indirections,
            ref instantiation,
            ..
        } => {
            let ItemPath::AssocItem(path) = instantiation.path() else {
                unreachable!()
            };
            let self_ty = self_argument_ty.deref_if_leashed(db);
            match path {
                AssocItemPath::TypeItem(_) => {
                    builder.delimited(RustDelimiter::Angle, |builder| {
                        self_ty.transpile_to_rust(builder)
                    });
                }
                AssocItemPath::TraitItem(_) => todo!(),
                AssocItemPath::TraitForTypeItem(_) => todo!(),
            }
            builder.punctuation(RustPunctuation::ColonColon);
            ident.transpile_to_rust(builder);
            builder.delimited(RustDelimiter::Par, |builder| {
                (
                    self_argument,
                    HirEagerExprRole::memoized_field_self_argument(
                        self_argument_ty,
                        indirections,
                        db,
                    ),
                )
                    .transpile_to_rust(builder)
            });
        }
        HirEagerExprData::MethodRitchieCall {
            self_argument,
            self_contract,
            ident,
            path,
            ref indirections,
            ref instantiation,
            ref arguments,
        } => {
            (
                self_argument,
                HirEagerExprRole::self_argument_with_indirections(indirections),
            )
                .transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            let contracted_quaries = instantiation.contracted_quaries();
            match contracted_quaries.len() {
                0 => ident.transpile_to_rust(builder),
                1 => match contracted_quaries[0].quary().place() {
                    Some(place) => match place_contract_site[place] {
                        HirContract::Pure => ident.transpile_to_rust(builder),
                        HirContract::Move => todo!(),
                        HirContract::Borrow => todo!(),
                        HirContract::BorrowMut => builder.method_ritchie_ident_mut(ident),
                        HirContract::Compterm => todo!(),
                        HirContract::Leash => todo!(),
                        HirContract::At => todo!(),
                    },
                    None => ident.transpile_to_rust(builder),
                },
                _ => todo!(),
            }
            match path.ident(db).unwrap().data(db) {
                // ad hoc, should use path menu instead or refinement
                "visualize" => builder.delimited(RustDelimiter::Par, |builder| {
                    builder.visual_synchrotron_argument()
                }),
                _ => builder.delimited_comma_list(RustDelimiter::Par, arguments),
            }
        }
        HirEagerExprData::NewTuple { items: _ } => {
            todo!()
        }
        HirEagerExprData::Index { owner, ref items } => {
            // ad hoc
            (owner, HirEagerExprRole::simple_self_argument()).transpile_to_rust(builder);
            builder.delimited(RustDelimiter::Box, |builder| {
                (
                    items[0],
                    HirEagerExprRole::subexpr(RustPrecedenceRange::Geq(RustPrecedence::As)),
                )
                    .transpile_to_rust(builder);
                builder.keyword(RustKeyword::As);
                builder.usize()
            })
        }
        HirEagerExprData::NewList {
            exprs: ref items, ..
        } => {
            builder.macro_name(RustMacroName::Vec);
            builder.delimited_comma_list(
                RustDelimiter::Box,
                items
                    .iter()
                    .copied()
                    .map(|item| (item, HirEagerExprRole::subexpr_with_any_precedence_range())),
            )
        }
        HirEagerExprData::Block { stmts } => stmts.transpile_to_rust(builder),
        HirEagerExprData::EmptyHtmlTag {
            function_ident,
            ref arguments,
        } => {
            let macro_name = RustMacroName::HtmlTag(function_ident);
            builder.macro_name(macro_name);
            builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                builder.heterogeneous_comma_list_items(arguments.iter());
                builder.heterogeneous_comma_list_item_with(|builder| {
                    builder.visual_synchrotron_argument()
                })
            })
        }
        HirEagerExprData::Todo => {
            builder.macro_name(RustMacroName::Todo);
            builder.delimited(RustDelimiter::Par, |_| ())
        }
        HirEagerExprData::Unreachable => {
            builder.macro_name(RustMacroName::Unreachable);
            builder.delimited(RustDelimiter::Par, |_| ())
        }
        HirEagerExprData::AssocRitchie { assoc_item_path } => {
            assoc_item_path.transpile_to_rust(builder)
        }
        HirEagerExprData::As { opd, ty } => {
            (
                opd,
                HirEagerExprRole::subexpr(RustPrecedenceRange::Geq(RustPrecedence::As)),
            )
                .transpile_to_rust(builder);
            builder.keyword(RustKeyword::As);
            ty.transpile_to_rust(builder)
        }
        HirEagerExprData::Closure {
            ref parameters,
            return_ty,
            body,
            ..
        } => {
            builder.delimited_comma_list(RustDelimiter::Vert, parameters);
            match return_ty {
                Some(return_ty) => {
                    builder.punctuation(RustPunctuation::LightArrow);
                    return_ty.transpile_to_rust(builder);
                    builder.delimited(RustDelimiter::Curl, |builder| {
                        (body, HirEagerExprRole::subexpr(RustPrecedenceRange::Any))
                            .transpile_to_rust(builder)
                    })
                }
                None => {
                    (
                        body,
                        HirEagerExprRole::subexpr(RustPrecedenceRange::Geq(
                            RustPrecedence::Closure,
                        )),
                    )
                        .transpile_to_rust(builder);
                }
            }
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for &HirEagerRitchieArgument {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match *self {
            HirEagerRitchieArgument::Simple(param, hir_eager_expr_idx, coercion) => (
                hir_eager_expr_idx,
                HirEagerExprRole::regular_call_item(param.contract),
            )
                .transpile_to_rust(builder),
            HirEagerRitchieArgument::Variadic => todo!(),
            HirEagerRitchieArgument::Keyed => todo!(),
        }
    }
}
