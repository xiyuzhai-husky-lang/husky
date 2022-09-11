structure TemporalArgument
  deriving DecidableEq
  
mutual
inductive EntityRoute
  | _ (variant : EntityRouteVariant)
      (temporal_arguments: List TemporalArgument)
      (spatial_arguments: List SpatialArgument)

inductive SpatialArgument
  | Ty (route : EntityRoute)
  | ConstUsize (value : Nat)

inductive EntityRouteVariant
  | Root
  | Child
end

mutual
instance : DecidableEq EntityRoute := by
  sorry

def SpatialArgument.deq (a b : EntityRoute) : Decidable (a = b) := by
  sorry

def EntityRouteVariant.deq (a b : EntityRoute) : Decidable (a = b) := by
  sorry
end

namespace SpatialArgument
end SpatialArgument

namespace EntityRouteVariant
end EntityRouteVariant

structure RangedEntityRoute
  deriving DecidableEq