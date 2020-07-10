module Lib
  ( names
  , subtitles
  )
where

import           Data.Function
import qualified Data.Text                     as T
import           System.IO                      ( FilePath )

names :: [FilePath] -> [String]
names xs =
  xs
    & map T.pack
    & filter
        (\x -> T.isSuffixOf (T.pack ".mp4") x || T.isSuffixOf (T.pack ".mkv") x)
    & map (T.dropEnd 4)
    & map (flip T.append (T.pack ".ass"))
    & map T.unpack

subtitles :: [FilePath] -> [FilePath]
subtitles xs =
  xs
    & map T.pack
    & filter
        (\x -> T.isSuffixOf (T.pack ".ass") x
          && not (T.isInfixOf (T.pack "cht") x || T.isInfixOf (T.pack "TC") x)
        )
    & map T.unpack
