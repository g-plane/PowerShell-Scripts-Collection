module Lib
  ( names
  , subtitles
  )
where

import           Data.Function
import           Data.List
import           Data.List.Extra
import           System.IO                      ( FilePath )

names :: [FilePath] -> [String]
names xs =
  xs
    & filter (\x -> isSuffixOf ".mp4" x || isSuffixOf ".mkv" x)
    & map (dropEnd 4)
    & map (++ ".ass")

subtitles :: [FilePath] -> [FilePath]
subtitles = filter
  (\x -> isSuffixOf ".ass" x && not (isInfixOf "cht" x || isInfixOf "TC" x))
