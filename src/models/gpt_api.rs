use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AskGpt {
    pub question: String,
}

#[derive(Deserialize, Serialize)]
pub struct GptAnswer {
    pub answer: String,
}
