inductive PrimitiveTy

structure f32 where
  raw : Float
 
structure f64 where
  raw : Float

structure i32
  deriving DecidableEq

structure i64
  deriving DecidableEq

structure r32
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

structure HuskyTyUniverse where
  raw : Nat

inductive HuskyTy
  | PrimitiveTy (ty : PrimitiveTy)
  | Struct (fields : List (String × HuskyTy))
  | TypeTy (u : HuskyTyUniverse)

def HuskyTy.universe : HuskyTy -> HuskyTyUniverse :=
  sorry
  -- | PrimitiveTy _ => { raw := 0 }
  -- | Struct [] => { raw := 0 }
  -- | Struct (field::fields) => {
  --   raw := max field.2.universe.raw (Struct fields).universe.raw
  -- }
  -- | TypeTy u => { raw := u.raw + 1 }

def HuskyTy.type_ty (ty : HuskyTy) : HuskyTy := HuskyTy.TypeTy ty.universe

inductive X
  | None
  | Single (x : X)
  | Many (xs : List X)

def X.n1 : X -> Nat
  | None => 0
  | Single x => x.n1
  | Many xs => xs.length

-- def X.n2 : X -> Nat
--   | None => 0
--   | Single x => x.n2
--   | Many xs => match xs with
--     | [] => 0
--     | x::xs => match (xs.map X.n2).maximum? with
--       | some v => max x.n2 v
--       | none => x.n2

def X.n2 : X -> Nat
  | None => 0
  | Single x => x.n2
  | Many [] => 0
  | Many (x::xs) => max x.n2 (Many xs).n2