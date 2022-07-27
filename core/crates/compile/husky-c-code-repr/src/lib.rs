use std::fmt::Display;

use convert_case::{Case, Casing};
pub struct CPrimitiveTypeRegistrationHeader<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CPrimitiveTypeRegistrationHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let uppercase_ty = ty.to_uppercase();
        write!(
            f,
            r#"
// {ty}
extern bool __{ty}_primitive_value_to_bool(__RegisterData data);
extern void *__{ty}_primitive_value_to_box(__RegisterData data);
extern void __{ty}_drop(void*);
extern const __RegisterVTable __{uppercase_ty}_VTABLE;
        "#
        )
    }
}

pub struct CPrimitiveTypeRegistrationSource<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CPrimitiveTypeRegistrationSource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let uppercase_ty = ty.to_uppercase();
        write!(
            f,
            r#"
const __RegisterVTable __{uppercase_ty}_VTABLE = {{
    .typename_str = "{ty}",
    .primitive_value_to_bool = __{ty}_primitive_value_to_bool,
    .primitive_value_to_box = __{ty}_primitive_value_to_box,
    .drop = __{ty}_drop,
}};
"#
        )
    }
}

pub struct CNonPrimitiveTypeRegistrationHeader<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CNonPrimitiveTypeRegistrationHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let lower_snake_ty = ty.to_case(Case::Snake);
        let upper_snake_ty = ty.to_case(Case::UpperSnake);
        write!(
            f,
            r#"
// {ty}
extern void __{lower_snake_ty}_drop(void*);
extern const __RegisterVTable __{upper_snake_ty}_VTABLE;
        "#
        )
    }
}

pub struct CNonPrimitiveTypeRegistrationSource<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CNonPrimitiveTypeRegistrationSource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let lower_snake_ty = ty.to_case(Case::Snake);
        let upper_snake_ty = ty.to_case(Case::UpperSnake);
        write!(
            f,
            r#"
const __RegisterVTable __{upper_snake_ty}_VTABLE = {{
    .typename_str = "{ty}",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .drop = __{lower_snake_ty}_drop,
}};
"#
        )
    }
}
