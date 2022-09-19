class Projected (Receiver : Type) where
  -- where
  receiver_dec_eq : DecidableEq Receiver
  -- impl

class Projectable (α : Type) where
  Projectile : Type
  Projection : Type
  State : Type
  -- where
  projection_is_projected : Projected Projection
  sender_is_decidable_eq : DecidableEq α
  -- impl
  project : State -> α -> State × Projection

structure Projector (α : Type) [Projectable α] where
  state : Projectable.State α
  value : α

namespace Projector
def project {α : Type} [Projectable α] : (Projector α) -> Projector α × Projectable.Projection α
  | projector =>
    let ( state, change ) := Projectable.project projector.state projector.value
    ⟨ ⟨ state, projector.value ⟩, change ⟩
end Projector