import Mathlib
import Obvious
open Obvious

def h(x : ℝ)(y : ℝ)(h1 : x = 1)(h2 : y = 1) : x + y = 2 := by
  have h3 : 1 + 1 = 2 := by obvious
  obvious
