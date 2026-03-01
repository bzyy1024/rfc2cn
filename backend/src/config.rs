use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    // Ollama配置
    pub ollama_url: String,
    pub ollama_model: String,
    // OpenAI兼容API配置（可选，用于备用或其他AI服务）
    pub openai_api_key: Option<String>,
    pub openai_api_base: String,
    pub openai_model: String,
    // AI提供商选择: "ollama" 或 "openai"
    pub ai_provider: String,
    // RFC抓取配置
    pub rfc_base_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: std::env::var("DATABASE_URL")?,
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            // Ollama默认配置
            ollama_url: std::env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            ollama_model: std::env::var("OLLAMA_MODEL")
                .unwrap_or_else(|_| "qwen2.5:14b".to_string()),
            // OpenAI配置
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            openai_api_base: std::env::var("OPENAI_API_BASE")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
            openai_model: std::env::var("OPENAI_MODEL")
                .unwrap_or_else(|_| "gpt-4-turbo-preview".to_string()),
            // 默认使用Ollama
            ai_provider: std::env::var("AI_PROVIDER")
                .unwrap_or_else(|_| "ollama".to_string()),
            rfc_base_url: std::env::var("RFC_BASE_URL")
                .unwrap_or_else(|_| "https://www.rfc-editor.org/rfc/".to_string()),
        })
    }
}
