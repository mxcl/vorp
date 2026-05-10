/// The key for the corresponding entry in UserDefaults.
pub const REQUEST_LIMIT_INFO_CACHE_KEY: &str = "AIAssistantRequestLimitInfo";

pub enum GenerateDialogueResult {
    Success {
        answer: String,
        truncated: bool,
        request_limit_info: crate::ai::RequestLimitInfo,
        transcript_summarized: bool,
    },
    Failure {
        request_limit_info: crate::ai::RequestLimitInfo,
    },
}
