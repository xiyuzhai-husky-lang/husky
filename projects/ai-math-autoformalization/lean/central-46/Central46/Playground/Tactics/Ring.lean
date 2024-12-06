import Mathlib

open Ring

example (R : Type) [Ring R] (a b c : R) : x + 1 = 1 + x := by ring

example (a b c: ℤ) : a * (b + c) = a * b + a * c := by ring

example (R : Type) [Ring R] (a b c : R) : a * (b + c) = a * b + a * c := by
  apply mul_add

example (R : Type) [Ring R] (a b c : R) : a * (b + c) = a * b + a * c := by
  noncomm_ring

example (x: ℤ) : (x + 1)^2 = x^2 + 2*x + 1 := by ring
example (x: ℤ) : (x + 1)^3 = x^3 + 3*x^2 + 3*x + 1 := by ring
example (x: ℤ) : (x + 1)^4 = x^4 + 4*x^3 + 6*x^2 + 4*x + 1 := by ring
example (x: ℤ) : (x + 1)^5 = x^5 + 5*x^4 + 10*x^3 + 10*x^2 + 5*x + 1 := by ring
example (x: ℤ) : (x + 1)^6 = x^6 + 6*x^5 + 15*x^4 + 20*x^3 + 15*x^2 + 6*x + 1 := by ring
example (x: ℤ) : (x + 1)^7 = x^7 + 7*x^6 + 21*x^5 + 35*x^4 + 35*x^3 + 21*x^2 + 7*x + 1 := by ring
example (x: ℤ) : (x + 1)^8 = x^8 + 8*x^7 + 28*x^6 + 56*x^5 + 70*x^4 + 56*x^3 + 28*x^2 + 8*x + 1 := by ring

example (x: ℤ) : (x + 1)^9 = x^9 + 9*x^8 + 36*x^7 + 84*x^6 + 126*x^5 + 126*x^4 + 84*x^3 + 36*x^2 + 9*x + 1 := by ring

example (x y z: ℤ) : (x + y + z)^2 = x^2 + y^2 + z^2 + 2*x*y + 2*y*z + 2*z*x := by ring
example (x y z: ℤ) : (x + y + z)^3 = x^3 + y^3 + z^3 + 3*x^2*y + 3*x*y^2 + 3*y^2*z + 3*y*z^2 + 3*z^2*x + 3*z*x^2 + 6*x*y*z := by ring

example (x: ℤ) (y: ℕ) : (x^y+1)^2 = (x^(2*y) + 2*x^y + 1) := by ring
example (x: ℤ) (y: ℕ) : (x^y+1)^2 = (x^(2*y) + 2*x^y + 1) := by ring

example (x: ℤ) :
  (x+1)^2 = x^2 + 2*x + 1 := by ring

example (x: ℤ) :
  ((((x+1)^100+1)^100+1)^100 + 1)^2 = (((x+1)^100+1)^100+1)^200 + 2*(((x+1)^100+1)^100+1)^100 + 1 := by ring

variables {R : Type*} [CommRing R]
variables (x y z : R)

example : ((x + y + z)^3 - 3*(x + y)*(y + z)*(z + x))^2 - (x^3 + y^3 + z^3)^2 = 0 :=
by ring

example : ((x + y + z)^3 - 3*(x + y)*(y + z)*(z + x))^2 - (x^3 + y^3 + z^3)^2 = 0 := by
  ring
