use async_trait::async_trait;
use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;
use futures::stream::StreamExt;
use std::io::{stdout, Write};
use std::result::Result;

pub struct GPTClient {
    pub client: ChatGPT,
}

#[async_trait]
pub trait ChatClient {
    async fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    async fn send_message(
        &self,
        message: &str,
    ) -> Result<CompletionResponse, Box<dyn std::error::Error>>;

    async fn send_message_streaming(
        &self,
        message: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

#[async_trait]
impl ChatClient for GPTClient {
    async fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = ChatGPT::new(api_key)?;
        Ok(Self { client })
    }

    async fn send_message(
        &self,
        message: &str,
    ) -> Result<CompletionResponse, Box<dyn std::error::Error>> {
        Ok(self.client.send_message(message).await?)
    }

    async fn send_message_streaming(
        &self,
        message: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
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
