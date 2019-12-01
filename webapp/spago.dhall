{-
Welcome to a Spago project!
You can edit this file as you like.
-}
{ name =
    "my-project"
, dependencies =
    [ "affjax"
    , "console"
    , "effect"
    , "formatters"
    , "generics-rep"
    , "psci-support"
    , "react-basic"
    , "sequences"
    , "simple-json"
    , "websocket-simple"
    ]
, packages =
    ./packages.dhall
, sources =
    [ "src/**/*.purs", "test/**/*.purs" ]
}
