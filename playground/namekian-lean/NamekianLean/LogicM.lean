import NamekianLean.State

structure LogicM where
  runLogicM : LogicState -> a × LogicState
