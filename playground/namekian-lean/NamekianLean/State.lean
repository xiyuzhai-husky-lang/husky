import NamekianLean.Registry
-- import NamekianLean.Registry

structure LogicState where
  registry:  LogicRegistry
  intros: List LogicIntro
  tree: LogicTree
