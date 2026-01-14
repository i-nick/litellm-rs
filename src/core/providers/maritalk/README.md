# Maritalk Provider

Maritalk is a Brazilian AI provider specializing in Portuguese language models, particularly the Sabiá family of models.

## Features

- **Brazilian Portuguese Focus**: Models optimized for Portuguese language understanding and generation
- **OpenAI-Compatible API**: Standard chat completion interface
- **Cost Calculation**: Built-in token counting and cost estimation
- **Health Monitoring**: Provider health checks

## Supported Models

| Model | Context Length | Max Output | Input Cost (per 1K tokens) | Output Cost (per 1K tokens) | Tools Support |
|-------|---------------|------------|---------------------------|----------------------------|---------------|
| sabia-2-medium | 8,192 | 4,096 | $0.00002 | $0.00004 | ✓ |
| sabia-2-small | 4,096 | 2,048 | $0.00001 | $0.00002 | ✗ |

## Configuration

### Environment Variable

```bash
export MARITALK_API_KEY="your-api-key-here"
```

### Code Configuration

```rust
use litellm_rs::core::providers::maritalk::{MaritalkConfig, MaritalkProvider};

// From environment variable
let provider = MaritalkProvider::new(MaritalkConfig::from_env()?)?;

// With explicit API key
let config = MaritalkConfig::new("your-api-key");
let provider = MaritalkProvider::new(config)?;

// With custom settings
let config = MaritalkConfig::new("your-api-key")
    .with_base_url("https://custom.api.com")
    .with_timeout(120);
let provider = MaritalkProvider::new(config)?;
```

## Usage Example

```rust
use litellm_rs::core::providers::maritalk::MaritalkProvider;
use litellm_rs::core::types::{
    requests::ChatRequest,
    common::RequestContext,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = MaritalkProvider::with_api_key("your-api-key").await?;

    let request = ChatRequest {
        model: "sabia-2-medium".to_string(),
        messages: vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Olá! Como você está?".to_string(),
                ..Default::default()
            }
        ],
        temperature: Some(0.7),
        max_tokens: Some(1000),
        ..Default::default()
    };

    let context = RequestContext::default();
    let response = provider.chat_completion(request, context).await?;

    println!("Response: {}", response.choices[0].message.content);
    Ok(())
}
```

## Supported Parameters

The provider supports the following OpenAI-compatible parameters:

- `temperature`: Controls randomness (0.0 to 2.0)
- `max_tokens`: Maximum tokens to generate
- `top_p`: Nucleus sampling parameter
- `stream`: Enable streaming responses
- `stop`: Stop sequences

## API Details

- **Base URL**: `https://chat.maritaca.ai/api`
- **Authentication**: API Key header (`Authorization: Key YOUR_API_KEY`)
- **Format**: OpenAI-compatible JSON

## Testing

```bash
# Run provider tests
cargo test --package litellm-rs --lib core::providers::maritalk --all-features

# Run specific test
cargo test maritalk::test_provider_creation --lib --all-features
```

## Cost Calculation

```rust
let cost = provider.calculate_cost("sabia-2-medium", 1000, 1000).await?;
println!("Cost for 1K input + 1K output tokens: ${:.5}", cost);
```

## Error Handling

The provider uses the unified `ProviderError` type:

- `Authentication`: Invalid or missing API key (401)
- `RateLimit`: Rate limit exceeded (429)
- `ModelNotFound`: Model not available (404)
- `InvalidRequest`: Malformed request (400)
- `ApiError`: Other API errors

## References

- [Maritalk API Documentation](https://chat.maritaca.ai/docs)
- [Sabiá Models](https://www.maritaca.ai/)
