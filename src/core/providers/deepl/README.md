# DeepL Translation Provider

This provider integrates DeepL's translation API into the LiteLLM-RS gateway.

## Overview

DeepL is a neural machine translation service that provides high-quality translations. This provider adapts DeepL's translation API to work with the LLMProvider interface by treating translation requests as chat completions.

## Features

- ✅ Text translation (30+ languages)
- ✅ Auto-detect source language
- ✅ Explicit source language specification
- ✅ Free and Pro API support
- ✅ Health checks via usage endpoint
- ✅ Cost calculation
- ❌ Streaming (not supported by DeepL)
- ❌ Chat completion (translation only)

## Configuration

### Environment Variables

```bash
export DEEPL_API_KEY="your-api-key-here"
```

### API Endpoints

- **Free API**: `https://api-free.deepl.com/v2` (default)
- **Pro API**: `https://api.deepl.com/v2`

### Usage Example

```rust
use litellm_rs::core::providers::deepl::{DeepLConfig, DeepLProvider};
use litellm_rs::core::traits::provider::llm_provider::trait_definition::LLMProvider;

// Create configuration
let config = DeepLConfig::from_env()?;
let provider = DeepLProvider::new(config)?;

// Or use Pro API
let config = DeepLConfig::from_env()?.with_pro(true);
let provider = DeepLProvider::new(config)?;
```

## Translation Request Format

The provider expects messages in a specific format:

### Simple Translation (auto-detect source)
```
Translate to {TARGET_LANG}: {text to translate}
```

Example:
```
Translate to DE: Hello, how are you?
```

### Translation with Source Language
```
Translate from {SOURCE_LANG} to {TARGET_LANG}: {text to translate}
```

Example:
```
Translate from EN to FR: Hello, how are you?
```

## Supported Languages

DeepL supports 30+ languages including:

**Target Languages:**
- BG (Bulgarian)
- CS (Czech)
- DA (Danish)
- DE (German)
- EL (Greek)
- EN (English) - EN-GB, EN-US
- ES (Spanish)
- ET (Estonian)
- FI (Finnish)
- FR (French)
- HU (Hungarian)
- ID (Indonesian)
- IT (Italian)
- JA (Japanese)
- KO (Korean)
- LT (Lithuanian)
- LV (Latvian)
- NB (Norwegian)
- NL (Dutch)
- PL (Polish)
- PT (Portuguese) - PT-BR, PT-PT
- RO (Romanian)
- RU (Russian)
- SK (Slovak)
- SL (Slovenian)
- SV (Swedish)
- TR (Turkish)
- UK (Ukrainian)
- ZH (Chinese)

## Model Information

### deepl-translate

- **Provider**: deepl
- **Max Context**: 50,000 characters
- **Streaming**: No
- **Capabilities**: AudioTranslation (translation service)
- **Cost**: ~$0.00002 per 1K tokens (approximate)

## API Integration

The provider maps DeepL's translation API to the chat completion interface:

1. **Request Transformation**: Extracts translation parameters from message content
2. **API Call**: Calls DeepL's `/translate` endpoint
3. **Response Transformation**: Converts translation response to chat completion format

## Error Handling

DeepL-specific error codes:

- **401/403**: Authentication failed (invalid API key)
- **429**: Rate limit exceeded
- **456**: Quota exceeded (DeepL-specific)
- **400**: Invalid request (bad language code, etc.)

## Health Check

The provider uses DeepL's `/usage` endpoint for health checks.

## Cost Calculation

DeepL pricing is character-based. The provider approximates:
- Input cost: $0.00002 per 1K tokens
- Output cost: $0.00002 per 1K tokens

## Example Usage

See `/examples/deepl_translation.rs` for a complete example.

```bash
export DEEPL_API_KEY="your-key"
cargo run --example deepl_translation --all-features
```

## Limitations

1. **Not a Chat Model**: DeepL only supports translation, not general conversation
2. **No Streaming**: Translation is returned as a complete response
3. **Format Required**: Messages must follow the translation format
4. **Character Limit**: 50KB per request

## Implementation Details

- **Config**: `DeepLConfig` with free/pro API selection
- **Provider**: `DeepLProvider` implementing `LLMProvider` trait
- **Error Mapper**: `DeepLErrorMapper` for DeepL-specific errors
- **Models**: Single model `deepl-translate`

## References

- [DeepL API Documentation](https://www.deepl.com/docs-api)
- [DeepL Pricing](https://www.deepl.com/pro-api)
- [Language Codes](https://www.deepl.com/docs-api/translating-text)
