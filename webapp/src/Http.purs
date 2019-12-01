module Http where

import Affjax (URL)
import Affjax as A
import Affjax.RequestBody as ARB
import Affjax.ResponseFormat (ResponseFormat)
import Affjax.ResponseFormat as ARF
import Affjax.StatusCode (StatusCode(..))
import Control.Alt ((<$>))
import Control.Bind ((>>=))
import Data.Bifunctor (lmap)
import Data.Either (Either(..))
import Data.Function (const, ($), (<<<), (>>>))
import Data.Functor (map)
import Data.List.NonEmpty as LNE
import Data.Maybe (Maybe(..))
import Data.Monoid ((<>))
import Data.Unit (unit)
import Effect.Aff (Aff)
import Foreign as F
import Prelude (Unit, otherwise, (&&), (<), (==), (>=))
import Simple.JSON (class ReadForeign, class WriteForeign, readJSON, writeJSON)
import Unsafe.Coerce (unsafeCoerce)

-- | performes a get request and returns the response as the given `Affjax.ResponseFormat`
get :: forall a. ResponseFormat a -> URL -> Aff (Either String a)
get rfmt url = interpret <$> A.get rfmt url

-- | performes a get request and decodes the json response
get' :: forall a. ReadForeign a => URL -> Aff (Either String a)
get' url = (_ >>= decode) <$> get ARF.string url

-- | performes a post request with the given payload and returns the response as the given `Affjax.ResponseFormat`
post :: forall a b. WriteForeign a => ResponseFormat b -> URL -> a -> Aff (Either String b)
post rfmt url payload = interpret <$> A.post rfmt url (Just <<< ARB.string <<< writeJSON $ payload)

-- | performes a post request with the given payload and ignores the response
post_ :: forall a. WriteForeign a => URL -> a -> Aff (Either String Unit)
post_ url payload = map (const unit) <$> post ARF.string url payload

-- | decodes the given json string as an instance
decode :: forall a. ReadForeign a => String -> Either String a
decode = lmap (LNE.head >>> F.renderForeignError) <<< readJSON

-- | interprets the server response
interpret :: forall a. Either A.Error (A.Response a) -> Either String a
interpret = case _ of
  Left err -> Left $ A.printError err
  Right r -> handleResponse r.status r.body
    where handleResponse (StatusCode code) body
            | code >= 200 && code < 300 = Right body
            | code == 400              = Left $ "BadRequest: " <> unsafeCoerce body
            | code == 401              = Left $ "Unauthorized"
            | code == 403              = Left $ "Forbidden"
            | code == 404              = Left $ "NotFound"
            | otherwise                = Left $ "ServerError: " <> unsafeCoerce body
