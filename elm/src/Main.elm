module Main exposing (main)

import Browser
import Html exposing (Html, div)
import Html.Attributes exposing (class)
import Html.Events exposing (onMouseEnter, onMouseLeave)
import List.Extra as LE
import Ports exposing (WindowSettings, windowSettings)



---- MODEL ----


type alias Model =
    { windowSettings : Maybe WindowSettings
    , entries : List (List Entry)
    }


type alias Entry =
    Bool


init : ( Model, Cmd Msg )
init =
    ( initialModel, Cmd.none )


initialModel : Model
initialModel =
    { windowSettings = Nothing
    , entries = []
    }



---- UPDATE ----


type Msg
    = WindowSettingsReceived WindowSettings
    | ToggleState ( Int, Int, Bool )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        WindowSettingsReceived ({ width, height } as settings) ->
            let
                roundedWidth =
                    ceiling <| (width / 20)

                roundedHeight =
                    ceiling <| (height / 20)

                singleList =
                    List.repeat roundedWidth False

                list =
                    List.repeat roundedHeight singleList
            in
            ( { model | windowSettings = Just settings, entries = list }, Cmd.none )

        ToggleState ( i, j, state ) ->
            let
                row =
                    model.entries
                        |> LE.getAt i
                        |> Maybe.withDefault []

                newRow =
                    LE.updateAt j (\_ -> state) row

                newEntries =
                    LE.updateAt i (\_ -> newRow) model.entries
            in
            ( { model | entries = newEntries }, Cmd.none )



---- VIEW ----


view : Model -> Html Msg
view { entries } =
    div [ class "wrapper" ]
        (List.indexedMap (\i row -> viewRow row i) entries)


viewRow : List Entry -> Int -> Html Msg
viewRow row i =
    div [ class "row" ]
        (List.indexedMap (\j cell -> viewCell cell i j) row)


viewCell : Entry -> Int -> Int -> Html Msg
viewCell cell i j =
    let
        classes =
            if cell == True then
                "cell cell--active"

            else
                "cell"
    in
    div
        [ class classes
        , onMouseEnter (ToggleState ( i, j, True ))
        , onMouseLeave (ToggleState ( i, j, False ))
        ]
        []



---- SUBSCRIPTIONS ----


subscriptions : Model -> Sub Msg
subscriptions _ =
    windowSettings WindowSettingsReceived



---- PROGRAM ----


main : Program () Model Msg
main =
    Browser.element
        { view = view
        , init = \_ -> init
        , update = update
        , subscriptions = subscriptions
        }
