def m: Nat := 1
def f (n : Nat) : String := toString n
def g (s : String) : Bool := s.length > 0

#check fun x : Nat => x
#check fun x : Nat => true
#check fun x : Nat => g (f x)
#check fun x => g (f x)