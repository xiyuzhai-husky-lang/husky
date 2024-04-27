module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn "someFunc"


data Animal = Dog | Cat | Bird

data Animals = Animals {a1:: Animal, a2::Animal}

feedAnimal :: Animal -> Bool
feedAnimal $ Animals (a@(Dog|Cat))  (b@Cat) = undefined
feedAnimal _ = undefined