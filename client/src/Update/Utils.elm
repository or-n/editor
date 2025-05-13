module Update.Utils exposing (..)


import Model exposing (..)

import Bytes exposing (Bytes)
import Bytes.Decode as Decode exposing (..)

import Http


noCmd : model -> (model, Cmd msg)
noCmd x =
    (x, Cmd.none)


listStep : Decoder a -> (Int, List a) -> Decoder (Step (Int, List a) (List a))
listStep decoder (n, xs) =
  if n <= 0 then
    succeed (Done (List.reverse xs))
  else
    map (\x -> Loop (n - 1, x :: xs)) decoder


decodePair decodeA decodeB =
    decodeA
    |> Decode.andThen (\a -> decodeB |> Decode.andThen (\b -> succeed (a, b)))


decodeBytes : Int -> Decoder (List Int)
decodeBytes n =
    loop (n, []) (listStep Decode.unsignedInt8)


toList : Bytes -> List Int
toList bytes =
    decode (decodeBytes (Bytes.width bytes)) bytes
    |> Maybe.withDefault []


handleBytes : Http.Response Bytes -> Result Http.Error Bytes
handleBytes response =
    case response of
        Http.BadUrl_ url ->
            Err (Http.BadUrl url)

        Http.Timeout_ ->
            Err Http.Timeout

        Http.NetworkError_ ->
            Err Http.NetworkError

        Http.BadStatus_ metadata body ->
            Err (Http.BadStatus metadata.statusCode)

        Http.GoodStatus_ metadata body ->
            Ok body


httpBody : Result Http.Error Bytes
            -> model
            -> (model -> Bytes -> model)
            -> Result Http.Error model
httpBody body model f =
        case body of
            Ok bytes ->
                Ok <| f model bytes

            Err error ->
                Err error

type Method
    = Get
    | Post
    | Put
    | Delete


toString method =
    case method of
        Get ->
            "GET"
        
        Post ->
            "POST"
        
        Put ->
            "PUT"

        Delete ->
            "DELETE"


httpRequest : Method -> String -> (Result Http.Error Bytes -> msg) -> Cmd msg
httpRequest method url response =
        Http.request
            { method = toString method
            , headers = []
            , url = url
            , body = Http.emptyBody
            , expect = Http.expectBytesResponse response handleBytes
            , timeout = Nothing
            , tracker = Nothing
            }
