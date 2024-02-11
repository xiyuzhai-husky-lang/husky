#[macro_export]
macro_rules! mk_id {
  ($($id:ident($ty:ty) $(+ Visit($visit:ident))?,)*) => {
    $(
      #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
      pub struct $id(pub $ty);
      impl $crate::Idx for $id {
        fn from_usize(n: usize) -> Self { Self(n as $ty) }
        fn into_usize(self) -> usize { self.0 as usize }
      }
      impl std::fmt::Debug for $id {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          std::fmt::Debug::fmt(&self.0, f)
        }
      }
      impl std::str::FromStr for $id {
        type Err = std::num::ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> { <$ty>::from_str(s).map($id) }
      }
      impl $crate::from_str_pos::FromStrPos for $id {
        fn to_err(_: Self::Err, pos: usize) -> $crate::from_str_pos::FromStrPosParseError {
          $crate::from_str_pos::FromStrPosParseError::BadInteger(pos)
        }
      }

      impl $crate::helpers::IdxOrBool for $id {
        fn is_bool_ty() -> bool {
          false
        }

        fn into_usize(self) -> usize {
          <$id as $crate::Idx>::into_usize(self)
        }

        #[track_caller]
        fn into_bool(self) -> bool {
          panic!("`{}` is not bool", std::any::type_name::<$id>())
        }
      }

      $(impl<V: VisitMut> Visitable<V> for $id {
        fn visit(&mut self, v: &mut V) { v.$visit(self) }
      })?
    )*
  };
}
