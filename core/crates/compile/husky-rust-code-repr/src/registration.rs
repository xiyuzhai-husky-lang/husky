pub use super::*;
pub struct PrimitiveTypeRegistration<'a> {
    pub ty: &'a str,
}

impl<'a> Display for PrimitiveTypeRegistration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let uppercase_ty = ty.to_uppercase();
        let result = match ty {
            "void" => "false",
            "bool" => "data",
            "i32" | "i64" | "b32" | "b64" => "data != 0",
            "f32" | "f64" => "data != 0.0",
            _ => panic!(),
        };
        write!(
            f,
            r#"
// {ty}
#[no_mangle]
pub unsafe extern "C" fn __{ty}_primitive_value_to_bool(data: __RegisterData) -> bool {{
    let data = data.as_{ty};
    {result}
}}
#[no_mangle]
pub unsafe extern "C" fn __{ty}_primitive_value_to_box(data: __RegisterData) -> *mut () {{
    let data = data.as_{ty};
    let ptr: *mut {ty} = Box::<{ty}>::into_raw(Box::new(data));
    ptr as *mut ()
}}
#[no_mangle]
pub unsafe extern "C" fn __{ty}_clone(data: *mut ()) -> *mut () {{
    Box::<{ty}>::into_raw(Box::new((*(data as *mut {ty})).clone())) as *mut ()
}}
#[no_mangle]
pub unsafe extern "C" fn __{ty}_drop(data: *mut ()) {{
    Box::from_raw(data as *mut {ty});
}}
#[no_mangle]
pub unsafe extern "C" fn __{ty}_eq(this: &(), other: &()) -> bool {{
    *(this as *const () as *const {ty}) == *(other as *const () as *const {ty})
}}
#[no_mangle]
pub unsafe extern "C" fn __{ty}_assign(registers: *mut __Register) {{
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<{ty}>() = registers[1].downcast_{ty}()
}}
#[no_mangle]
pub static __{uppercase_ty}_VTABLE: __RegisterVTable = __RegisterVTable {{
    primitive_value_to_bool: Some(__{ty}_primitive_value_to_bool),
    primitive_value_to_box: Some(__{ty}_primitive_value_to_box),
    clone: __{ty}_clone,
    drop: __{ty}_drop,
    eq: __{ty}_eq,
    assign: __{ty}_assign,
}};
impl<'eval> __Register<'eval> {{
    pub fn downcast_{ty}(&self) -> {ty} {{
        unsafe {{
            assert_eq!(self.vtable as *const _, &__{uppercase_ty}_VTABLE as *const _);
            match self.data_kind {{
                __RegisterDataKind::PrimitiveValue => self.data.as_{ty},
                _ => *(self.data.as_ptr as *const {ty}),
            }}
        }}
    }}
}}
"#
        )
    }
}

pub struct NonPrimitiveTypeRegistration<'a> {
    pub ty: &'a str,
}

impl<'a> Display for NonPrimitiveTypeRegistration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let snake_ty = ty.to_case(Case::Snake);
        let upper_snake_ty = ty.to_case(Case::UpperSnake);
        write!(
            f,
            r#"
// {ty}
#[no_mangle]
pub unsafe extern "C" fn __{snake_ty}_clone(data: *mut ()) -> *mut () {{
    Box::<{ty}>::into_raw(Box::new((*(data as *mut {ty})).clone())) as *mut ()
}}
#[no_mangle]
pub unsafe extern "C" fn __{snake_ty}_drop(data: *mut ()) {{
    Box::from_raw(data as *mut {ty});
}}
#[no_mangle]
pub unsafe extern "C" fn __{snake_ty}_eq(this: &(), other: &()) -> bool {{
    *(this as *const () as *const {ty}) == *(other as *const () as *const {ty})
}}
#[no_mangle]
pub unsafe extern "C" fn __{snake_ty}_assign(registers: *mut __Register) {{
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<{ty}>() = registers[1].downcast_move()
}}
#[no_mangle]
pub static __{upper_snake_ty}_VTABLE: __RegisterVTable = __RegisterVTable {{
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __{snake_ty}_clone,
    drop: __{snake_ty}_drop,
    eq: __{snake_ty}_eq,
    assign: __{snake_ty}_assign,
}};
"#
        )
    }
}
