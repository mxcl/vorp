use serde::{Deserialize, Serialize};

use crate::ai::agent::FileLocations;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateAMQuerySuggestionsRequest {
    pub context_messages: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_context: Option<String>,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateAMQuerySuggestionsResponse {
    pub id: String,
    pub suggestion: Option<Suggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Suggestion {
    Simple(SimpleQuery),
    Coding(CodingQuery),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimpleQuery {
    pub query: String,
    pub should_plan_task: bool,
}

impl GenerateAMQuerySuggestionsResponse {
    pub fn is_valid_code_delegation(&self) -> bool {
        matches!(&self.suggestion, Some(Suggestion::Coding(coding_query)) if !coding_query.files.is_empty())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodingQuery {
    pub files: Vec<GeneratedFileLocations>,
    pub query: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GeneratedFileLocations {
    pub file_name: String,
    pub line_numbers: Option<Vec<usize>>,
}

impl From<GeneratedFileLocations> for FileLocations {
    fn from(value: GeneratedFileLocations) -> Self {
        Self {
            name: value.file_name,
            lines: Vec::new(),
        }
    }
}
