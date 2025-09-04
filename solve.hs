import Control.Exception
import Data.Char
import Data.Foldable
import Data.Function
import Data.Functor
import Data.List
import Data.List.Split
import Data.Maybe
import Data.Void (Void)
import Debug.Trace
import System.Exit
import Text.Megaparsec
import Text.Megaparsec.Char
import Text.Megaparsec.Char.Lexer
import Text.Megaparsec.Char.Lexer qualified as L

type Parser = Parsec Void String

newtype SwapP = SwapP (Int, Int) deriving (Show)

newtype SwapL = SwapL (Char, Char) deriving (Show)

newtype RotateS = RotateS (Bool, Int) deriving (Show)

newtype RotateP = RotateP Char deriving (Show)

newtype Reverse = Reverse (Int, Int) deriving (Show)

newtype Move = Move (Int, Int) deriving (Show)

data Line = SwapP_ SwapP | SwapL_ SwapL | RotateS_ RotateS | RotateP_ RotateP | Reverse_ Reverse | Move_ Move deriving (Show)

j a = string a & s

skipSpace = L.space space1 empty empty

integer :: Parser Int = read <$> some numberChar & s

ch = s anySingle

s :: Parser a -> Parser a
s = L.lexeme skipSpace

main :: IO ()
main = do
  contents :: String <- readFile "src/inp.txt"
  let nums = map digitToInt contents
  print (length nums)
  let w = take (length nums) (windows (nums ++ nums) ((length nums `div` 2) + 1)) -- or 2
  let e = [head x | x <- w, head x == last x]

  print (sum e)

swapTwo f s xs =
  zipWith
    ( \x y ->
        if x == f
          then xs !! s
          else
            if x == s
              then xs !! f
              else y
    )
    [0 ..]
    xs

windows c n = (\x -> take n (drop x c)) <$> [n .. length c - n]

rotateL n xs
  | n >= 0 = take (length xs) $ drop n $ cycle xs
  | otherwise = rotateL (length xs + n) xs

rotateR n = rotateL (-n)
