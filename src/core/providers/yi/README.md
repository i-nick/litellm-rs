# Yi (01.AI) Provider

Implementation of the Yi (01.AI) provider for LiteLLM-RS.

## Overview

Yi is a series of large language models developed by 01.AI, offering various models with different capabilities and price points.

## Configuration

### Environment Variables

- `YI_API_KEY` - Your Yi API key (required)

### Configuration Example

```rust
use litellm_rs::core::providers::yi::{YiConfig, YiProvider};

// Create from environment variable
let provider = YiConfig::from_env()
    .and_then(|config| YiProvider::new(config))?;

// Create with explicit API key
let config = YiConfig::new("your-api-key-here");
let provider = YiProvider::new(config)?;

// With custom settings
let config = YiConfig::new("your-api-key-here")
    .with_base_url("https://api.lingyiwanwu.com/v1")
    .with_timeout(120);
let provider = YiProvider::new(config)?;
```

## Supported Models

| Model ID | Name | Context Length | Capabilities |
|----------|------|----------------|--------------|
| `yi-large` | Yi-Large | 32,768 | Chat, Tools |
| `yi-large-turbo` | Yi-Large-Turbo | 16,384 | Chat, Tools |
| `yi-medium` | Yi-Medium | 16,384 | Chat, Tools |
| `yi-spark` | Yi-Spark | 16,384 | Chat, Tools |
| `yi-vision` | Yi-Vision | 16,384 | Chat, Tools, Multimodal |

## Pricing

- **yi-large**: $0.003/1K input tokens, $0.012/1K output tokens
- **yi-large-turbo**: $0.0012/1K input tokens, $0.0012/1K output tokens
- **yi-medium**: $0.00025/1K input tokens, $0.00025/1K output tokens
- **yi-spark**: $0.0001/1K input tokens, $0.0001/1K output tokens
- **yi-vision**: $0.0006/1K input tokens, $0.0006/1K output tokens

## Features

- OpenAI-compatible API
- Unified error handling with `ProviderError`
- Support for streaming (planned)
- Support for function calling/tools
- Multimodal capabilities (yi-vision)

## API Compatibility

The Yi provider uses an OpenAI-compatible API, supporting the following parameters:

- `temperature`
- `max_tokens`
- `top_p`
- `stream`
- `stop`
- `frequency_penalty`
- `presence_penalty`

## Usage Example

```rust
use litellm_rs::core::providers::yi::{YiConfig, YiProvider};
use litellm_rs::core::types::{
    requests::ChatRequest,
    common::{Message, RequestContext},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize provider
    let config = YiConfig::from_env()?;
    let provider = YiProvider::new(config)?;

    // Create a chat request
    let request = ChatRequest {
        model: "yi-large".to_string(),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: "Hello! How are you?".to_string(),
                ..Default::default()
            }
        ],
        temperature: Some(0.7),
        max_tokens: Some(1000),
        ..Default::default()
    };

    // Send request
    let context = RequestContext::default();
    let response = provider.chat_completion(request, context).await?;

    println!("Response: {:?}", response);

    Ok(())
}
```

## Error Handling

The Yi provider uses the unified `ProviderError` type with specific error mappings:

- `401` - Authentication error (invalid API key)
- `429` - Rate limit exceeded
- `404` - Model not found
- `400` - Invalid request

## API Endpoint

Default base URL: `https://api.lingyiwanwu.com/v1`

## Testing

```bash
# Run Yi provider tests
cargo test --package litellm-rs --lib core::providers::yi --all-features

# Run specific test
cargo test --package litellm-rs --lib yi::tests::test_provider_creation
```

## References

- [Yi API Documentation](https://platform.lingyiwanwu.com/docs)
- [01.AI Official Website](https://01.ai/)
