module Main where

import           Lib

main :: IO ()
main = putStrLn myhtml

myhtml = wrapHtml "Hello, world!"

wrapHtml content = "<html><body>" <> content <> "</body></html>"

el :: String -> String -> String
el tag content = "<" <> tag <> ">" <> content <> "</" <> tag <> ">"

html_ :: String -> String
html_ = el "html"

body_ :: String -> String
body_ = el "body"

four = (\num1 num2 -> num1 + num2 + 1) 1 2

newtype Html = Html String

data Person = Person String Int -- where the first is the name and the second is
                                -- the age

data Person2 = Person2 { name :: String, age :: Int }

a = Person2 "Gil" 32

data Person3 = Person3 { pName :: String, pAge :: Int }

data Tuple a b = Tuple a b

t1 = Tuple "Clicked" True

t2 = Tuple 'a' 'z'

data Either a b = Left a
                | Right b