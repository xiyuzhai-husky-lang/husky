use xxhash_rust::xxh3::xxh3_64;

pub use super::*;
pub struct PrimitiveTypeRegistration<'a> {
    pub ty: &'a str,
}

#[test]
fn it_works() {
    let i32_str: String = "i32".to_string();
    let hash = xxh3_64(i32_str.as_bytes());
    println!("hash = {hash}");
    assert_eq!(hash, 6639413044669031007)
}

impl<'a> Display for PrimitiveTypeRegistration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let uppercase_ty = ty.to_uppercase();
        let result = match ty {
            "void" => "false",
            "bool" => "data",
            "i32" | "i64" | "r32" | "b64" => "data != 0",
            "f32" | "f64" => "data != 0.0",
            _ => panic!(),
        };
        let typename_str_hash_u64: u64 = xxh3_64(ty.as_bytes());
        write!(
            f,
            r#"
// {ty}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{ty}_primitive_value_to_bool(data: __RegisterData) -> bool {{
    let data = data.as_{ty};
    {result}
}}

#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{ty}_primitive_ref_to_bool(data_ptr: *mut std::ffi::c_void) -> bool {{
    let data = unsafe {{ *(data_ptr as *const {ty}) }};
    {result}
}}

#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{ty}_primitive_value_to_box(data: __RegisterData) -> *mut std::ffi::c_void {{
    let data = data.as_{ty};
    let ptr: *mut {ty} = Box::<{ty}>::into_raw(Box::new(data));
    ptr as *mut std::ffi::c_void
}}

#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{ty}_clone(data: *mut c_void) -> *mut std::ffi::c_void {{
    Box::<{ty}>::into_raw(Box::new((*(data as *mut {ty})).clone())) as *mut std::ffi::c_void
}}

#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{ty}_drop(data: *mut c_void) {{
    drop(Box::from_raw(data as *mut {ty}))
}}

#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{ty}_eq(this: &std::ffi::c_void, other: &std::ffi::c_void) -> bool {{
    *(this as *const std::ffi::c_void as *const {ty}) == *(other as *const std::ffi::c_void as *const {ty})
}}

#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{ty}_assign(registers: *mut __Register) {{
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<{ty}>(&__{uppercase_ty}_VTABLE) = registers[1].downcast_{ty}()
}}

#[rustfmt::skip]
#[no_mangle]
pub static __{uppercase_ty}_VTABLE: __RegisterTyVTable = __RegisterTyVTable {{
    primitive_value_to_bool: Some(__{ty}_primitive_value_to_bool),
    primitive_ref_to_bool: Some(__{ty}_primitive_ref_to_bool),
    primitive_value_to_box: Some(__{ty}_primitive_value_to_box),
    clone: __{ty}_clone,
    drop: __{ty}_drop,
    eq: __{ty}_eq,
    assign: __{ty}_assign,
    typename_str: "{ty}",
    typename_str_hash_u64: {typename_str_hash_u64},
}};

#[rustfmt::skip]
impl<'eval> __Register<'eval> {{
    pub fn downcast_{ty}(&self) -> {ty} {{
        unsafe {{
            if self.vtable.typename_str_hash_u64 != {typename_str_hash_u64} {{
                panic!("expect `{ty}` but get {{}} instead", self.vtable.typename_str)
            }}
            match self.data_kind {{
                __RegisterDataKind::PrimitiveValue => self.data.as_{ty},
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => *(self.data.as_ptr as *const {ty}),
                _ => panic!(),
            }}
        }}
    }}

    pub fn downcast_opt_{ty}(&self) -> Option<{ty}> {{
        unsafe {{
            if self.vtable.typename_str_hash_u64 != {typename_str_hash_u64} {{
                panic!("expect `{ty}` but get `{{}}` instead", self.vtable.typename_str)
            }}
            match self.data_kind {{
                __RegisterDataKind::PrimitiveValue => Some(self.data.as_{ty}),
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => Some(*(self.data.as_ptr as *const {ty})),
                __RegisterDataKind::SomeNone => {{
                    assert_eq!(unsafe {{self.data.as_number_of_somes}}, 0);
                    None
                }},
                _ => panic!(),
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
        let typename_str_hash_u64: u64 = xxh3_64(ty.as_bytes());
        write!(
            f,
            r#"
// {ty}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{snake_ty}_clone(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {{
    Box::<{ty}>::into_raw(Box::new((*(data as *mut {ty})).clone())) as *mut std::ffi::c_void
}}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{snake_ty}_drop(data: *mut std::ffi::c_void) {{
    drop(Box::from_raw(data as *mut {ty}))
}}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{snake_ty}_eq(this: &std::ffi::c_void, other: &std::ffi::c_void) -> bool {{
    *(this as *const std::ffi::c_void as *const {ty}) == *(other as *const std::ffi::c_void as *const {ty})
}}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __{snake_ty}_assign(registers: *mut __Register) {{
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<{ty}>(&__{upper_snake_ty}_VTABLE) = registers[1].downcast_move(&__{upper_snake_ty}_VTABLE)
}}
#[rustfmt::skip]
#[no_mangle]
pub static __{upper_snake_ty}_VTABLE: __RegisterTyVTable = __RegisterTyVTable {{
    primitive_value_to_bool: None,
    primitive_ref_to_bool: None,
    primitive_value_to_box: None,
    clone: __{snake_ty}_clone,
    drop: __{snake_ty}_drop,
    eq: __{snake_ty}_eq,
    assign: __{snake_ty}_assign,
    typename_str_hash_u64: {typename_str_hash_u64},
    typename_str: "{ty}",
}};
"#
        )
    }
}
