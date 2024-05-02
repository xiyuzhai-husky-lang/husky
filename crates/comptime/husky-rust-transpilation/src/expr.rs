mod closure;
mod html;
pub(crate) mod pattern;
pub(crate) mod precedence;
pub(crate) mod site;
mod stmt;

pub(crate) use self::precedence::{RustPrecedence, RustPrecedenceRange};

use self::{precedence::hir_eager_expr_precedence, site::HirEagerExprSite};
use crate::{binding::RustBinding, *};
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::{major_item::MajorItemPath, PrincipalEntityPath};
use husky_hir_eager_expr::{
    emit_note_on_hir_eager_expr_codespan, HirEagerCondition, HirEagerElifBranch,
    HirEagerElseBranch, HirEagerExprData, HirEagerExprEntry, HirEagerExprIdx, HirEagerExprRegion,
    HirEagerIfBranch, HirEagerPatternData, HirEagerPatternIdx, HirEagerRitchieArgument,
    HirEagerStmtData, HirEagerStmtIdx, HirEagerStmtIdxRange,
};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    instantiation::HirTermSymbolicVariableResolution, place_contract_site::HirPlaceContractSite,
    quary::HirQuary, ritchie::HirContract, HirTemplateVariable, HirTemplateVariableClass,
};
use husky_opr::BinaryClosedOpr;
use smallvec::SmallVec;

