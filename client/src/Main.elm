module Main exposing (..)


import Browser
import Browser.Events exposing (onResize)

import Model exposing (..)
import View exposing (..)
import Update exposing (..)
import Update.API exposing (..)

import Keyboard exposing (subscriptions)
import Element
import Task


main : Program Flags Model Message
main = Browser.application
    { init = init
    , view = \model ->
        { title = "Voluntary Trade"
        , body = [ view model ]
        }
    , update = update
    , subscriptions = subscriptions
    , onUrlChange = Update.Url
    , onUrlRequest = Update.UrlRequest
    }


type alias Flags = Resolution


init flags url key =
    let
        device = Element.classifyDevice flags
    in
        (   { key = key
            , url = url
            , pressedKeys = []
            , device = device
            , resolution = flags
            , useKeyboard = device.orientation == Element.Landscape
            , mode = Edit editEmpty
            }
        , Task.perform API (Task.succeed GoToTop)
        )


subscriptions : Model -> Sub Message
subscriptions model = Sub.batch
    [ Sub.map KeyMsg Keyboard.subscriptions
    , onResize <|
        \width height ->
            Resize { width = width, height = height }
    ]
