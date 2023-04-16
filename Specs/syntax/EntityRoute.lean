structure TemporalArgument
  deriving DecidableEq

structure EntityRoutePtr where
  raw: Nat
  deriving DecidableEq

inductive EntityRouteVariant
  | Root
  | Child
  deriving DecidableEq

inductive SpatialArgument
  | EtherealTerm (route : EntityRoutePtr)
  | ConstUsize (value : Nat)
  deriving DecidableEq
  
structure EntityRoute where
  variant : EntityRouteVariant
  temporal_arguments: List TemporalArgument
  spatial_arguments: List SpatialArgument
  deriving DecidableEq

namespace SpatialArgument
end SpatialArgument

namespace EntityRouteVariant
end EntityRouteVariant

structure EtherealTerm
  deriving DecidableEq