impl TranspileToRustWith<HirEagerExprRegion> for (HirEagerExprIdx, HirEagerExprSite) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let (slf, mut site) = self;
        let entry = slf.entry(builder.hir_eager_expr_arena());
        let data = entry.data();
        let place_contract_site = entry.place_contract_site();
        let db = builder.db;
        let precedence = hir_eager_expr_precedence(data);
        let needs_deref = site.hir_eager_expr_needs_deref(entry);
        if needs_deref {
            site.rust_bindings.push(RustBinding::Deref)
        }
        let mut wrap_in_some_flag = false;
        if let Some(rust_binding) = site.rust_bindings.last() {
            match rust_binding {
                RustBinding::Deref
                | RustBinding::DerefCustomed
                | RustBinding::Reref
                | RustBinding::RerefMut => {
                    site.rust_precedence_range = RustPrecedenceRange::Geq(RustPrecedence::Prefix)
                }
                RustBinding::SelfValue => (),
                RustBinding::WrapInSome => site.rust_precedence_range = RustPrecedenceRange::ANY,
            }
        }
        let needs_extra_pars = site
            .rust_bindings
            .first()
            .map(|rust_binding| match rust_binding {
                RustBinding::Deref
                | RustBinding::DerefCustomed
                | RustBinding::Reref
                | RustBinding::RerefMut => {
                    !site.rust_precedence_range.include(RustPrecedence::Prefix)
                }
                RustBinding::SelfValue | RustBinding::WrapInSome => false,
            })
            .unwrap_or(false);
        if needs_extra_pars {
            builder.lpar();
        }
        for &rust_binding in &*site.rust_bindings {
            match rust_binding {
                RustBinding::Deref | RustBinding::DerefCustomed => {
                    builder.punctuation(RustPunctuation::DerefStar)
                }
                RustBinding::Reref => builder.punctuation(RustPunctuation::Ambersand),
                RustBinding::RerefMut => {
                    builder.punctuation(RustPunctuation::Ambersand);
                    builder.keyword(RustKeyword::Mut)
                }
                RustBinding::SelfValue => (),
                RustBinding::WrapInSome => builder.wrap_in_some_left(&mut wrap_in_some_flag),
            }
        }
        if !site.rust_precedence_range.include(precedence) {
            builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                transpile_hir_eager_expr_to_rust(data, place_contract_site, precedence, builder)
            })
        } else {
            transpile_hir_eager_expr_to_rust(data, place_contract_site, precedence, builder)
        }
        if wrap_in_some_flag {
            builder.wrap_in_some_right()
        }
        if needs_extra_pars {
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
    let subexpr_geq = || HirEagerExprSite::subexpr(RustPrecedenceRange::Geq(precedence));
    let subexpr_greater = || HirEagerExprSite::subexpr(RustPrecedenceRange::Greater(precedence));
    let db = builder.db();
    match *data {
        HirEagerExprData::Literal(term_literal) => term_literal.transpile_to_rust(builder),
        HirEagerExprData::PrincipalEntityPath(principal_entity_path) => {
            principal_entity_path.transpile_to_rust(builder);
            match principal_entity_path {
                PrincipalEntityPath::Module(_) => unreachable!(),
                PrincipalEntityPath::MajorItem(MajorItemPath::Form(path)) => {
                    if let MajorFormKind::Val = path.major_form_kind(db) {
                        builder.delimited(RustDelimiter::Par, |_| ())
                    }
                }
                PrincipalEntityPath::TypeVariant(_) => (),
                PrincipalEntityPath::MajorItem(_) => (),
            }
        }
        HirEagerExprData::ConstVariable { ident, .. } => ident.transpile_to_rust(builder),
        HirEagerExprData::Variable(hir_eager_runtime_symbol_idx) => {
            hir_eager_runtime_symbol_idx.transpile_to_rust(builder)
        }
        HirEagerExprData::Binary { lopd, opr, ropd } => match opr {
            HirBinaryOpr::Closed(BinaryClosedOpr::RemEuclid) => {
                (lopd, HirEagerExprSite::self_expr_on_site(false)).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                builder.rem_eulid();
                builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                    (ropd, HirEagerExprSite::any_precedence()).transpile_to_rust(builder)
                })
            }
            HirBinaryOpr::Closed(BinaryClosedOpr::Power) => {
                (lopd, HirEagerExprSite::self_expr_on_site(false)).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                builder.pow();
                builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                    (ropd, HirEagerExprSite::any_precedence()).transpile_to_rust(builder)
                })
            }
            HirBinaryOpr::Assign | HirBinaryOpr::AssignClosed(_) | HirBinaryOpr::AssignShift(_) => {
                (lopd, HirEagerExprSite::self_expr_on_site(false)).transpile_to_rust(builder);
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
        HirEagerExprData::Prefix {
            opr,
            opd: opd_hir_expr_idx,
        } => {
            match opr {
                HirPrefixOpr::NotInt => builder.delimited(RustDelimiter::Par, |builder| {
                    (
                        opd_hir_expr_idx,
                        HirEagerExprSite::subexpr(RustPrecedenceRange::Geq(
                            RustPrecedence::EqComparison,
                        )),
                    )
                        .transpile_to_rust(builder);
                    builder.eq_zero()
                }),
                _ => {
                    // todo: check some details
                    opr.transpile_to_rust(builder);
                    (opd_hir_expr_idx, subexpr_geq()).transpile_to_rust(builder)
                }
            }
        }
        HirEagerExprData::Suffix {
            opd: opd_hir_expr_idx,
            opr,
        } => match opr {
            HirSuffixOpr::Incr | HirSuffixOpr::Decr => {
                (opd_hir_expr_idx, subexpr_geq()).transpile_to_rust(builder);
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
                            HirTemplateVariable::Const(symbol) => (symbol.index(db).class()
                                == HirTemplateVariableClass::Runtime)
                                .then_some(resolution),
                            _ => None,
                        })
                        .collect();
                builder.delimited_comma_list_with_last_comma(RustDelimiter::Par, runtime_constants)
            })
        }
        HirEagerExprData::Unwrap { opd } => {
            (opd, HirEagerExprSite::self_expr_on_site(false)).transpile_to_rust(builder);
            builder.call_unwrap()
        }
        HirEagerExprData::TypeConstructorCall {
            path,
            instantiation: _,
            arguments: ref item_groups,
        } => {
            builder.struct_ty_constructor_path(path);
            builder.delimited_comma_list(RustDelimiter::Par, item_groups)
        }
        HirEagerExprData::TypeVariantConstructorCall {
            path,
            instantiation: _,
            arguments: ref item_groups,
        } => {
            path.transpile_to_rust(builder);
            builder.delimited_comma_list(RustDelimiter::Par, item_groups)
        }
        HirEagerExprData::FunctionRitchieCall {
            path,
            instantiation: _,
            arguments: ref item_groups,
        } => {
            path.transpile_to_rust(builder);
            builder.delimited_comma_list(RustDelimiter::Par, item_groups)
        }
        HirEagerExprData::AssocFunctionRitchieCall {
            path,
            arguments: ref item_groups,
            ..
        } => {
            path.transpile_to_rust(builder);
            builder.delimited_comma_list(RustDelimiter::Par, item_groups)
        }
        HirEagerExprData::PropsStructField {
            self_argument: owner_hir_expr_idx,
            ident,
            ..
        } => {
            (
                owner_hir_expr_idx,
                HirEagerExprSite::self_expr_on_site(true),
            )
                .transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            ident.transpile_to_rust(builder)
        }
        HirEagerExprData::MemoizedField {
            self_argument: owner_hir_expr_idx,
            ident,
            ..
        } => {
            (owner_hir_expr_idx, subexpr_geq()).transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            ident.transpile_to_rust(builder);
            builder.delimited(RustDelimiter::Par, |_| ())
        }
        HirEagerExprData::MethodRitchieCall {
            self_argument,
            self_contract,
            ident,
            path,
            ref instantiation,
            arguments: ref item_groups,
        } => {
            (self_argument, HirEagerExprSite::self_expr_on_site(true)).transpile_to_rust(builder);
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
                        HirContract::Const => todo!(),
                        HirContract::Leash => todo!(),
                        HirContract::At => todo!(),
                    },
                    None => ident.transpile_to_rust(builder),
                },
                _ => todo!(),
            }
            match path.ident(db).unwrap().data(db) {
                // ad hoc, should use path menu instead
                "visualize" => builder.delimited(RustDelimiter::Par, |builder| {
                    builder.visual_synchrotron_argument()
                }),
                _ => builder.delimited_comma_list(RustDelimiter::Par, item_groups),
            }
        }
        HirEagerExprData::NewTuple { items: _ } => {
            todo!()
        }
        HirEagerExprData::Index { owner, ref items } => {
            // ad hoc
            (owner, HirEagerExprSite::self_expr_on_site(true)).transpile_to_rust(builder);
            builder.delimited(RustDelimiter::Box, |builder| {
                (
                    items[0],
                    HirEagerExprSite::subexpr(RustPrecedenceRange::Geq(RustPrecedence::As)),
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
                    .map(|item| (item, HirEagerExprSite::any_precedence())),
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
                HirEagerExprSite::subexpr(RustPrecedenceRange::Geq(RustPrecedence::As)),
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
                        (body, HirEagerExprSite::subexpr(RustPrecedenceRange::Any))
                            .transpile_to_rust(builder)
                    })
                }
                None => {
                    (
                        body,
                        HirEagerExprSite::subexpr(RustPrecedenceRange::Geq(
                            RustPrecedence::Closure,
                        )),
                    )
                        .transpile_to_rust(builder);
                }
            }
        }
    }
}

