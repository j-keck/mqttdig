module Message where

import Prelude
import SystemTime (SystemTime)
import Foreign as F
import Partial.Unsafe (unsafePartial)
import Simple.JSON (class ReadForeign)
import Simple.JSON as JSON
import Web.File.Blob (Blob)


type Message =
  { ts :: SystemTime
  , topic :: String
  , payload :: Payload
  }

data Payload =
    Json String
  | Text String
  | Numeric Number
  | Binary Blob


instance showPayload :: Show Payload where
  show = case _ of
    Json s -> s
    Text s -> s
    Numeric n -> show n
    Binary _ -> "<Binary>"

instance readForeignPayload :: ReadForeign Payload where
  readImpl f = toPayload <$> JSON.readImpl f
    where toPayload :: { "type" :: String, content :: F.Foreign } -> Payload
          toPayload { "type": t, content } = unsafePartial $ case t of
            "Json" -> Json $ F.unsafeFromForeign content
            "Numeric" -> Numeric $ F.unsafeFromForeign content
            "Text" -> Text $ F.unsafeFromForeign content
            "Binary" -> Binary $ F.unsafeFromForeign content
