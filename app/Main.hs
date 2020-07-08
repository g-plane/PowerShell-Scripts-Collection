module Main where

import           Lib
import           System.Directory

main :: IO ()
main = do
  entries <- listDirectory "."
  mapM_ (uncurry renameFile) $ zip (subtitles entries) (names entries)
