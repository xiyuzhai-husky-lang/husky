module Lib (someFunc) where

someFunc :: IO ()
someFunc = do
  putStrLn $ show $ map_simple [2, 3]
  putStrLn $ show $ initial_parse [LiteralToken $ Int 1, LiteralToken $ Int 2]
  putStrLn $ show $ newExprArena [LiteralToken $ Int 1, LiteralToken $ Int 2]
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

newtype ExprArena = ExprArena [ExprArenaEntry]
  deriving Show

data ExprArenaEntry = ExprArenaEntry { normal:: Maybe Expr, extra:: Maybe Expr }
  deriving Show

data ExprIdx = Normal Int | Extra Int

newExprArena:: [Token] -> [ExprArenaEntry]
newExprArena tokens = map (\_ -> ExprArenaEntry { normal = Nothing, extra = Nothing }) tokens

getExpr:: ExprArena -> ExprIdx -> Maybe Expr
getExpr (ExprArena arena) (Normal idx) = normal $ arena!!idx
getExpr (ExprArena arena) (Extra idx) = extra $ arena!!idx

allocExprs:: ExprArena -> [Maybe Expr] -> (ExprArena, [Maybe ExprIdx])
allocExprs (ExprArena arena) new_exprs =
  let new_arena = map allocExpr (zip arena new_exprs) in
  undefined

allocExpr:: (ExprArenaEntry, Maybe Expr) -> ExprArenaEntry
allocExpr (ExprArenaEntry{normal, extra}, Just expr) =
  case normal of
    Just _ -> undefined
    Nothing -> ExprArenaEntry {normal=Just expr, extra}
allocExpr (entry, Nothing) = entry

attention:: ([(Float, v)]-> o) -> [k-> Float] -> [k] -> [v]-> [o]
attention f qs ks vs =
  map (\q -> f $ zip (map q ks) vs) qs