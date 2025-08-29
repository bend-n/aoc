import Data.Foldable
import Data.List
import Data.List.Split

main :: IO ()
main = do
  contents :: String <- readFile "src/inp.txt"
  let g = sort (map read . splitOn "-" <$> lines contents) :: [[Int]]
  let x = foldl' (\acc (a : b : _) -> if acc `elem` [a .. b] then b + 1 else acc) 0 g :: Int
  print x

windows c n = (\x -> take n (drop x c)) <$> [n .. length c - n]