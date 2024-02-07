use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;
use futures::stream::StreamExt;
use std::io::{stdout, Write};

pub struct GPTClient {
    pub client: ChatGPT,
}

impl GPTClient {
    pub async fn new(api_key: String) -> Result<Self> {
        let client = ChatGPT::new(api_key)?;
        Ok(Self { client })
    }

    pub async fn send_message(&self, message: &str) -> Result<CompletionResponse> {
        self.client.send_message(message).await
    }

    pub async fn send_message_streaming(&self, message: &str) -> Result<String> {
        let mut stream = self.client.send_message_streaming(message).await?;
        let mut result = String::new();

        while let Some(chunk) = stream.next().await {
            match chunk {
                ResponseChunk::Content {
                    delta,
                    response_index: _,
                } => {
                    print!("{}", delta);
                    stdout().lock().flush().unwrap();
                    result.push_str(&delta);
                }
                _ => {}
            }
        }

        Ok(result)
    }
}
