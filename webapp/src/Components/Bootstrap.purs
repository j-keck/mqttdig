module Components.Bootstrap.Card where

import React.Basic (Component, JSX, createComponent, makeStateless)
import React.Basic.DOM as R


type Props =
  { title :: String
  , body :: JSX
  }

card :: Props -> JSX
card = makeStateless component \props ->
  R.div
  { className: "card mt-3"
  , children:
    [ R.div
      { className: "card-header p-1"
      , children: [ R.text props.title ]
      }
    , R.div
      { className: "card-body p-1"
      , children: [ props.body ]
      }
    ]
  }

  where

    component :: Component Props
    component = createComponent "Card"


