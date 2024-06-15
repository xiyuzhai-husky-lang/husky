def hello := "world"

def boolAttention
  (f: List V -> O)
  (qs: List (K -> Bool))
  (ks: List K)
  (vs: List V): List O :=
  qs.map (fun q => f $ List.filterMap (fun (k,v) => if q k then some v else none) (ks.zip vs))


#eval (
  let xs : List Nat := [1,2,3,4,1,3]
  let qs : List (Nat -> Bool) := xs.map fun x => fun y => x == y
  let ks := xs
  let vs := xs
  boolAttention List.length qs ks vs
)

def floatAttention
  (f: List (Float × V) -> O)
  (qs: List (K -> Float))
  (ks: List K)
  (vs: List V): List O :=
  qs.map (fun q => f $ (ks.zip vs).map (fun (k,v) => (q k, v)))
