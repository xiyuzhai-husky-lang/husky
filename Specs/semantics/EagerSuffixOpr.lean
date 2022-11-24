import Specs.syntax.EntityRoute

inductive PurePattern
  deriving DecidableEq

inductive EagerSuffixOpr
  | Incr                           -- ++
  | Decr                           -- --
  | AsTy (ty : Term)  -- :
  | BePattern (patt : PurePattern) -- be <patt>
  | Unveil                         -- ?
  deriving DecidableEq