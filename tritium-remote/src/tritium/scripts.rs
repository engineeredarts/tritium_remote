use crate::error::TritiumError;
use crate::graphql::mutations::manually_trigger_script::{
    manually_trigger_script,
    manually_trigger_script::ManuallyTriggerScriptManuallyTriggerScriptScript,
    ManuallyTriggerScript,
};
use crate::graphql::QueryOperation;
use crate::tritium::Tritium;

impl Tritium {
    pub async fn start_script(&mut self, script_path: &str) -> Result<Script, TritiumError> {
        let input = manually_trigger_script::ScriptTriggerInput {
            action: manually_trigger_script::ScriptTriggerAction::START,
            path: script_path.to_string(),
        };
        let operation =
            QueryOperation::<ManuallyTriggerScript>::new(manually_trigger_script::Variables {
                input,
            });
        let query = self.client.graphql_query(operation).await?;
        let response = query.result.await?;

        // TODO - generic way to extract data or return errors
        if let Some(errors) = response.errors {
            return Err(TritiumError::from(errors));
        }

        match response.data {
            Some(data) => match data.manually_trigger_script.script {
                Some(script) => Ok(Script::from(script)),
                _ => Err(TritiumError::GenericError(
                    "GraphQL response contained no script object".to_string(),
                )),
            },
            _ => Err(TritiumError::GenericError(
                "GraphQL response contained no data".to_string(),
            )),
        }
    }

    pub async fn stop_script(&mut self, script_path: &str) -> Result<Script, TritiumError> {
        let input = manually_trigger_script::ScriptTriggerInput {
            action: manually_trigger_script::ScriptTriggerAction::STOP,
            path: script_path.to_string(),
        };
        let operation =
            QueryOperation::<ManuallyTriggerScript>::new(manually_trigger_script::Variables {
                input,
            });
        let query = self.client.graphql_query(operation).await?;
        let response = query.result.await?;

        // TODO - generic way to extract data or return errors
        if let Some(errors) = response.errors {
            return Err(TritiumError::from(errors));
        }

        match response.data {
            Some(data) => match data.manually_trigger_script.script {
                Some(script) => Ok(Script::from(script)),
                _ => Err(TritiumError::GenericError(
                    "GraphQL response contained no script object".to_string(),
                )),
            },
            _ => Err(TritiumError::GenericError(
                "GraphQL response contained no data".to_string(),
            )),
        }
    }
}

pub struct Script {
    // path: Option<String>,
    #[allow(dead_code)]
    pub status: Option<ScriptStatus>,
}

pub enum ScriptStatus {
    LOADED,
    RUNNING,
    ERROR,
}

impl From<manually_trigger_script::ScriptStatus> for ScriptStatus {
    fn from(status: manually_trigger_script::ScriptStatus) -> ScriptStatus {
        match status {
            manually_trigger_script::ScriptStatus::LOADED => ScriptStatus::LOADED,
            manually_trigger_script::ScriptStatus::RUNNING => ScriptStatus::RUNNING,
            _ => ScriptStatus::ERROR,
        }
    }
}

impl From<ManuallyTriggerScriptManuallyTriggerScriptScript> for Script {
    fn from(script: ManuallyTriggerScriptManuallyTriggerScriptScript) -> Script {
        Script {
            // path: script.path,
            status: script.status.map(ScriptStatus::from),
        }
    }
}
