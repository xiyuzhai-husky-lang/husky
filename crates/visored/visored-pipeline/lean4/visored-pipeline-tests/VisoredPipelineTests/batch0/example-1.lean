import Mathlib
import Obvious
open Obvious

def h(a : ℝ)(b : ℝ) := by
  have h1 : (a + b) ^ 2 ≥ 0 := by obvious
  obvious
