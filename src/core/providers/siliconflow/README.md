# SiliconFlow Provider

SiliconFlow provider implementation for LiteLLM-RS gateway.

## Overview

SiliconFlow (https://siliconflow.cn) is an AI inference platform providing access to various open-source models through an OpenAI-compatible API.

## Configuration

### Environment Variable
```bash
export SILICONFLOW_API_KEY="your-api-key-here"
```

### API Details
- **Base URL**: `https://api.siliconflow.cn/v1`
- **Authentication**: Bearer token (API Key)
- **API Format**: OpenAI-compatible

## Supported Models

1. **DeepSeek-V2.5** (`deepseek-ai/DeepSeek-V2.5`)
   - Context Length: 32,768 tokens
   - Max Output: 8,192 tokens
   - Cost: $0.14/M input tokens, $0.28/M output tokens

2. **Qwen2.5-72B-Instruct** (`Qwen/Qwen2.5-72B-Instruct`)
   - Context Length: 32,768 tokens
   - Max Output: 8,192 tokens
   - Cost: $0.56/M input tokens, $0.56/M output tokens

3. **Qwen2.5-Coder-32B-Instruct Pro** (`Pro/Qwen/Qwen2.5-Coder-32B-Instruct`)
   - Context Length: 32,768 tokens
   - Max Output: 8,192 tokens
   - Cost: $0.42/M input tokens, $0.42/M output tokens

## Usage Examples

### Basic Usage

```rust
use litellm_rs::core::providers::siliconflow::{SiliconFlowConfig, SiliconFlowProvider};
use litellm_rs::core::traits::provider::llm_provider::trait_definition::LLMProvider;
use litellm_rs::core::types::{
    common::RequestContext,
    requests::{ChatRequest, Message},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider from environment variable
    let config = SiliconFlowConfig::from_env()?;
    let provider = SiliconFlowProvider::new(config)?;

    // Create chat request
    let request = ChatRequest {
        model: "deepseek-ai/DeepSeek-V2.5".to_string(),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: "Explain quantum computing in simple terms".to_string(),
                ..Default::default()
            }
        ],
        temperature: Some(0.7),
        max_tokens: Some(1000),
        ..Default::default()
    };

    // Execute chat completion
    let response = provider.chat_completion(request, RequestContext::default()).await?;
    println!("Response: {:?}", response);

    Ok(())
}
```

### With Custom Configuration

```rust
let config = SiliconFlowConfig::new("your-api-key")
    .with_base_url("https://api.siliconflow.cn/v1")
    .with_timeout(120);

let provider = SiliconFlowProvider::new(config)?;
```

### Using the `with_api_key` Helper

```rust
let provider = SiliconFlowProvider::with_api_key("your-api-key").await?;
```

### Listing Models

```rust
let models = provider.models();
for model in models {
    println!("Model ID: {}", model.id);
    println!("  Name: {}", model.name);
    println!("  Context: {} tokens", model.max_context_length);
    println!("  Cost: ${:.6}/1k input, ${:.6}/1k output",
        model.input_cost_per_1k_tokens.unwrap_or(0.0),
        model.output_cost_per_1k_tokens.unwrap_or(0.0)
    );
}
```

### Cost Calculation

```rust
// Calculate cost for 1000 input tokens and 500 output tokens
let cost = provider.calculate_cost(
    "deepseek-ai/DeepSeek-V2.5",
    1000,
    500
).await?;
println!("Estimated cost: ${:.6}", cost);
```

## Supported OpenAI Parameters

The provider supports the following OpenAI-compatible parameters:
- `temperature` - Controls randomness (0.0 to 2.0)
- `max_tokens` - Maximum tokens to generate
- `top_p` - Nucleus sampling parameter
- `stream` - Enable streaming responses
- `stop` - Stop sequences
- `frequency_penalty` - Penalize frequent tokens
- `presence_penalty` - Penalize tokens based on presence

## Capabilities

- ✅ Chat Completion
- ✅ Chat Completion Stream (planned)
- ✅ Cost Calculation
- ✅ Health Check
- ✅ Model Listing

## Architecture

### Files

- `mod.rs` - Module exports and constants
- `config.rs` - Configuration struct with builder pattern
- `provider.rs` - Provider implementation of LLMProvider trait
- `model_info.rs` - Supported models and metadata
- `error_mapper.rs` - HTTP error mapping to ProviderError

### Error Handling

The provider uses the unified `ProviderError` type:
- `401` → Authentication error
- `429` → Rate limit error
- `404` → Model not found
- `400` → Invalid request
- Other → Generic API error

## Testing

```bash
# Run all siliconflow tests
cargo test --lib siliconflow --all-features

# Run specific tests
cargo test --lib siliconflow::config::tests
cargo test --lib siliconflow::provider::tests
```

## Configuration Examples

### Via Gateway Config (YAML)

```yaml
providers:
  - name: siliconflow
    type: siliconflow
    api_key: ${SILICONFLOW_API_KEY}
    timeout: 60
    max_retries: 3
```

### Via Code

```rust
use std::collections::HashMap;

let mut config = SiliconFlowConfig::default();
config.base.api_key = Some("your-key".to_string());
config.base.timeout = 120;
config.base.max_retries = 3;

let provider = SiliconFlowProvider::new(config)?;
```

## Notes

- SiliconFlow API is OpenAI-compatible, making integration straightforward
- Streaming support is planned but not yet implemented
- All models support function calling and tools
- Pricing is significantly lower than OpenAI for comparable models

## Contributing

When adding new models:
1. Update `model_info.rs` with model details
2. Add pricing information from SiliconFlow documentation
3. Update this README with the new model
4. Add tests for the new model

## References

- [SiliconFlow Official Website](https://siliconflow.cn)
- [SiliconFlow API Documentation](https://docs.siliconflow.cn)
- [Provider Development Guide](../../../CLAUDE.md#provider-development)
