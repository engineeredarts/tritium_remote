use crate::error::TritiumError;
use crate::graphql::mutations::manually_trigger_script::{
    manually_trigger_script,
    manually_trigger_script::ManuallyTriggerScriptManuallyTriggerScriptScript,
    ManuallyTriggerScript,
};
use crate::graphql::QueryOperation;
use crate::tritium::Tritium;

impl Tritium {
    pub async fn start_script(
        &mut self,
        script_path: &str,
    ) -> Result<ManuallyTriggerScriptManuallyTriggerScriptScript, TritiumError> {
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

        match response.data {
            Some(data) => match data.manually_trigger_script.script {
                Some(script) => Ok(script),
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
