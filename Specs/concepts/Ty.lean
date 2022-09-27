inductive PrimitiveTy

structure f32 where
  raw : Float
 
structure f64 where
  raw : Float

structure i32
  deriving DecidableEq

structure i64
  deriving DecidableEq

structure b32
  deriving DecidableEq

structure b64 where
  raw : UInt64
  deriving DecidableEq

structure OrderedFloat (α : Type) where
  -- data : α
  -- valid : not nan
  deriving DecidableEq

structure OrderedF32
  deriving DecidableEq
structure OrderedF64
  deriving DecidableEq

structure TyUniverse where
  raw : Nat

structure NonEmptyList (α : Type) where
  a : α
  as : List α

theorem haha : sizeOf α < sizeOf (NonEmptyList α) := sorry

namespace NonEmptyList
def maxf [LT β] [DecidableRel (@LT.lt β _)] (list : NonEmptyList α) (f : α -> β) : β :=
  match (list.as.map f).maximum? with
  | some v => max v (f list.a)
  | none => f list.a
end NonEmptyList

inductive Ty
  | PrimitiveTy (ty : PrimitiveTy)
  | Struct (fields : NonEmptyList (String × Ty))
  | TypeTy (u : TyUniverse)

def Ty.universe : Ty -> TyUniverse
  | PrimitiveTy _ => { raw := 0 }
  | Struct fields => { raw := (fields.maxf fun field => field.2.universe.raw) }
  | TypeTy u => { raw := u.raw + 1 }
  termination_by Ty.universe ty => ty

def Ty.type_ty (ty : Ty) : Ty := Ty.TypeTy ty.universe