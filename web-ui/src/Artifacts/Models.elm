module Artifacts.Models exposing (..)
import Dict


import Regex

import JsonRpc exposing (RpcError)


spacePat : Regex.Regex
spacePat = Regex.regex " "

artifactValidPat : Regex.Regex
artifactValidPat = Regex.regex "^(REQ|SPC|RSK|TST)(-[A-Z0-9_-]*[A-Z0-9_])?$"


-- pretty much only used when updating artifacts
type alias ArtifactId =
  Int

-- the standard lookup method for artifacts
type alias NameKey = 
  String

type alias Loc =
  { path : String
  , row : Int
  , col : Int
  }

type alias Name =
  { raw: String
  , value: String
  }

-- How artifacts are stored
type alias Artifacts = Dict.Dict NameKey Artifact

initialArtifacts : Artifacts
initialArtifacts =
  Dict.empty


-- representation of an Artifact object
type alias Artifact =
  { id : ArtifactId
  , name : Name
  , path : String
  , text : String
  , partof : List Name
  , parts : List Name
  , loc : Maybe Loc
  , completed : Float
  , tested : Float
  , config : ArtifactConfig
  , edited : Maybe ArtifactEditable
  }

-- Editable part of an artifact
type alias ArtifactEditable =
  { name : Name
  , path : String
  , text : String
  , partof : List Name
  }

-- gets the edited variable of the artifact
-- or creates the default one
getEdited : Artifact -> ArtifactEditable
getEdited artifact =
  case artifact.edited of
    Just e -> e
    Nothing ->
      { name = artifact.name
      , path = artifact.path
      , text = artifact.text
      , partof = artifact.partof
      }


type alias ArtifactConfig =
  { partsExpanded : Bool
  , partofExpanded : Bool
  , pathExpanded : Bool
  , locExpanded : Bool
  , textExpanded : Bool
  }

type alias ArtifactsResponse =
  { result: Maybe (List Artifact)
  , error: Maybe RpcError
  }

defaultConfig : ArtifactConfig
defaultConfig =
  { partsExpanded = False
  , partofExpanded = False
  , pathExpanded = False
  , locExpanded = False
  , textExpanded = False
  }

artifactsUrl : String
artifactsUrl =
  "#artifacts" 

artifactNameUrl : String -> String
artifactNameUrl name =
  "#artifacts/" ++ name


-- get the real name from a raw name
indexNameUnchecked : String -> String
indexNameUnchecked name =
  let
    replaced = Regex.replace Regex.All spacePat (\_ -> "") name
  in
    String.toUpper replaced

-- get the real name from a raw name
-- return Err if name is invalid
indexName : String -> Result String String
indexName name =
  let
    index = indexNameUnchecked name
  in
    if Regex.contains artifactValidPat index then
      Ok index
    else
      Err ("Invalid name: " ++ name)

initName : String -> Result String Name
initName name =
  let
    value = indexName name
  in
    case value of
      Ok value -> Ok <|
        { raw = name
        , value = value
        }
      Err err ->
        Err err

-- convert a list of artifacts to a dictionary
artifactsFromList : List Artifact -> Artifacts
artifactsFromList artifacts =
  let
    pairs = List.map (\a -> ( a.name.value, a )) artifacts
  in
    Dict.fromList pairs
