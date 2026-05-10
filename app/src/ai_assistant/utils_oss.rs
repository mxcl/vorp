#[derive(Clone)]
pub struct TranscriptPart {
    user_prompt: String,
    assistant_answer: String,
}

impl TranscriptPart {
    pub fn raw_user_prompt(&self) -> &str {
        self.user_prompt.as_str()
    }

    pub fn raw_assistant_answer(&self) -> &str {
        self.assistant_answer.as_str()
    }
}
