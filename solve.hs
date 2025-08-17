import Data.List (inits, subsequences)

n = 25

main :: IO Int
main = do
  contents :: String <- readFile "src/inp.txt"
  let i = read <$> lines contents
  let (y : _) = [last x | x <- i `windows` (n + 1), last x `notElem` [sum x | x <- subsequences (take n x), length x == 2]]
  let (a : _) = [x | x <- concatMap inits [drop x i | x <- [0 ..]], sum x == y]
  let y = minimum a + maximum a
  print y
  return y

windows c n = (\x -> take n (drop x c)) <$> [n .. length c - n]