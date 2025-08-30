import Control.Exception
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

integer = read <$> some numberChar & s

ch = s anySingle

swapP :: Parser SwapP
swapP = do
  x <- j "position" *> integer
  y <- j "with position" *> integer
  pure (SwapP (x, y))

swapL :: Parser SwapL
swapL = do
  x <- j "letter" *> ch
  y <- j "with letter" *> ch
  pure (SwapL (x, y))

lr = (j "left" $> True) <|> (j "right" $> False)

rotateS :: Parser RotateS
rotateS = do
  side <- lr
  count <- integer <* (j "steps" <|> j "step")
  pure (RotateS (side, count))

rotateP :: Parser RotateP
rotateP = RotateP <$> (j "based on position of letter" *> ch)

reverse :: Parser Reverse
reverse = do
  x <- j "reverse positions" *> integer
  y <- j "through" *> integer
  pure (Reverse (x, y))

move :: Parser Move = do
  x <- j "position" *> integer
  y <- j "to position" *> integer
  pure (Move (x, y))

s :: Parser a -> Parser a
s = L.lexeme skipSpace

line :: Parser Line =
  s
    (j "swap" *> (SwapP_ <$> swapP <|> SwapL_ <$> swapL))
    <|> (j "rotate" *> (RotateP_ <$> rotateP <|> RotateS_ <$> rotateS))
    <|> (Reverse_ <$> Main.reverse)
    <|> (j "move" *> fmap Move_ move)

-- swap position X with position Y means that the letters at indexes X and Y (counting from 0) should be swapped.
-- swap letter X with letter Y means that the letters X and Y should be swapped (regardless of where they appear in the string).
-- rotate left/right X steps means that the whole string should be rotated; for example, one right rotation would turn abcd into dabc.
-- rotate based on position of letter X means that the whole string should be rotated to the right based on the index of letter X (counting from 0) as determined before this instruction does any rotations. Once the index is determined, rotate the string to the right one time, plus a number of times equal to that index, plus one additional time if the index was at least 4.
-- reverse positions X through Y means that the span of letters at indexes X through Y (including the letters at X and Y) should be reversed in order.
-- move position X to position Y means that the letter which is at index X should be removed from the string, then inserted such that it ends up at index Y.

apply :: String -> Line -> String
apply x (SwapP_ (SwapP (a, b))) = valid x & swapTwo a b
apply x (SwapL_ (SwapL (a, b))) =
  ( \x ->
      ( case x of
          c | c == a -> b
          c | c == b -> a
          c -> c
      )
  )
    <$> valid x
apply x (RotateS_ (RotateS (True, n))) = rotateL n (valid x)
apply x (RotateS_ (RotateS (False, n))) = rotateR n (valid x)
apply x (RotateP_ (RotateP l)) =
  let index = fromMaybe 0 (elemIndex l (valid x))
   in let times = index + 1 + if index >= 4 then 1 else 0
       in rotateR times x
apply l (Reverse_ (Reverse (x, y))) =
  let middle = take (y + 1) (valid l) & drop x & Data.List.reverse
   in take x l ++ middle ++ drop (y + 1) l
apply l (Move_ (Move (x, y))) = do
  let elem = valid l !! x
  let removed = take x l ++ drop (x + 1) l
  take y removed ++ elem : drop y removed

valid l = assert (length l == 8) l

main :: IO ()
main = do
  contents :: String <- readFile "src/inp.txt"

  let x = runParser (many line <* eof) "inp" contents
  x <- case x of
    Left err -> exitFailure
    Right output -> pure (foldl' apply "abcdefgh" output, head [x | x <- permutations "abcdefgh", foldl' apply x output == "fbgdceah"])

  print x

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
