import Mathlib
import Obvious
open Obvious

def h(a : ℝ)(b : ℝ) : (a + b) ^ 2 ≥ 0 := by
  have h1 : (a + b) ^ 2 ≥ 0 := by obvious
  obvious
