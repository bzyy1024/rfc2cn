use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::{AppError, Result};

// =============== Ollama API 结构 ===============

#[derive(Debug, Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_ctx: i32,  // 上下文窗口大小
}

#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: Message,
}

// =============== OpenAI API 结构 ===============

#[derive(Debug, Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

// =============== 翻译系统提示词 ===============

const TRANSLATION_SYSTEM_PROMPT: &str = r#"你是一个专业的RFC文档翻译专家。请将以下英文技术文档翻译成中文。

翻译要求：
1. 保持专业术语的准确性和一致性
2. 不要翻译以下内容：
   - 代码示例和代码块
   - URL和链接
   - RFC编号（如 RFC 6749）
   - HTTP方法名（GET, POST, PUT, DELETE等）
   - HTTP头字段名（Content-Type, Authorization等）
   - 技术标识符和参数名
3. 保持原文档的格式和结构
4. 技术术语应使用业界标准译法
5. 对于没有标准译法的术语，可以保留英文或在括号中附上英文原词

请直接输出翻译结果，不要添加额外的解释或说明。"#;

/// 使用配置的AI提供商翻译文本
pub async fn translate_text(config: &Config, text: &str) -> Result<String> {
    match config.ai_provider.as_str() {
        "ollama" => translate_with_ollama(config, text).await,
        "openai" => translate_with_openai(config, text).await,
        _ => Err(AppError::InternalError(format!(
            "不支持的AI提供商: {}",
            config.ai_provider
        ))),
    }
}

/// 使用Ollama翻译文本
pub async fn translate_with_ollama(config: &Config, text: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/chat", config.ollama_url);

    let request = OllamaChatRequest {
        model: config.ollama_model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: TRANSLATION_SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: text.to_string(),
            },
        ],
        stream: false,
        options: OllamaOptions {
            temperature: 0.3,
            num_ctx: 8192,
        },
    };

    tracing::debug!("正在调用 Ollama API: {}", url);

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(300)) // 5分钟超时
        .send()
        .await
        .map_err(|e| AppError::ExternalApiError(format!("调用 Ollama API 失败: {}", e)))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        return Err(AppError::ExternalApiError(format!(
            "Ollama API 返回错误: {}",
            error_text
        )));
    }

    let chat_response: OllamaChatResponse = response
        .json()
        .await
        .map_err(|e| AppError::ExternalApiError(format!("解析 Ollama API 响应失败: {}", e)))?;

    Ok(chat_response.message.content)
}

/// 使用OpenAI兼容API翻译文本
pub async fn translate_with_openai(config: &Config, text: &str) -> Result<String> {
    let api_key = config
        .openai_api_key
        .as_ref()
        .ok_or_else(|| AppError::InternalError("未配置 OpenAI API Key".to_string()))?;

    let client = reqwest::Client::new();
    let url = format!("{}/chat/completions", config.openai_api_base);

    let request = OpenAIChatRequest {
        model: config.openai_model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: TRANSLATION_SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: text.to_string(),
            },
        ],
        temperature: 0.3,
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| AppError::ExternalApiError(format!("调用 OpenAI API 失败: {}", e)))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        return Err(AppError::ExternalApiError(format!(
            "OpenAI API 返回错误: {}",
            error_text
        )));
    }

    let chat_response: OpenAIChatResponse = response
        .json()
        .await
        .map_err(|e| AppError::ExternalApiError(format!("解析 API 响应失败: {}", e)))?;

    let translated_text = chat_response
        .choices
        .first()
        .map(|choice| choice.message.content.clone())
        .ok_or_else(|| AppError::ExternalApiError("API 响应中没有翻译结果".to_string()))?;

    Ok(translated_text)
}

/// 检查Ollama服务是否可用
pub async fn check_ollama_health(config: &Config) -> Result<bool> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/tags", config.ollama_url);

    match client.get(&url).timeout(std::time::Duration::from_secs(5)).send().await {
        Ok(response) => Ok(response.status().is_success()),
        Err(_) => Ok(false),
    }
}

/// 获取Ollama可用的模型列表
pub async fn list_ollama_models(config: &Config) -> Result<Vec<String>> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/tags", config.ollama_url);

    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| AppError::ExternalApiError(format!("获取 Ollama 模型列表失败: {}", e)))?;

    #[derive(Deserialize)]
    struct ModelsResponse {
        models: Vec<ModelInfo>,
    }

    #[derive(Deserialize)]
    struct ModelInfo {
        name: String,
    }

    let models_response: ModelsResponse = response
        .json()
        .await
        .map_err(|e| AppError::ExternalApiError(format!("解析模型列表失败: {}", e)))?;

    Ok(models_response.models.into_iter().map(|m| m.name).collect())
}

// =============== 标签提取系统提示词 ===============

const TAG_EXTRACTION_PROMPT: &str = r#"你是一个专业的技术文档关键词提取专家。请从以下RFC文档的标题和摘要中生成3-8个最相关且具备可搜索指向性的技术标签（tags）。

