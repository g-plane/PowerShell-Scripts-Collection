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
    & filter (\x -> ".mp4" `isSuffixOf` x || ".mkv" `isSuffixOf` x)
    & map (dropEnd 4)
    & map (++ ".ass")

subtitles :: [FilePath] -> [FilePath]
subtitles = filter
  (\x ->
    ".ass" `isSuffixOf` x && not ("cht" `isInfixOf` x || "TC" `isInfixOf` x)
  )
