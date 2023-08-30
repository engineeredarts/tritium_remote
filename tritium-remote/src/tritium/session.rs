use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Metadata {
    pub session_type: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
