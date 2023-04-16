import Specs.abstraction

-- chore
inductive TyKind
  | Enum
  | Record
  | Struct
  | Primitive
  | Vec
  | Slice
  | CyclicSlice
  | Array
  | Tuple
  | Mor
  | ThickFp
  | AssociatedAny
  | ThisAny
  | TargetOutputAny
  | SpatialPlaceholderAny
  | BoxAny
  | HigherKind
  | Ref
  | Option
  deriving DecidableEq

namespace TyKind
instance : Enumerable TyKind where
    enumeration := [
        Enum,
        Record,
        Struct,
        Primitive,
        Vec,
        Slice,
        CyclicSlice,
        Array,
        Tuple,
        Mor,
        ThickFp,
        AssociatedAny,
        ThisAny,
        TargetOutputAny,
        SpatialPlaceholderAny,
        BoxAny,
        HigherKind,
        Ref,
        Option
    ]
    hvalid := sorry
end TyKind

-- impl From<TyKeyword> for TyKind {
--     fn from(keyword: TyKeyword) -> Self {
--         match keyword {
--             TyKeyword::Struct => TyKind::Struct,
--             TyKeyword::Enum => TyKind::Enum,
--             TyKeyword::Record => TyKind::Record,
--         }
--     }
-- }

inductive MemberKind 
   | Field
   | Method ( is_lazy: Bool ) 
   | Call
   | TraitAssociatedType
   | TraitAssociatedConstSize
   | TraitAssociatedAny

inductive EntityKind 
   | Module
   | EtherealTerm (kind: TyKind)
   | Trait
   | Member(kind: MemberKind)
   | Function (requires_lazy: Bool)
   | Feature
   | EnumVariant
   | Main

structure Cat where
  height : Float
  weight : Float

structure House where
    window : Window
    front_door : Door
    back_door : Door