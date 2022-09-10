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

namespace SpatialArgument
end SpatialArgument

namespace EntityRouteVariant
end EntityRouteVariant