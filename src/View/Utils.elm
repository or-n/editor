module View.Utils exposing (..)


import Update exposing (..)
import View.Style exposing (..)

import Element exposing (..)
import Element.Border as Border
import Element.Input as Input

import Http exposing (Error(..))


type alias View = Element Message


button : Color -> { onPress : Maybe Message, label : View } -> View
button borderColor =
    Input.button
        [ focused []
        , padding 8
        , Border.width 2
        , Border.color borderColor
        , mouseOver [ Border.color blue ]
        ]


viewError : Http.Error -> View
viewError error =
    case error of
        BadUrl x ->
            text ("BadUrl: " ++ x)

        Timeout ->
            text "Timeout"

        NetworkError ->
            text "NetworkError"

        BadStatus errorCode ->
            text ("BadStatus: " ++ String.fromInt errorCode)

        BadBody _ ->
            text "BadBody"
