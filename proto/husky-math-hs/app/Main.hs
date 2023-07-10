{-# LANGUAGE DerivingVia #-}

module HuskyMath.Main (main) where

import           Text.ParserCombinators.Parsec
import           Lib
import           Data.Generics

main :: IO ()
main = do
  putStrLn
    $ case parse (parseM :: Parser SuchThat) "husky" "such that" of
      Left err  -> "No Match"
      Right val -> "Haha"

class ParseM a where
  parseM :: Parser a

data SuchThat = SuchThat {  }

instance ParseM SuchThat where
  parseM = do
    _ <- string "such that"
    return SuchThat {  }