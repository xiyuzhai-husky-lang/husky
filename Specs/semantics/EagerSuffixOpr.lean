import Specs.syntax.EntityRoute

inductive PurePattern
  deriving DecidableEq

inductive EagerSuffixOpr
  | Incr                           -- ++
  | Decr                           -- --
  | AsTy (ty : EtherealTerm)  -- :
  | BePattern (patt : PurePattern) -- be <patt>
  | Unveil                         -- ?
  deriving DecidableEq