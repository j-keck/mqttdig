module WebApp where

import Components.LiveStream (liveStream)
import Components.Publish (publish)
import Data.Function ((#))
import Data.Unit (Unit, unit)
import React.Basic (Component, JSX, createComponent, makeStateless)
import React.Basic.DOM as R

webApp :: String -> JSX
webApp url = unit # makeStateless component \props ->
    R.div_
    [ publish
    , liveStream { url }
    ]

  where
    component :: Component Unit
    component = createComponent "WebApp"

