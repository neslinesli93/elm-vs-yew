port module Ports exposing (WindowSettings, windowSettings)


type alias WindowSettings =
    { width : Float
    , height : Float
    }


port windowSettings : (WindowSettings -> msg) -> Sub msg
