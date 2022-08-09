module Main where

import           System.Environment (getArgs)
import           Text.ParserCombinators.Parsec hiding (spaces)

main :: IO ()
main = do
  args <- getArgs
  putStrLn ("Hello, " ++ head args)
  putStrLn ("Hello, " ++ args !! 1)

symbol :: Parser Char
symbol = oneOf "!$%&|*+-/:<=?>@^_~#"

readExpr :: String -> String
readExpr input = case parse symbol "lisp" input of
  Left err  -> "No match: " ++ show err
  Right val -> "Found value"