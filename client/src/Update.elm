module Update exposing (..)


import Model exposing (..)
import Update.Utils exposing (..)
import Update.API exposing (..)

import Browser exposing (UrlRequest(..))
import Browser.Navigation as Nav

import Http
import Url exposing (Url)
import Keyboard exposing (Key)
import Element exposing (Device)
import Bytes exposing (Bytes)
import Task


type Message
    = Url Url
    | UrlRequest UrlRequest
    | KeyMsg Keyboard.Msg
    | Resize Resolution
    | API API_Message
    | UpdateForm String
    | Login String
    | GetLoginStatus (Result Http.Error Bytes)


update : Message -> Model -> (Model, Cmd Message)
update message model = case message of

    API apiMessage ->
        case model.mode of
            Edit editModel ->
                case api apiMessage editModel of
                     Ok (newModel, command) ->
                         ({ model | mode = Edit newModel }, Cmd.map API command)
                     
                     Err (Http.BadStatus 400) ->
                         noCmd { model | mode = NoAuth "" }
                     
                     Err _ ->
                         noCmd model
            
            NoAuth username ->
                noCmd model
    
    UpdateForm newUsername ->
        noCmd { model | mode = NoAuth newUsername }

    Login username ->
        ( model
        , httpRequest Get ("/login/" ++ username) GetLoginStatus
        )

    GetLoginStatus _ ->
        ( { model | mode = Edit editEmpty }
        , Task.perform API (Task.succeed GoToTop)
        )

    Url x ->
        noCmd model
    
    UrlRequest x ->
        ( model
        , case x of
            Internal url ->
                Nav.pushUrl model.key (Url.toString url)

            External href ->
                Nav.load href
        )
    
    KeyMsg msg ->
        let
            keyUpdates = Keyboard.updateWithKeyChange Keyboard.anyKeyOriginal
            
            (pressedKeys, change) = keyUpdates msg model.pressedKeys
        in
            noCmd { model | pressedKeys = pressedKeys }
    
    Resize resolution ->
        noCmd
            { model
            | device = Element.classifyDevice resolution
            , resolution = resolution
            }
