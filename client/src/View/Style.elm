module View.Style exposing (..)


import Element exposing (Color, rgb255, rgb)
import Element.Font as Font exposing (Font)


firaCode : Font
firaCode =
    Font.external
        { url = "https://fonts.googleapis.com/css2?family=Fira+Code"
        , name = "Fira Code"
        }


firaCodeFamily : Element.Attribute msg
firaCodeFamily = 
    Font.family [ firaCode ]


darkPurple : Color
darkPurple =
    rgb255 16 8 18


white : Color
white =
    rgb 1 1 1


light : Color
light =
    rgb 0.9 0.9 0.9


red : Color
red =
    rgb 1 0 0


green : Color
green =
    rgb 0 1 0


blue : Color
blue =
    rgb 0 0 1

black =
    rgb 0 0 0

edges : { top : number, right : number, bottom : number, left : number }
edges =
    { top = 0
    , right = 0
    , bottom = 0
    , left = 0
    }


mainContentPadding : number
mainContentPadding = 64
