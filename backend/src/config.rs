use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    // Ollama配置（可选）
    pub ollama_url: Option<String>,
    pub ollama_model: Option<String>,
    pub ollama_timeout_secs: u64,
    // OpenAI兼容API配置（可选，用于备用或其他AI服务）
    pub openai_api_key: Option<String>,
    pub openai_api_base: Option<String>,
    pub openai_model: Option<String>,
    // AI提供商选择: "ollama" 或 "openai" 或 "none"（不启用AI）
    pub ai_provider: Option<String>,
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
            // Ollama配置（可选）
            ollama_url: std::env::var("OLLAMA_URL").ok(),
            ollama_model: std::env::var("OLLAMA_MODEL").ok(),
            ollama_timeout_secs: std::env::var("OLLAMA_TIMEOUT_SECS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            // OpenAI配置（可选）
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            openai_api_base: std::env::var("OPENAI_API_BASE").ok(),
            openai_model: std::env::var("OPENAI_MODEL").ok(),
            // AI提供商（如果未配置，AI功能将不可用）
            ai_provider: std::env::var("AI_PROVIDER").ok(),
            rfc_base_url: std::env::var("RFC_BASE_URL")
                .unwrap_or_else(|_| "https://www.rfc-editor.org/rfc/".to_string()),
        })
    }
}
