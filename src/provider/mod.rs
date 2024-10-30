use async_trait::async_trait;
use groq::GroqProvider;
use openai::OpenAIProvider;
use phind::PhindProvider;

use crate::{error::LumenError, git_commit::GitCommit, ProviderType};

pub mod groq;
pub mod openai;
pub mod phind;

#[async_trait]
pub trait AIProvider {
    async fn explain(&self, commit: GitCommit) -> Result<String, Box<dyn std::error::Error>>;
}

pub enum LumenProvider {
    OpenAI(Box<OpenAIProvider>),
    Phind(Box<PhindProvider>),
    Groq(Box<GroqProvider>),
}

impl LumenProvider {
    pub fn new(
        client: reqwest::Client,
        provider_type: ProviderType,
        api_key: Option<String>,
        model: Option<String>,
    ) -> Result<Self, LumenError> {
        match provider_type {
            ProviderType::Openai => {
                let api_key = api_key.ok_or(LumenError::MissingApiKey("OpenAI".to_string()))?;
                let provider =
                    LumenProvider::OpenAI(Box::new(OpenAIProvider::new(client, api_key, model)));
                Ok(provider)
            }
            ProviderType::Phind => Ok(LumenProvider::Phind(Box::new(PhindProvider::new(
                client, None,
            )))),
            ProviderType::Groq => {
                let api_key = api_key.ok_or(LumenError::MissingApiKey("Groq".to_string()))?;
                let provider =
                    LumenProvider::Groq(Box::new(GroqProvider::new(client, api_key, model)));
                Ok(provider)
            }
        }
    }
}

#[async_trait]
impl AIProvider for LumenProvider {
    async fn explain(&self, commit: GitCommit) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            LumenProvider::OpenAI(provider) => provider.explain(commit).await,
            LumenProvider::Phind(provider) => provider.explain(commit).await,
            LumenProvider::Groq(provider) => provider.explain(commit).await,
        }
    }
}
