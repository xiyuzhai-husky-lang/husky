
import           Test.HUnit (assertEqual, runTestTT, Counts(failures, errors)
                           , Test(TestCase), Testable(test))
import           System.Exit (exitFailure, exitSuccess)

testNonZeroCase :: Test
testNonZeroCase = TestCase (assertEqual "haha" ((1 :: Integer) + 1) 2)

main :: IO ()
main = do
  test_counts <- runTestTT (test [testNonZeroCase])
  if errors test_counts + failures test_counts == 0
    then exitSuccess
    else exitFailure