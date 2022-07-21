module Main where

import           Lib
import           GHC.Natural
import           Data.Word

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

increment n = n + 1

decrement m = m - 1

add n m = if m /= 0
          then add (increment n) (decrement m)
          else n

type Document = [Structure]

data Structure = Heading Natural String
               | Paragraph String
               | UnorderedList [String]
               | OrderedList [String]
               | CodeBlock [String]

parseLines :: [String] -> [String] -> Document
parseLines currentParagraph txts =
  let paragraph = Paragraph (unlines (reverse currentParagraph)) -- (2), (3)
  in case txts    -- (4)
      of
       [] -> [paragraph]
       currentLine:rest -> if trim currentLine == ""
                           then paragraph:parseLines [] rest -- (5)
                           else parseLines (currentLine:currentParagraph) rest -- (6)

trim :: String -> String
trim = unwords . words

data Color = RGB Word8 Word8 Word8

getBluePart :: Color -> Word8
getBluePart color = case color of
  RGB _ _ blue -> blue

data Brightness = Dark
                | Bright

data EightColor =
    Black
  | Red
  | Green
  | Yellow
  | Blue
  | Magenta
  | Cyan
  | White

data AnsiColor = AnsiColor Brightness EightColor

ansiColorToVGA :: AnsiColor -> Color
ansiColorToVGA ansicolor = case ansicolor of
  AnsiColor Dark Black -> RGB 0 0 0
  AnsiColor Bright Black -> RGB 85 85 85
  AnsiColor Dark Red -> RGB 170 0 0
  AnsiColor Bright Red -> RGB 255 85 85
  _ -> RGB 0 0 0

safeHead :: [a] -> Maybe a
safeHead list = case list of
  -- Empty list
  []  -> Nothing
  -- Cons cell pattern, will match any list with at least one element
  x:_ -> Just x

exactlyTwo :: [a] -> Maybe (a, a)
exactlyTwo list = case list of
  -- Will match a list with exactly two elements
  [x, y] -> Just (x, y)
  -- Will match any other pattern
  _      -> Nothing

-- This will also work
exactlyTwoVersion2 :: [a] -> Maybe (a, a)
exactlyTwoVersion2 list = case list of
  -- Will match a list with exactly two elements
  [x, y] -> Just (x, y)
  -- Will match any other pattern
  _      -> Nothing

parse :: String -> Document
parse = parseLines [] . lines