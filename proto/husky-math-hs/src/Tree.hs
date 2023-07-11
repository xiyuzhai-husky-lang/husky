module Tree where

data Tree a = Tree { a :: a, children :: [Tree a] }

data TreePattern = TreePattern [Int]

eval :: (a -> [v] -> v) -> Tree a -> v
eval f (Tree a children) = f a (eval f <$> children)