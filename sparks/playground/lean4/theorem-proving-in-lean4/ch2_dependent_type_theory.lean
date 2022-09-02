/- Define some constants. -/

def m: Nat := 1
def n: Nat := 0
def b1: Bool := true
def b2: Bool := false

/- Check their types -/

#check m
#eval m
#check n + 0
#eval n + 0
#eval b1 && b2
#eval b1 || b2
#check true
#eval 5 * 4

#check Nat → Nat
#check Nat -> Nat
#check Nat × Nat
#check Prod Nat Nat
#check Nat -> Nat -> Nat
#check Nat -> (Nat -> Nat)

#check Nat.succ
#check Nat.add

#check (5, 9).1

def α : Type := Nat
def β : Type := Bool
def F : Type -> Type := List
def G : Type -> Type -> Type := Prod

#check α
#check F α
#check F Nat
#check G α

#check Type
#check Type 1
#check List

def f (n : Nat) : String := toString n
def g (s : String) : Bool := s.length > 0

#check fun x : Nat => x
#check λ x : Nat => x + 5
#check fun x : Nat => true
#check fun x : Nat => g (f x)
#check fun x => g (f x)
