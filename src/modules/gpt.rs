use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{self, ChatCompletionRequest},
    common::GPT4_O,
};

pub struct Gpt {
    client: OpenAIClient,
}

impl Gpt {
    pub fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = OpenAIClient::builder().with_api_key(api_key).build()?;
        Ok(Self { client })
    }

    pub async fn ask(&self, question: &str) -> Result<String, Box<dyn std::error::Error>> {
        let req = ChatCompletionRequest::new(
            GPT4_O.to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(question.to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }],
        );

        let result = self.client.chat_completion(req).await?;
        let response_content = result.choices[0].message.content.clone().unwrap();

        Ok(response_content)
    }
}
