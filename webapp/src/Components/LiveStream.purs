module Components.LiveStream where

import Prelude

import Data.Array as A
import Data.Bifunctor (lmap)
import Data.Either (Either, either, fromRight)
import Data.Formatter.DateTime (format, parseFormatString)
import Data.List.NonEmpty as LNE
import Data.Maybe (Maybe(..), fromMaybe, isJust, maybe)
import Data.Monoid (guard)
import Data.Newtype (unwrap)
import Data.Sequence (Seq)
import Data.Sequence as Seq
import Data.String as S
import Effect.Console (log)
import Effect.Exception (throw)
import Effect.Var (($=))
import Foreign as F
import Message (Message)
import Partial.Unsafe (unsafePartial)
import React.Basic (Component, JSX, createComponent, fragment, make, readState)
import React.Basic.DOM as R
import React.Basic.DOM.Events (capture, capture_, targetValue)
import Simple.JSON (class ReadForeign, readJSON)
import SystemTime (SystemTime)
import WebSocket (Connection(..), URL(..), newWebSocket, runMessage, runMessageEvent)

type Props = { url :: String }
type State = { messages :: Seq Message, topicFilter :: Maybe String }

liveStream :: Props -> JSX
liveStream = make component { initialState, didMount, didUpdate, render }
  where

    component :: Component Props
    component = createComponent "LiveStream"

    initialState = { messages: Seq.empty, topicFilter: Nothing }

    didMount self = do
      log $ "connect websocket: " <> self.props.url
      Connection ws <- newWebSocket (URL self.props.url) []
      ws.onmessage $= \event -> do
        let raw = runMessage $ runMessageEvent event
        (msg :: Message) <- either throw pure $ decode raw

        state <- readState self
        -- FIXME: configurable cache size
        -- cut message cache to n if we are over (n + 50)
        let messages = if (Seq.length state.messages > 250)
                        then Seq.take 200 state.messages
                        else state.messages

        self.setState _ { messages = Seq.cons msg messages }


    didUpdate self _ =
      guard (self.state.topicFilter == Just "") $ self.setState _ { topicFilter = Nothing }


    render self =
      -- FIXME: case insensitive search
      let messages = maybe (Seq.take 50 self.state.messages)
                     (\p -> Seq.filter (\m -> S.contains (S.Pattern p) m.topic) self.state.messages)
                     self.state.topicFilter in
      fragment
        [ topicFilter self
        , messagesCount self messages
        , messagesTable self messages
        ]


    -- simple topic filter
    topicFilter self =
      R.div
      { className: "input-group mt-3"
      , children:
        [ R.input
              { className: "form-control"
              , placeholder: "Topic filter"
              , onChange: capture targetValue \v -> self.setState _ { topicFilter = v }
              , value: fromMaybe "" self.state.topicFilter
              }
        , R.div
              { className: "input-group-append"
              , children: [ R.button { className: "btn btn-outline-secondary"
                                     , children: [R.text "Clear" ]
                                     , onClick: capture_ $ self.setState _ { topicFilter = Nothing }
                                     }
                          ]
              }
        ]
      }


    -- messages count
    messagesCount self messages =
      R.div
      { className: "small text-muted mt-2 mb-2"
      , style: R.css { height: "15px" }
      , children:
        [ guard (Seq.length self.state.messages /= Seq.length messages) $
          let nOfMessages = show $ Seq.length self.state.messages
              nOfDisplay = show $ Seq.length messages
          in if (isJust self.state.topicFilter)
             then R.text $ nOfDisplay <> " from " <> nOfMessages <> " messages contains the given topic filter pattern"
             else R.text $ "Show only the last " <> nOfDisplay <> " messages from " <> nOfMessages <> " messages"
        ]
      }




    -- messages
    messagesTable self messages =
      R.table
      { className: "table table-hover table-sm"
      , children:
        [ R.thead_ [ R.tr_ $ map (R.text >>> A.singleton >>> R.th_) ["Timestamp (UTC)", "Topic", "Message"] ]
        , R.tbody_ $ flip map (A.fromFoldable messages) \msg ->
           R.tr_
           [ R.td
             { title: unsafeFormatSystemTime "ddd MMM DD HH:mm:ss YYYY" msg.ts
             , children: [ R.text $ unsafeFormatSystemTime "HH:mm:ss" msg.ts ]
             }
           , R.td
             { style: R.css { cursor: "pointer" }
             , title: "Click to apply as topic filter"
             , onClick: capture_ $ self.setState _ { topicFilter = Just msg.topic }
             , children: [ R.text msg.topic ]
             }
           , R.td_ [ R.pre_ [R.text $ show msg.payload ]]
           ]
        ]
      }

    unsafeFormatSystemTime :: String -> SystemTime -> String
    unsafeFormatSystemTime fmt = format (unsafePartial $ fromRight <<< parseFormatString $ fmt) <<< unwrap


decode :: forall a. ReadForeign a => String -> Either String a
decode = lmap (LNE.head >>> F.renderForeignError) <<< readJSON
