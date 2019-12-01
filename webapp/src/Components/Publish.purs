module Components.Publish where

import Prelude

import Components.Bootstrap.Card (card)
import Data.Array as A
import Data.Int (fromString)
import Data.List (List(..), mapWithIndex, (!!), (:))
import Data.List as L
import Data.Maybe (Maybe(..), fromMaybe, maybe)
import Data.Tuple (Tuple(..))
import Effect (Effect)
import Effect.Aff (launchAff_)
import Effect.Exception (throw)
import Http as Http
import Partial.Unsafe (unsafePartial)
import React.Basic (Component, JSX, Self, createComponent, fragment, make)
import React.Basic.DOM as R
import React.Basic.DOM.Events (capture, capture_, targetValue)

type Topic = String
type Message = String

type State = { topic :: Maybe Topic, message :: Maybe Message, history :: List (Tuple Topic Message) }

publish :: JSX
publish = unit # make component { initialState, render }
  where

    component :: Component Unit
    component = createComponent "Publish"

    initialState = { topic: Nothing, message: Nothing, history: Nil }

    render self =
      card
      { title: "Publish"
      , body: fragment
        [ R.div
          { className: "input-group"
          , children:
            [ R.input
              { className: "form-control"
              , placeholder: "Topic"
              , onChange: capture targetValue \topic -> self.setState _ { topic = topic }
              , value: fromMaybe "" self.state.topic
              }
            , R.input
              { className: "form-control"
              , placeholder: "Message"
              , onChange: capture targetValue \message -> self.setState _ { message = message }
              , value: fromMaybe "" self.state.message
              }
            , R.div
              { className: "input-group-append"
              , children: [ R.button { className: "btn btn-outline-secondary"
                                     , children: [R.text "Publish" ]
                                     , onClick: capture_ $ update self $ Publish
                                     }
                          ]
              }
            ]
          }
        , R.div
          { className: "input-group"
          , children:
            [ R.select
              { className: "custom-select"
              , onChange: capture targetValue \idx ->
                  maybe (throw "not an int?!?") (update self <<< SelectFromHistory) $ fromString =<< idx
              , value: "0"
              , children: A.fromFoldable $
                            mapWithIndex (\idx (Tuple t m) -> R.option
                                                             { value: show idx
                                                             , children:
                                                               [ R.text $ t <> " @ " <> m ]
                                                             }) self.state.history
              }
            ]
          }
        ]
      }


data Action =
    Publish
  | SelectFromHistory Int

update :: Self Unit State -> Action -> Effect Unit
update self = case _ of

  SelectFromHistory idx -> unsafePartial $
    let Just (Tuple topic message) = self.state.history !! idx
    in self.setState _ { topic = Just topic, message = Just message }

  Publish -> do
    case Tuple <$> self.state.topic <*> self.state.message of
      Just t@(Tuple topic message) ->
        self.setStateThen (\s -> s { history = t : L.filter ((/=) t) s.history }) $
        launchAff_ $ Http.post_ "/api/publish" {topic: topic, message: message}
      Nothing -> pure unit
