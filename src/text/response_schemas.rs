use crate::config::prompt::Message;
use serde::Deserialize;
use std::fmt::Debug;

use super::request_schemas::Content;

// OpenAi
#[derive(Debug, Deserialize)]
pub(super) struct OpenAiResponse {
    pub choices: Vec<MessageWrapper>,
}

#[derive(Debug, Deserialize)]
pub(super) struct MessageWrapper {
    pub message: Message,
}

impl From<OpenAiResponse> for String {
    fn from(value: OpenAiResponse) -> Self {
        value.choices.first().unwrap().message.content.to_owned()
    }
}

// Anthropic
#[derive(Debug, Deserialize)]
pub(super) struct AnthropicMessage {
    pub text: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub _type: String,
}

impl From<AnthropicResponse> for String {
    fn from(value: AnthropicResponse) -> Self {
        value.content.first().unwrap().text.to_owned()
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct AnthropicResponse {
    pub content: Vec<AnthropicMessage>,
}

// Ollama
#[derive(Debug, Deserialize)]
pub(super) struct OllamaResponse {
    pub message: Message,
}

impl From<OllamaResponse> for String {
    fn from(value: OllamaResponse) -> Self {
        value.message.content
    }
}

#[derive(Deserialize, Debug)]
pub struct SafetyRating {
    #[allow(dead_code)]
    pub category: String,
    #[serde(rename = "probability")]
    #[allow(dead_code)]
    pub probability: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct Candidate {
    pub content: Content,
    #[serde(rename = "finishReason")]
    #[allow(dead_code)]
    pub finish_reason: Option<String>,
    #[allow(dead_code)]
    pub index: usize,
    #[serde(rename = "safetyRatings")]
    #[allow(dead_code)]
    pub safety_ratings: Vec<SafetyRating>,
}

#[derive(Deserialize, Debug)]
pub struct UsageMetadata {
    #[serde(rename = "promptTokenCount")]
    #[allow(dead_code)]
    pub prompt_token_count: usize,
    #[serde(rename = "candidatesTokenCount")]
    #[allow(dead_code)]
    pub candidates_token_count: usize,
    #[serde(rename = "totalTokenCount")]
    #[allow(dead_code)]
    pub total_token_count: usize,
}

#[derive(Deserialize, Debug)]
pub(super) struct GeminiResponse {
    pub candidates: Vec<Candidate>,
    #[serde(rename = "usageMetadata")]
    #[allow(dead_code)]
    pub usage_metadata: UsageMetadata,
}

impl From<GeminiResponse> for String {
    fn from(response: GeminiResponse) -> Self {
        response.candidates.into_iter()
            .map(|candidate| candidate.content.parts.into_iter().map(|part| part.text).collect::<Vec<String>>().join("\n"))
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}
