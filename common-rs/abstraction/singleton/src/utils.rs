pub use paste::paste;

#[macro_export]
macro_rules! singleton {
    ($Type:ty) => {
        singleton::utils::paste! {
            use singleton::*;
            static mut [<$Type:upper SINGLETON>]: Option<*const $Type> = None;

            pub struct [<$Type SingletonKeeper>](Box<$Type>);

            type SingletonKeeper = [<$Type SingletonKeeper>];

            pub fn init_singleton() -> &'static $Type {
                unsafe { &*([<$Type:upper SINGLETON>].unwrap()) }
            }

            pub fn __access_singleton() -> &'static $Type {
                unsafe { &*([<$Type:upper SINGLETON>].unwrap()) }
            }

            impl std::ops::Deref for SingletonKeeper {
                type Target = $Type;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl Drop for SingletonKeeper {
                fn drop(&mut self) {
                    unsafe { [<$Type:upper SINGLETON>] = None }
                }
            }

            impl From<$Type> for SingletonKeeper {
                fn from(eval_time: $Type) -> Self {
                    let boxed_eval_time = Box::new(eval_time);
                    unsafe {
                        assert!([<$Type:upper SINGLETON>].is_none());
                        [<$Type:upper SINGLETON>] = Some(&*boxed_eval_time)
                    };
                    Self(boxed_eval_time)
                }
            }
        }
    };
}
