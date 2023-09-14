use crate::error::TritiumError;
use crate::tritium::Tritium;

pub type Variables = ();
pub type QueryData = String;

impl Tritium {
    pub async fn query(
        &mut self,
        document: &str,
        variables: Option<Variables>,
    ) -> Result<QueryData, TritiumError> {
        todo!()
    }
}
