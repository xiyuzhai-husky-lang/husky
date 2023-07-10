import           Distribution.Simple
import           Text.ParserCombinators.Parsec

main :: IO ()
main = do
  putStrLn
    $ case parse (parseM :: Parser SuchThat) "husky" "such that" of
      Left err  -> "No Match"
      Right val -> "Haha"

class ParseFromStream a where
  parse_from_stream :: Parser a

parseM :: ParseFromStream a => Parser a
parseM = parse_from_stream

data SuchThat = SuchThat {  }

instance ParseFromStream SuchThat where
  parse_from_stream = do
    _ <- string "such that"
    return SuchThat {  }