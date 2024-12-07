import Lean.Elab.Tactic
import Lean.Log

open Lean
open Lean.Elab.Tactic

elab "mymy" : tactic => do
  let goal ← getMainGoal
  let goalType ← goal.getType
  logInfoAt (← getRef) m!"Current goal: {goalType}"

example : 2 + 2 = 4 := by
  mymy
  rfl
