#![allow(clippy::all, warnings)]
pub struct ManuallyTriggerScript;
pub mod manually_trigger_script {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ManuallyTriggerScript";
    pub const QUERY : & str = "mutation ManuallyTriggerScript($input: ManuallyTriggerScriptInput!) {\n    manuallyTriggerScript(input: $input) {\n        script {\n            status\n        }\n    }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive()]
    pub enum ScriptTriggerAction {
        START,
        STOP,
        Other(String),
    }
    impl ::serde::Serialize for ScriptTriggerAction {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                ScriptTriggerAction::START => "START",
                ScriptTriggerAction::STOP => "STOP",
                ScriptTriggerAction::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScriptTriggerAction {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "START" => Ok(ScriptTriggerAction::START),
                "STOP" => Ok(ScriptTriggerAction::STOP),
                _ => Ok(ScriptTriggerAction::Other(s)),
            }
        }
    }
    #[derive()]
    pub enum ScriptStatus {
        ERROR,
        LOADED,
        RUNNING,
        Other(String),
    }
    impl ::serde::Serialize for ScriptStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                ScriptStatus::ERROR => "ERROR",
                ScriptStatus::LOADED => "LOADED",
                ScriptStatus::RUNNING => "RUNNING",
                ScriptStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScriptStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "ERROR" => Ok(ScriptStatus::ERROR),
                "LOADED" => Ok(ScriptStatus::LOADED),
                "RUNNING" => Ok(ScriptStatus::RUNNING),
                _ => Ok(ScriptStatus::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct ManuallyTriggerScriptInput {
        pub path: String,
        pub action: ScriptTriggerAction,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub input: ManuallyTriggerScriptInput,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "manuallyTriggerScript")]
        pub manually_trigger_script: Option<ManuallyTriggerScriptManuallyTriggerScript>,
    }
    #[derive(Deserialize)]
    pub struct ManuallyTriggerScriptManuallyTriggerScript {
        pub script: Option<ManuallyTriggerScriptManuallyTriggerScriptScript>,
    }
    #[derive(Deserialize)]
    pub struct ManuallyTriggerScriptManuallyTriggerScriptScript {
        pub status: Option<ScriptStatus>,
    }
}
impl graphql_client::GraphQLQuery for ManuallyTriggerScript {
    type Variables = manually_trigger_script::Variables;
    type ResponseData = manually_trigger_script::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: manually_trigger_script::QUERY,
            operation_name: manually_trigger_script::OPERATION_NAME,
        }
    }
}
