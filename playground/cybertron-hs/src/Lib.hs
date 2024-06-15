module Lib (someFunc) where

someFunc :: IO ()
someFunc = do
  putStrLn $ show $ map_simple [2, 3]
  putStrLn $ show $ initial_parse [LiteralToken $ Int 1, LiteralToken $ Int 2]
  putStrLn "someFunc"

map_simple :: [Int] -> [Int]
map_simple is = map (+ 1) is

data Expr =
  LiteralExpr Literal
  | BinaryExpr Expr BinaryOpr Expr
  deriving Show

data Literal = Int Int | Float Float | String String
  deriving Show

data BinaryOpr = Add | Sub
  deriving Show

data Token = LiteralToken Literal
  | OprToken Opr
  deriving Show

data Opr = BinaryOpr BinaryOpr 
  deriving Show

initial_parse:: [Token] -> [Either Opr Expr]
initial_parse tokens = map (
  \token ->  case token of
    LiteralToken lit -> Right $ LiteralExpr lit
    OprToken opr -> Left opr
  ) tokens