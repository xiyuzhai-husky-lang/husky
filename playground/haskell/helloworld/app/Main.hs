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