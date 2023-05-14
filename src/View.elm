module View exposing (..)


import Model exposing (..)
import Update exposing (..)
import Update.API exposing (..)
import Update.Utils exposing (..)
import View.Style exposing (..)
import View.Utils exposing (..)

import Element exposing (..)
import Element.Background as Background
import Element.Font as Font
import Element.Border as Border
import Element.Input as Input

import Hex.Convert as Hex
import Html exposing (Html)
import Dict exposing (Dict)


view : Model -> Html Message
view model =
    Element.layout
        [ Background.color darkPurple
        , firaCodeFamily
        , Font.color light
        ]
    <| el
        [ centerX
        , centerY
        , width (px 900)
        , height fill
        , paddingXY mainContentPadding 0
        , Border.width 2
        ]
    <| viewStageContents model


viewStageContents : Model -> View
viewStageContents model =
    case model.mode of
        Edit editorModel ->
            viewEditor model editorModel
        
        NoAuth username ->
            viewLoginPage model username


viewLoginPage : Model -> String -> View
viewLoginPage model username =
    column [ centerX, spacing 40 ]
        [ text "Login" |> el [ Font.size 32, padding 100, centerX ] 
        , Input.username [ Font.color black, Border.width 0 ]
            { onChange = UpdateForm
            , text = username
            , placeholder = Nothing
            , label = Input.labelLeft [ padding 10 ] (text "username")
            }
        , button green
            { onPress = Just (Login username)
            , label = text "Submit"
            }
        ]


viewEditor : Model -> API_Model -> View
viewEditor model editorModel =
    case editorModel.stage of
         NotLoaded ->
             text "Not Loaded"
         
         TopNodes top_nodes ->
             viewTopNodes model editorModel top_nodes
         
         Node node inputs ->
             viewNode model editorModel node inputs
        
         ParsingError ->
            text "Parsing Error"


viewTopNodes : Model -> API_Model -> List Address -> View
viewTopNodes model editorModel top_nodes =
    let
        header =
            text "Top Nodes"
            |> el [ Font.size 32 ]
        
        footer =
            button red
                { onPress = Just (API Clear)
                , label = text "-"
                }
    in
        mainContent model editorModel header footer top_nodes


viewNode : Model -> API_Model -> Address -> List Address -> View
viewNode model editorModel node inputs =
    let
        header =
            row [ Font.size 24, spacing 10 ]
                [ button green
                    { onPress = Just (API <| GoToTop)
                    , label = text "/"
                    }
                , button green
                    { onPress = Just (API <| Save node)
                    , label = text (getName editorModel.names node)
                    }
                ]

        viewMemory address =
            row [ spacing 10 ]
                [ button green
                    { onPress = Just (API <| Create <| inputs ++ [address])
                    , label = text "+"
                    }
                , text ("Saved: " ++ Hex.toString address)
                ]
        
        footer =
            Maybe.withDefault none
            <| Maybe.map viewMemory editorModel.memory
    in
        mainContent model editorModel header footer inputs


mainContent : Model -> API_Model -> View -> View -> List Address -> View
mainContent model editorModel header footer nodes =
    column [ spacing 10, height fill ]
        [ el [ paddingXY 0 mainContentPadding ] header
        , viewAddresses editorModel nodes
        , Input.text [ alignBottom, Font.color black, Border.width 0 ]
            { onChange = API << UpdateName
            , text = editorModel.name
            , placeholder = Nothing
            , label = Input.labelLeft [ padding 10 ] (text "text")
            }
        , el [ alignBottom, paddingXY 0 mainContentPadding ] footer
        ]


viewAddresses : API_Model -> List Address -> View
viewAddresses model addresses =
    if List.isEmpty addresses then
        text "-"
    else
        List.indexedMap (viewAddress model) addresses
        |> column [ spacing 10 ]


getName : Names -> Address -> String
getName names address =
    case Dict.get (toList address) names of
        Just (Just x) ->
            x
        _ ->
            Hex.toString address


viewAddress : API_Model -> Int -> Address -> View
viewAddress model index address =
    row []
        [ text (String.fromInt index ++ ": ")
        , button green
            { onPress = Just (API <| Visit address)
            , label = text <| getName model.names address
            }
        , button green
            { onPress = Just (API <| ToggleName address model.name)
            , label = text "rename"
            }
        ]