impl HirEagerExprSite {
    fn hir_eager_expr_needs_deref(&self, entry: &HirEagerExprEntry) -> bool {
        match *entry.data() {
            HirEagerExprData::Variable(_) => match entry.quary() {
                HirQuary::Const | HirQuary::StackPure { .. } => !entry.is_always_copyable(),
                HirQuary::Ref { .. } => true,
                HirQuary::RefMut { .. } => true,
                _ => false,
            },
            HirEagerExprData::ConstVariable { .. }
            | HirEagerExprData::FunctionRitchieCall { .. }
            | HirEagerExprData::AssocFunctionRitchieCall { .. }
            | HirEagerExprData::MemoizedField { .. }
            | HirEagerExprData::MethodRitchieCall { .. }
            | HirEagerExprData::Suffix { .. }
            | HirEagerExprData::Unveil { .. }
            | HirEagerExprData::Unwrap { .. } => match entry.quary() {
                HirQuary::Const | HirQuary::StackPure { .. } => !entry.is_always_copyable(),
                quary => match quary.place() {
                    Some(place) => match entry.place_contract_site().get(place) {
                        Some(contract) => match contract {
                            HirContract::Pure | HirContract::Const => !entry.is_always_copyable(),
                            HirContract::Move => false,
                            HirContract::Borrow => true,
                            HirContract::BorrowMut => true,
                            HirContract::Leash => todo!(),
                            HirContract::At => todo!(),
                        },
                        None => false,
                    },
                    None => false,
                },
            },
            _ => false,
        }
    }
}
impl TranspileToRustWith<HirEagerExprRegion> for &HirEagerRitchieArgument {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match *self {
            HirEagerRitchieArgument::Simple(param, hir_eager_expr_idx, coercion) => (
                hir_eager_expr_idx,
                HirEagerExprSite::regular_call_item(param, coercion, builder.db()),
            )
                .transpile_to_rust(builder),
            HirEagerRitchieArgument::Variadic => todo!(),
            HirEagerRitchieArgument::Keyed => todo!(),
        }
    }
}
