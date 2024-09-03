use crate::*;
use delimiter::Delimiter;
use ident::Ident;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    LetInit {
        pattern: Idx,
    },
    LetInitIdent,
    StructDefn(Ident),
    EnumDefn(Ident),
    FnDefn(Ident),
    FnDefnCallForm(Ident),
    FnDefnCallFormParameters(Ident),
    FnDefnCallFormBody(Ident),
    StructFields(Ident),
    FnDefnCallFormParameter {
        fn_ident: Ident,
        rank: Rank,
        ty: Idx,
    },
    FnDefnCallFormParameterType {
        fn_ident: Ident,
        rank: Rank,
    },
    StructField {
        ty_ident: Ident,
        field_ident_idx: Idx,
        ty_idx: Idx,
    },
    StructFieldType {
        ty_ident: Ident,
        field_ident_idx: Idx,
    },
}

impl Ast {
    fn role(self) -> Option<Role> {
        match self.data {
            AstData::LetInit {
                expr,
                pattern,
                initial_value,
            } => Some(Role::LetInit { pattern }),
            AstData::Defn {
                keyword,
                ident_idx,
                ident,
                content,
            } => Some(match keyword {
                DefnKeyword::Struct => Role::StructDefn(ident),
                DefnKeyword::Enum => Role::EnumDefn(ident),
                DefnKeyword::Fn => Role::FnDefn(ident),
            }),
            _ => None,
        }
    }
}

pub fn populate_roles_n_times(asts: Seq<Option<Ast>>, n: usize) -> Seq<Option<Role>> {
    let mut roles: Seq<Option<Role>> = asts.map(|ast| ast?.role());
    let ranks = calc_ranks(asts);
    for _ in 0..n {
        let parent_roles = parent_queries(asts, roles);
        roles = populate_roles(asts, parent_roles, roles, ranks);
    }
    roles
}

fn populate_roles(
    asts: Seq<Option<Ast>>,
    parent_roles: Seq<Option<Role>>,
    roles: Seq<Option<Role>>,
    ranks: Seq<Option<Rank>>,
) -> Seq<Option<Role>> {
    populate_role.apply_enumerated(asts, parent_roles, roles, ranks)
}

