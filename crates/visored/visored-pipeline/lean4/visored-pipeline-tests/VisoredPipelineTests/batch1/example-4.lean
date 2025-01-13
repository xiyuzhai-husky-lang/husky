import Mathlib
syntax "attack" : tactic

macro_rules
| `(tactic| attack) => `(tactic|
  first
  | simp; done
  | ring; done
  | ring_nf; rw [Real.sq_sqrt]; ring; all_goals attack; done
  | nlinarith; done
  | apply sq_nonneg; all_goals attack; done
  | apply div_nonneg; all_goals attack; done
  | field_simp; ring; done
  | linarith; done
)

macro "obvious": tactic =>`(tactic|
  first
  | attack; done
  | congr; all_goals attack; done
  | gcongr; all_goals attack; done
  | fail "Could not prove this goal automatically. It might not be as obvious as you think!"
)

macro "in_set" : term => `(true)

def h (x : ℝ) (h1 : x > (0 : ℝ)) : x + (1 : ℝ) / x ≥ (2 : ℝ) := by
  have h2 : x > (0 : ℝ) := by obvious
  first
  | have h3 : (x - (1 : ℝ)) ^ 2 ≥ (0 : ℝ) := by calc
    (x - (1 : ℝ)) ^ 2 = x ^ 2 - (2 : ℝ) * x + (1 : ℝ) := by obvious
    _ ≥ (0 : ℝ) := by obvious
  | have h4 : x ^ 2 - (2 : ℝ) * x + (1 : ℝ) ≥ (0 : ℝ) := by calc
    x ^ 2 - (2 : ℝ) * x + (1 : ℝ) = (x - (1 : ℝ)) ^ 2 := by obvious
    _ ≥ (0 : ℝ) := by obvious
  have h5 : x ^ 2 - (2 : ℝ) * x + (1 : ℝ) ≥ (0 : ℝ) := by obvious
  have h6 : x ^ 2 + (1 : ℝ) ≥ (2 : ℝ) * x := by obvious
  have h7 : x > (0 : ℝ) := by obvious
  have h8 : x + (1 : ℝ) / x ≥ (2 : ℝ) := by obvious
  obvious
