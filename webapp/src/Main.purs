module Main where

import Prelude

import Data.Maybe (maybe)
import Data.String as S
import Effect (Effect)
import Effect.Exception (throw)
import React.Basic.DOM (render)
import Web.DOM.NonElementParentNode (getElementById)
import Web.HTML (window)
import Web.HTML.HTMLDocument (toNonElementParentNode)
import Web.HTML.Location (host)
import Web.HTML.Window (document, location)
import WebApp (webApp)

main :: Effect Unit
main = do
  host <- window >>= location >>= host
  -- cut the port
  let ip = maybe host (flip S.take host) $ S.lastIndexOf (S.Pattern ":") host

  -- fixme: port configurable
  lookupElement' "webapp" >>= render (webApp $ "ws://" <> ip <> ":9001")

  where
    lookupElement id = getElementById id =<< (map toNonElementParentNode $ document =<< window)
    lookupElement' id = lookupElement id >>= maybe (throw $ "element with id: "<>id<>" not found") pure