fn populate_role(
    idx: Idx,
    ast: Option<Ast>,
    parent_role: Option<Role>,
    role: Option<Role>,
    rank: Option<Rank>,
) -> Option<Role> {
    if let Some(role) = role {
        return Some(role);
    }
    let ast = ast?;
    if let Some(role) = ast.role() {
        return Some(role);
    }
    match parent_role? {
        Role::LetInit { pattern } => match ast.data {
            AstData::Ident(ident) if idx == pattern => Some(Role::LetInitIdent),
            AstData::Binary {
                lopd,
                opr: BinaryOpr::Assign,
                ropd,
                lopd_ident,
            } if lopd == pattern => Some(Role::LetInit { pattern }),
            _ => None,
        },
        Role::LetInitIdent => todo!(),
        Role::StructDefn(ident) => match ast.data {
            AstData::Literal(_) => todo!(),
            AstData::Ident(_) => None,
            AstData::Prefix { opr, opd } => todo!(),
            AstData::Binary {
                lopd,
                opr,
                ropd,
                lopd_ident,
            } => todo!(),
            AstData::Suffix { opd, opr } => todo!(),
            AstData::Delimited {
                left_delimiter_idx,
                left_delimiter,
                right_delimiter,
            } => Some(Role::StructFields(ident)),
            AstData::SeparatedItem { content, separator } => todo!(),
            AstData::Call { .. } => todo!(),
            AstData::LetInit {
                expr,
                pattern,
                initial_value,
            } => todo!(),
            AstData::Return { result } => todo!(),
            AstData::Assert { condition } => todo!(),
            AstData::If { condition, body } => todo!(),
            AstData::Else { if_stmt, body } => todo!(),
            AstData::Defn {
                keyword,
                ident_idx,
                ident,
                content,
            } => todo!(),
        },
        Role::EnumDefn(_) => None, // ad hoc
        Role::FnDefn(ident) => match ast.data {
            AstData::Literal(_) => todo!(),
            AstData::Ident(_) => None,
            AstData::Prefix { opr, opd } => todo!(),
            AstData::Binary {
                lopd,
                opr,
                ropd,
                lopd_ident,
            } => todo!(),
            AstData::Suffix { opd, opr } => todo!(),
            AstData::Delimited {
                left_delimiter_idx,
                left_delimiter,
                right_delimiter,
            } => todo!(),
            AstData::SeparatedItem { content, separator } => todo!(),
            AstData::Call { .. } => Some(Role::FnDefnCallForm(ident)),
            AstData::LetInit {
                expr,
                pattern,
                initial_value,
            } => todo!(),
            AstData::Return { result } => todo!(),
            AstData::Assert { condition } => todo!(),
            AstData::If { condition, body } => todo!(),
            AstData::Else { if_stmt, body } => todo!(),
            AstData::Defn {
                keyword,
                ident_idx,
                ident,
                content,
            } => todo!(),
        },
        Role::FnDefnCallForm(ident) => match ast.data {
            AstData::Literal(_) => todo!(),
            AstData::Ident(_) => todo!(),
            AstData::Prefix { opr, opd } => todo!(),
            AstData::Binary {
                lopd,
                opr,
                ropd,
                lopd_ident,
            } => todo!(),
            AstData::Suffix { opd, opr } => todo!(),
            AstData::Delimited {
                left_delimiter_idx,
                left_delimiter,
                right_delimiter,
            } => match left_delimiter.delimiter() {
                Delimiter::Parenthesis => Some(Role::FnDefnCallFormParameters(ident)),
                Delimiter::Box => todo!(),
                Delimiter::Curly => Some(Role::FnDefnCallFormBody(ident)),
            },
            AstData::SeparatedItem { content, separator } => todo!(),
            AstData::Call { .. } => todo!(),
            AstData::LetInit {
                expr,
                pattern,
                initial_value,
            } => todo!(),
            AstData::Return { result } => todo!(),
            AstData::Assert { condition } => todo!(),
            AstData::If { condition, body } => todo!(),
            AstData::Else { if_stmt, body } => todo!(),
            AstData::Defn {
                keyword,
                ident_idx,
                ident,
                content,
            } => todo!(),
        },
        Role::FnDefnCallFormParameters(fn_ident) => match ast.data {
            AstData::Literal(_) => todo!(),
            AstData::Ident(_) => todo!(),
            AstData::Prefix { opr, opd } => todo!(),
            AstData::Binary {
                lopd,
                opr,
                ropd,
                lopd_ident,
            } => {
                if opr == BinaryOpr::TypeIs {
                    Some(Role::FnDefnCallFormParameter {
                        fn_ident,
                        rank: rank.unwrap(),
                        ty: ropd,
                    })
                } else {
                    todo!()
                }
            }
            AstData::Suffix { opd, opr } => todo!(),
            AstData::Delimited {
                left_delimiter_idx,
                left_delimiter,
                right_delimiter,
            } => todo!(),
            AstData::SeparatedItem { content, separator } => todo!(),
            AstData::Call { .. } => todo!(),
            AstData::LetInit {
                expr,
                pattern,
                initial_value,
            } => todo!(),
            AstData::Return { result } => todo!(),
            AstData::Assert { condition } => todo!(),
            AstData::If { condition, body } => todo!(),
            AstData::Else { if_stmt, body } => todo!(),
            AstData::Defn {
                keyword,
                ident_idx,
                ident,
                content,
            } => todo!(),
        },
        Role::FnDefnCallFormBody(_) => None,
        Role::StructFields(ty_ident) => match ast.data {
            AstData::Literal(_) => todo!(),
            AstData::Ident(_) => todo!(),
            AstData::Prefix { opr, opd } => todo!(),
            AstData::Binary {
                lopd,
                opr,
                ropd,
                lopd_ident,
            } => {
                assert_eq!(opr, BinaryOpr::TypeIs);
                Some(Role::StructField {
                    ty_ident,
                    field_ident_idx: lopd,
                    ty_idx: ropd,
                })
            }
            AstData::Suffix { opd, opr } => todo!(),
            AstData::Delimited {
                left_delimiter_idx,
                left_delimiter,
                right_delimiter,
            } => todo!(),
            AstData::SeparatedItem { content, separator } => todo!(),
            AstData::Call {
                caller,
                caller_ident,
                left_delimiter,
                right_delimiter,
                delimited_arguments,
            } => todo!(),
            AstData::LetInit {
                expr,
                pattern,
                initial_value,
            } => todo!(),
            AstData::Return { result } => todo!(),
            AstData::Assert { condition } => todo!(),
            AstData::If { condition, body } => todo!(),
            AstData::Else { if_stmt, body } => todo!(),
            AstData::Defn {
                keyword,
                ident_idx,
                ident,
                content,
            } => todo!(),
        },
        Role::FnDefnCallFormParameter { fn_ident, rank, ty } => {
            if idx == ty {
                Some(Role::FnDefnCallFormParameterType { fn_ident, rank })
            } else {
                None
            }
        }
        Role::FnDefnCallFormParameterType { fn_ident, rank } => todo!(),
        Role::StructField {
            ty_ident,
            field_ident_idx,
            ty_idx,
        } => {
            if idx == ty_idx {
                Some(Role::StructFieldType {
                    ty_ident,
                    field_ident_idx,
                })
            } else {
                None
            }
        }
        Role::StructFieldType {
            ty_ident,
            field_ident_idx,
        } => todo!(),
    }
}

