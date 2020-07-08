module Lib
  ( names
  , subtitles
  )
where

import qualified Data.Text                     as T
import           System.IO                      ( FilePath )

names :: [FilePath] -> [String]
names =
  map (T.unpack . T.dropEnd 4)
    . filter
        (\x -> T.isSuffixOf (T.pack ".mp4") x || T.isSuffixOf (T.pack ".mkv") x)
    . map T.pack

subtitles :: [FilePath] -> [FilePath]
subtitles =
  map T.unpack
    . filter
        (\x -> T.isSuffixOf (T.pack ".ass") x
          && not (T.isInfixOf (T.pack "cht") x || T.isInfixOf (T.pack "TC") x)
        )
    . map T.pack
