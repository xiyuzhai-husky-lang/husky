use crate::*;
use delimiter::Delimiter;
use ident::Ident;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    LetInit { pattern: Idx },
    LetInitIdent,
    StructDefn(Ident),
    EnumDefn(Ident),
    FnDefn(Ident),
    FnDefnCallForm(Ident),
    FnDefnCallFormParameters(Ident),
    FnDefnCallFormBody(Ident),
    StructFields(Ident),
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
    for _ in 0..n {
        let parent_roles = parent_queries(asts, roles);
        roles = populate_roles(asts, parent_roles, roles);
    }
    roles
}

fn populate_roles(
    asts: Seq<Option<Ast>>,
    parent_roles: Seq<Option<Role>>,
    roles: Seq<Option<Role>>,
) -> Seq<Option<Role>> {
    populate_role.apply_enumerated(asts, parent_roles, roles)
}

fn populate_role(
    idx: Idx,
    ast: Option<Ast>,
    parent_role: Option<Role>,
    role: Option<Role>,
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
            } if lopd == pattern => Some(Role::LetInit { pattern }),
            _ => None,
        },
        Role::LetInitIdent => todo!(),
        Role::StructDefn(ident) => match ast.data {
            AstData::Literal(_) => todo!(),
            AstData::Ident(_) => None,
            AstData::Prefix { opr, opd } => todo!(),
            AstData::Binary { lopd, opr, ropd } => todo!(),
            AstData::Suffix { opd, opr } => todo!(),
            AstData::Delimited {
                left_delimiter_idx,
                left_delimiter,
                right_delimiter,
            } => Some(Role::StructFields(ident)),
            AstData::SeparatedItem { content, separator } => todo!(),
            AstData::Call {
                caller,
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
        Role::EnumDefn(_) => None, // ad hoc
        Role::FnDefn(ident) => match ast.data {
            AstData::Literal(_) => todo!(),
            AstData::Ident(_) => None,
            AstData::Prefix { opr, opd } => todo!(),
            AstData::Binary { lopd, opr, ropd } => todo!(),
            AstData::Suffix { opd, opr } => todo!(),
            AstData::Delimited {
                left_delimiter_idx,
                left_delimiter,
                right_delimiter,
            } => todo!(),
            AstData::SeparatedItem { content, separator } => todo!(),
            AstData::Call {
                caller,
                left_delimiter,
                right_delimiter,
                delimited_arguments,
            } => Some(Role::FnDefnCallForm(ident)),
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
            AstData::Binary { lopd, opr, ropd } => todo!(),
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
            AstData::Call {
                caller,
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
        Role::FnDefnCallFormParameters(_) => todo!(),
        Role::FnDefnCallFormBody(_) => None,
        Role::StructFields(_) => todo!(),
    }
}

#[test]
fn populate_roles_n_times_works() {
    fn t(input: &str, expect: Expect) {
        let (tokens, pre_asts, asts) =
            calc_asts_from_input_together_with_tokens_and_pre_asts(input, 10);
        let roles = populate_roles_n_times(asts, 10);
        expect.assert_debug_eq(&show_asts_mapped_values(tokens, pre_asts, asts, roles))
    }
    t(
        "",
        expect![[r#"
        []
    "#]],
    );
}
