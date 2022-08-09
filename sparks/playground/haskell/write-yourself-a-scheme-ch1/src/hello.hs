module Main where

import           System.Environment

main :: IO ()
main = do
  args <- getArgs
  putStrLn ("Hello, " ++ head args)
  putStrLn ("Hello, " ++ head args)