module SystemTime where


import Prelude
import Data.DateTime (DateTime)
import Data.DateTime.Instant (instant, toDateTime)
import Data.Either (fromRight)
import Data.Formatter.DateTime (format, parseFormatString)
import Data.Maybe (fromJust)
import Data.Newtype (class Newtype, unwrap)
import Data.Time.Duration (Milliseconds(..))
import Partial.Unsafe (unsafePartial)
import Simple.JSON (class ReadForeign)
import Simple.JSON as F

newtype SystemTime = SystemTime DateTime

instance showSystemTime :: Show SystemTime where
  show = format fmt <<< unwrap
    where fmt = unsafePartial $ fromRight <<< parseFormatString $ "ddd MMM DD HH:mm:ss YYYY"


derive newtype instance eqSystemTime :: Eq SystemTime

derive instance newtypeSystemTime :: Newtype SystemTime _

instance readForeignSystemTime :: ReadForeign SystemTime where
  readImpl f = toSystemTime <$> F.readImpl f
   where toSystemTime :: { secs_since_epoch :: Number, nanos_since_epoch :: Number } -> SystemTime
         toSystemTime obj =
           let ms = Milliseconds $ (obj.secs_since_epoch * 1000.0) + (obj.nanos_since_epoch / 1000000.0)
           in SystemTime $ toDateTime $ unsafePartial $ fromJust $ instant ms

