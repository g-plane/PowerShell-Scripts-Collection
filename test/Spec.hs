import           Data.Functor
import           Data.Function
import           Data.List
import           Data.List.Extra
import           Test.Hspec
import           Test.QuickCheck
import           Lib

extensionName :: Gen String
extensionName = elements ["mp4", "mkv", "ass"]

baseName :: Gen String
baseName = listOf arbitraryPrintableChar

appendExt :: Gen (String -> String)
appendExt = flip (++) . ('.' :) <$> extensionName

fileName :: Gen String
fileName = appendExt <*> baseName

video2ass :: Gen [String]
video2ass = listOf fileName

lang :: Gen String
lang = elements ["chs", "SC", "cht", "TC"]

appendLang :: Gen (String -> String)
appendLang = flip (++) . ('_' :) . (++ ".ass") <$> lang

subtitlesName :: Gen [String]
subtitlesName = listOf $ appendLang <*> baseName

main :: IO ()
main = hspec $ do
  describe "retrieve video files name" $ it "test" $ property $ forAll
    video2ass
    (\xs ->
      names xs
        == (xs & filter (not . isSuffixOf ".ass") & map (dropEnd 4) & map
             (++ ".ass")
           )
    )

  describe "retrieve subtitle files" $ it "test" $ property $ forAll
    subtitlesName
    (\xs ->
      subtitles xs
        == filter (\x -> not (isInfixOf "cht" x || isInfixOf "TC" x)) xs
    )
