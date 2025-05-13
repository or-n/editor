module Model exposing (..)


import Browser.Navigation as Nav
import Url exposing (Url)
import Keyboard exposing (Key)
import Element exposing (Device)
import Http
import Bytes exposing (Bytes)
import Dict exposing (Dict)


type alias Model =
    { key : Nav.Key
    , url : Url
    , pressedKeys : List Key
    , device : Device
    , resolution : Resolution
    , useKeyboard : Bool
    , mode : Mode
    }


type alias Names = Dict (List Int) (Maybe String)


type alias Resolution =
    { width : Int
    , height : Int
    }


type alias API_Model =
    { stage : Stage
    , memory : Maybe Address
    , names : Names
    , name : String
    }


editEmpty =
    { stage = NotLoaded
    , memory = Nothing
    , names = Dict.empty
    , name = ""
    }


type Mode
    = Edit API_Model
    | NoAuth String


type Stage
    = NotLoaded
    | TopNodes (List Address)
    | Node Address (List Address)
    | ParsingError


type alias Address = Bytes

