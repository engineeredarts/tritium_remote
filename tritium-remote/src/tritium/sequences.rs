use crate::error::TritiumError;
use crate::graphql::mutations::play_sequence::{play_sequence, PlaySequence};
use crate::graphql::QueryOperation;
use crate::tritium::Tritium;

impl Tritium {
    pub async fn play_sequence(
        &mut self,
        project_path: &str,
    ) -> Result<PlayingSequence, TritiumError> {
        let input = play_sequence::PlaySequenceInput {
            project_path: project_path.to_string(),
        };
        let operation = QueryOperation::<PlaySequence>::new(play_sequence::Variables { input });
        let query = self.client.graphql_query(operation).await?;
        let response = query.result.await?;

        // TODO - generic way to extract data or return errors
        if let Some(errors) = response.errors {
            return Err(TritiumError::from(errors));
        }

        match response.data {
            Some(data) => Ok(PlayingSequence::from(data.play_sequence)),
            _ => Err(TritiumError::GenericError(
                "GraphQL response contained no data".to_string(),
            )),
        }
    }
}

pub struct PlayingSequence {
    #[allow(dead_code)]
    pub id: String,
}

impl From<play_sequence::PlaySequencePlaySequence> for PlayingSequence {
    fn from(p: play_sequence::PlaySequencePlaySequence) -> PlayingSequence {
        PlayingSequence { id: p.id }
    }
}
