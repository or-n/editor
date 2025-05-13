module Update.API exposing (..)


import Model exposing (..)
import Update.Utils exposing (..)

import Http

import Bytes exposing (Bytes, Endianness(..))
import Bytes.Decode as Decode exposing (..)

import Hex.Convert as Hex
import Dict
import Task


type API_Message
    = GoToTop
    | Clear
    | Create (List Address)
    | Visit Address
    | ToggleName Address String
    | AskForNames Address
    | GotTopNodes (Result Http.Error Bytes)
    | GotNewNode (List Address) (Result Http.Error Bytes)
    | GotNodeInputs Bytes (Result Http.Error Bytes)
    | GotClearStatus (Result Http.Error Bytes)
    | GotNameChangeStatus Address (Result Http.Error Bytes)
    | GotNodeNames Address (Result Http.Error Bytes)
    | Save Address
    | UpdateName String


decodeAddresses : Int -> Decoder (List Address)
decodeAddresses n =
    loop (n // 16, []) (listStep (Decode.bytes 16))


parseNodes : Bytes -> Maybe (List Address)
parseNodes bytes =
        decode (decodeAddresses (Bytes.width bytes)) bytes


type CreationStatus = AlreadyExisted | Created


parseNode : Bytes -> Maybe (Address, CreationStatus)
parseNode bytes =
        if Bytes.width bytes == 20 then
            decode (decodePair (Decode.bytes 16) decodeCreationStatus) bytes
        else
            Nothing

decodeCreationStatus =
    let
        interpret value = if value == 0 then AlreadyExisted else Created
    in
        Decode.unsignedInt32 BE
        |> Decode.andThen (interpret >> succeed)


tryChangingStage model parse f cmd =
    Result.map
        (  parse
        >> Maybe.map (\x -> ( { model | stage = f x }, cmd x))
        >> Maybe.withDefault (noCmd { model | stage = ParsingError })
        )


api : API_Message -> API_Model -> Result Http.Error (API_Model, Cmd API_Message)
api message model =
    
    case message of    
        GoToTop ->
            Ok
            ( model
            , httpRequest Get "/top" GotTopNodes
            )

        Clear ->
            Ok
            ( model
            , httpRequest Delete "/clear" GotClearStatus
            )

        Save address ->
            Ok <| noCmd { model | memory = Just address }

        UpdateName name ->
            Ok <| noCmd { model | name = name }

        Create newInputs ->
            let
                path =
                    newInputs
                    |> List.reverse
                    |> List.map Hex.toString
                    |> List.intersperse ","
                    |> String.concat
            in
                Ok
                ( model
                , httpRequest Post ("/create/" ++ path) (GotNewNode newInputs)
                )
        
        Visit node ->
                Ok
                ( model
                , httpRequest Get ("/node/" ++ Hex.toString node) (GotNodeInputs node)
                )
        
        ToggleName node name ->
            let
                path = Hex.toString node ++ "/" ++ name
            in
                Ok
                ( model
                , httpRequest Put ("/name/" ++ path) (GotNameChangeStatus node)
                )

        AskForNames node ->
            Ok
            ( model
            , httpRequest Get ("/names/" ++ Hex.toString node) (GotNodeNames node)
            )

        GotNodeNames node body ->
            let
                parseNames bytes =
                    let
                        a = decode (Decode.string (Bytes.width bytes)) bytes
                    in
                        if a == Just "" then Nothing else a
                            |> Maybe.map (String.replace "\n" ", ")

                updateNames value =
                    Dict.insert (toList node) value model.names
            in
                body
                |> Result.map (parseNames >> \value ->
                         noCmd { model | names = updateNames value }
                )

        GotTopNodes body ->
            body |> tryChangingStage model parseNodes TopNodes namesBatch

        GotNewNode inputs body ->
            let
                setStage (node, creationStatus) =
                    case creationStatus of
                        AlreadyExisted ->
                            model.stage

                        Created ->
                            Node node inputs
            in
                body |> tryChangingStage model parseNode setStage (\_ -> Cmd.none)

        GotNodeInputs node body ->
            body |> tryChangingStage model parseNodes (Node node) namesBatch
        
        GotClearStatus body ->
            Ok
            ( { model | stage = NotLoaded, memory = Nothing }
            , httpRequest Get "/top" GotTopNodes
            )

        GotNameChangeStatus node body ->
            Ok
            ( model
            , Task.perform AskForNames (Task.succeed node)
            )
    

namesBatch =
    List.map (Task.perform AskForNames << Task.succeed)
    >> Cmd.batch
