inductive Animal
  | Dog (height : Nat)
  | Cat

def Animal.getName : Animal -> String
  | Dog _ => "Dog"
  | Cat => "Cat"

def Animal.isEqual : Animal -> Animal -> Bool
  -- Dog
  | Dog _, Dog _ => True
  | Dog _, Cat => False

  -- Cat
  | Cat, Cat => True
  | Cat, Dog _ => False

#eval Animal.getName Animal.Cat
