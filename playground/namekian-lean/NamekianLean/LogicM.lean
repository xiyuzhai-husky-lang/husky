import NamekianLean.State

structure LogicM (a: Type) where
  runLogicM : LogicState -> a × LogicState

instance: Monad LogicM where
  pure a :=  { runLogicM := fun s => (a, s) }
  bind a_m f := {
    runLogicM := fun s =>
      let (a, s) := a_m.runLogicM s
      let rec b_m := f a
      (b_m.runLogicM s)
  }
