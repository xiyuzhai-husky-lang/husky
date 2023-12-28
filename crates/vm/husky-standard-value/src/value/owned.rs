use super::*;

#[derive(Debug)]
pub struct OwnedValue(Option<Box<dyn StaticDyn>>);

impl Drop for OwnedValue {
    fn drop(&mut self) {
        match std::mem::take(&mut self.0) {
            Some(boxed_value) => {
                let type_name = boxed_value.type_name_dyn();
                println!("Dropping OwnedValue of type `{}` begin", type_name);
                println!("value is {boxed_value:?}");
                std::mem::drop(boxed_value);
                println!("Dropping OwnedValue of type `{}` end", type_name);
            }
            None => println!("Dropping OwnedValue moved"),
        }
    }
}

impl OwnedValue {
    pub(super) fn upcast_from_owned<T>(t: T) -> Self
    where
        T: Static,
    {
        Self(Some(Box::<T>::new(t)))
    }

    pub fn downcast_into_owned<T>(mut self) -> T
    where
        T: 'static,
    {
        *((std::mem::take(&mut self.0).unwrap() as Box<dyn std::any::Any>)
            .downcast()
            .unwrap())
    }

    pub(super) fn downcast_as_ref<T>(&self) -> &T
    where
        T: WeakStatic,
    {
        unsafe {
            std::mem::transmute(
                ((&**self.0.as_ref().unwrap() as &dyn StaticDyn) as &dyn std::any::Any)
                    .downcast_ref::<T::Static>()
                    .unwrap(),
            )
        }
    }

    pub(super) fn into_inner(mut self) -> Box<dyn StaticDyn> {
        std::mem::take(&mut self.0).unwrap()
    }

    pub(super) fn as_ref(&self) -> &dyn StaticDyn {
        &**self.0.as_ref().unwrap()
    }
}