严格要求：
1. 只输出3到8个标签，优先选择能明确指向协议、标准或核心技术概念的标签（越具体越好）。
2. 避免过于通用或描述性弱的词，例如："rfc", "document", "standard", "protocol", "paper", "technical"等（这些词不应作为标签）。
3. 标签应为小写英文，单词间用连字符连接，长度不要超过30字符，通常为1-3个词。
4. 优先包括：具体协议/规范名称（如：oauth2.0, tls1.3, jwt）、关键技术术语（如：token-validation, bearer-token）、以及明确的应用场景/问题域（如：api-authentication, web-security）。
5. 如有可能，避免仅使用非常宽泛的领域词（如：security、network），而应用更具体的子领域（如：http-security、tls-cipher-suites）。
6. 输出格式必须仅为以逗号分隔的标签列表，不要带任何说明或多余文字。
7. 如果标题/摘要中有版本号或标准编号（如 RFC 6749），可把版本或编号保留在标签中（如：oauth2.0, rfc6749）。

示例输出（严格遵守格式）：oauth2.0, authorization-code, access-token, bearer-token, api-authentication, token-refresh
"#;

/// 从RFC内容中提取标签
/// 
/// # 参数
/// * `config` - 配置信息
/// * `title` - RFC标题（英文）
/// * `abstract_text` - RFC摘要（英文）
/// * `chinese_title` - RFC中文标题（可选）
/// * `chinese_abstract` - RFC中文摘要（可选）
/// 
/// # 返回
/// 提取的标签列表
pub async fn extract_tags_from_content(
    config: &Config,
    title: &str,
    abstract_text: &str,
    chinese_title: Option<&str>,
    chinese_abstract: Option<&str>,
) -> Result<Vec<String>> {
    // 构建输入文本
    let mut input = format!("标题: {}\n摘要: {}", title, abstract_text);
    
    // 如果有中文内容，也加入分析
    if let Some(cn_title) = chinese_title {
        input.push_str(&format!("\n中文标题: {}", cn_title));
    }
    if let Some(cn_abstract) = chinese_abstract {
        input.push_str(&format!("\n中文摘要: {}", cn_abstract));
    }

    // 使用Ollama或OpenAI提取标签
    let tags_text = match config.ai_provider.as_str() {
        "ollama" => extract_tags_with_ollama(config, &input).await?,
        "openai" => extract_tags_with_openai(config, &input).await?,
        _ => return Err(AppError::InternalError(format!(
            "不支持的AI提供商: {}",
            config.ai_provider
        ))),
    };

    // 解析标签（去除空白、转小写、去重）
    let tags: Vec<String> = tags_text
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty() && s.len() <= 30) // 过滤空标签和过长的标签
        .collect::<std::collections::HashSet<_>>() // 去重
        .into_iter()
        .take(12) // 最多12个标签
        .collect();

    Ok(tags)
}

/// 使用Ollama提取标签
async fn extract_tags_with_ollama(config: &Config, content: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120)) // 总超时2分钟
        .build()
        .map_err(|e| AppError::ExternalApiError(format!("创建 HTTP 客户端失败: {}", e)))?;
    
    let url = format!("{}/api/chat", config.ollama_url);

    let request = OllamaChatRequest {
        model: config.ollama_model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: TAG_EXTRACTION_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: content.to_string(),
            },
        ],
        stream: false,
        options: OllamaOptions {
            temperature: 0.3,
            num_ctx: 4096,
        },
    };

    tracing::debug!("正在调用 Ollama API 提取标签: {}", url);

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                AppError::ExternalApiError("Ollama API 超时 (>120秒)，请检查 Ollama 服务状态".to_string())
            } else if e.is_connect() {
                AppError::ExternalApiError(format!("无法连接到 Ollama 服务 ({})", config.ollama_url))
            } else {
                AppError::ExternalApiError(format!("调用 Ollama API 失败: {}", e))
            }
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        return Err(AppError::ExternalApiError(format!(
            "Ollama API 返回错误 ({}): {}",
            status,
            error_text
        )));
    }

    let chat_response: OllamaChatResponse = response
        .json()
        .await
        .map_err(|e| AppError::ExternalApiError(format!("解析 Ollama API 响应失败: {}", e)))?;

    tracing::debug!("Ollama 返回的标签: {}", chat_response.message.content);
    Ok(chat_response.message.content)
}

/// 使用OpenAI提取标签
async fn extract_tags_with_openai(config: &Config, content: &str) -> Result<String> {
    let api_key = config
        .openai_api_key
        .as_ref()
        .ok_or_else(|| AppError::InternalError("未配置 OpenAI API Key".to_string()))?;

    let client = reqwest::Client::new();
    let url = format!("{}/chat/completions", config.openai_api_base);

    let request = OpenAIChatRequest {
        model: config.openai_model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: TAG_EXTRACTION_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: content.to_string(),
            },
        ],
        temperature: 0.3,
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| AppError::ExternalApiError(format!("调用 OpenAI API 失败: {}", e)))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        return Err(AppError::ExternalApiError(format!(
            "OpenAI API 返回错误: {}",
            error_text
        )));
    }

    let chat_response: OpenAIChatResponse = response
        .json()
        .await
        .map_err(|e| AppError::ExternalApiError(format!("解析 API 响应失败: {}", e)))?;

    let tags_text = chat_response
        .choices
        .first()
        .map(|choice| choice.message.content.clone())
        .ok_or_else(|| AppError::ExternalApiError("API 响应中没有标签结果".to_string()))?;

    Ok(tags_text)
}