#[cfg(test)]
fn t(input: &str, expect: Expect) {
    let (tokens, pre_asts, asts) =
        calc_asts_from_input_together_with_tokens_and_pre_asts(input, 10);
    let roles = populate_roles_n_times(asts, 10);
    expect.assert_debug_eq(&show_asts_mapped_values(tokens, pre_asts, asts, roles))
}

#[test]
fn populate_roles_n_times_works() {
    t(
        "",
        expect![[r#"
        []
    "#]],
    );
    t(
        "struct A {}",
        expect![[r#"
            [
                #0 `struct`: "struct A {}" ✓ → StructDefn(`A`),
                #1 `A`: "A",
                #2 `{`: `{`,
                #3 `}`: "{}" → StructFields(`A`),
            ]
        "#]],
    );
    t(
        "fn f(x: i32) {}",
        expect![[r#"
            [
                #0 `fn`: "fn f(x : i32) {}" ✓ → FnDefn(`f`),
                #1 `f`: "f",
                #2 `(`: `(`,
                #3 `x`: "x",
                #4 `:`: "x : i32" → FnDefnCallFormParameter { fn_ident: `f`, rank: Rank(0), ty: #5 },
                #5 `i32`: "i32" → FnDefnCallFormParameterType { fn_ident: `f`, rank: Rank(0) },
                #6 `)`: "(x : i32)" → FnDefnCallFormParameters(`f`),
                #7 `{`: "(x : i32) {}" → FnDefnCallForm(`f`),
                #8 `}`: "{}" → FnDefnCallFormBody(`f`),
            ]
        "#]],
    );
}

#[test]
fn populate_roles_n_times_works1() {
    t(
        "",
        expect![[r#"
        []
    "#]],
    );
}

#[test]
fn populate_roles_n_times_for_struct_works() {
    t(
        "struct A {}",
        expect![[r#"
            [
                #0 `struct`: "struct A {}" ✓ → StructDefn(`A`),
                #1 `A`: "A",
                #2 `{`: `{`,
                #3 `}`: "{}" → StructFields(`A`),
            ]
        "#]],
    );
    t(
        "struct A { a: i32 }",
        expect![[r#"
            [
                #0 `struct`: "struct A { a : i32 }" ✓ → StructDefn(`A`),
                #1 `A`: "A",
                #2 `{`: `{`,
                #3 `a`: "a",
                #4 `:`: "a : i32" → StructField { ty_ident: `A`, field_ident_idx: #3, ty_idx: #5 },
                #5 `i32`: "i32" → StructFieldType { ty_ident: `A`, field_ident_idx: #3 },
                #6 `}`: "{ a : i32 }" → StructFields(`A`),
            ]
        "#]],
    );
}
