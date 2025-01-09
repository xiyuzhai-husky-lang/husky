structure MiracleState where
  vector : List Nat

structure MiracleConfig where
  norm_max : Nat

structure Miracle where
  state : MiracleState
  config : MiracleConfig

namespace Miracle
def push (miracle: Miracle) (i: Nat) : Miracle :=
  { miracle with state := { miracle.state with vector := i :: miracle.state.vector } }

def is_valid (miracle: Miracle) : Bool :=
  miracle.state.vector.sum <= miracle.config.norm_max

end Miracle

structure MiracleM (γ: Type -> Type) (α : Type) where
  exec_batch : {o : Type} -> Miracle × (List (Miracle × α -> γ o)) -> γ o

instance [Monad γ] [inst: Alternative γ] : Monad <| MiracleM γ where
  pure x := {
    exec_batch := fun (miracle, rs) =>
      if miracle.is_valid then
        let rs := rs.enum
        rs.foldr (fun (i, r) acc => r (miracle.push i, x) <|> acc) failure
      else
        failure
  }
  bind := fun m f => {
    exec_batch := fun (miracle, rs) =>
      m.exec_batch (miracle, [fun (miracle, a) => (f a).exec_batch (miracle, rs)])
  }
