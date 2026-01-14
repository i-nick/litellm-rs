# Qwen Provider

Qwen (Alibaba Tongyi Qianwen) provider integration for LiteLLM-RS.

## Configuration

### Environment Variables

- `QWEN_API_KEY` or `DASHSCOPE_API_KEY`: API key for Qwen
- `QWEN_API_BASE`: API base URL (default: https://dashscope.aliyuncs.com/api/v1)

### Usage

```rust
use litellm_rs::core::providers::qwen::{QwenConfig, QwenProvider};
use litellm_rs::core::traits::ProviderConfig;

// From environment
let config = QwenConfig::from_env()?;
let provider = QwenProvider::new(config)?;

// With explicit API key
let config = QwenConfig {
    api_key: Some("your-api-key".to_string()),
    ..Default::default()
};
let provider = QwenProvider::new(config)?;
```

## Supported Models

- **qwen-turbo**: Fast and efficient model for general tasks (8K context)
- **qwen-plus**: Balanced model with enhanced capabilities (32K context)
- **qwen-max**: Most capable model for complex tasks (8K context)
- **qwen-max-longcontext**: Maximum capability with extended context (30K context)

## Capabilities

- Chat Completion
- Streaming Chat Completion
- Embeddings

## API Documentation

For more information about Qwen API, visit:
https://help.aliyun.com/zh/dashscope/
