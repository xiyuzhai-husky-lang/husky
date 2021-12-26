#[macro_export]
macro_rules! any_func {
    // tys are raw types, don't use reference
    (pure $f: ident($($args:ident:$tys:ty), *) $body:expr) => {
        pub fn $f(pure_inputs: Vec<&dyn Any>) -> Result<Box<dyn Any>, RuntimeError> {
            let mut pure_input_iter = pure_inputs.into_iter();
            Ok(Box::new((
                |$($args:&$tys), *|$body)(
                    $(pure_input_iter.next().unwrap().downcast_ref::<$tys>().unwrap()),*
                )
            ))
        }
    };
}

#[macro_export]
macro_rules! call {
    (pure $f:ident($($args:expr),*):$output_ty:ty) => {{
         *$f(vec![$($args),*]).unwrap().downcast::<$output_ty>().unwrap()
    }}
}
