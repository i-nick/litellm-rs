//! Heroku Streaming Support
//!
//! Uses the unified SSE parser for consistent streaming across providers.
//! Heroku uses OpenAI-compatible SSE format.

use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;

use crate::core::providers::base::sse::{OpenAICompatibleTransformer, UnifiedSSEStream};

/// Heroku uses OpenAI-compatible SSE format
pub type HerokuStream = UnifiedSSEStream<
    Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    OpenAICompatibleTransformer,
>;

/// Helper function to create Heroku stream
pub fn create_heroku_stream(
    stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
) -> HerokuStream {
    let transformer = OpenAICompatibleTransformer::new("heroku");
    UnifiedSSEStream::new(Box::pin(stream), transformer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::providers::base::sse::UnifiedSSEParser;
    use futures::StreamExt;

    #[test]
    fn test_sse_parsing() {
        let transformer = OpenAICompatibleTransformer::new("heroku");
        let mut parser = UnifiedSSEParser::new(transformer);

        let test_data = b"data: {\"id\":\"chatcmpl-test\",\"object\":\"chat.completion.chunk\",\"created\":1640995200,\"model\":\"claude-4-5-sonnet\",\"choices\":[{\"index\":0,\"delta\":{\"role\":\"assistant\",\"content\":\"Hello\"},\"finish_reason\":null}]}\n\n";

        let result = parser.process_bytes(test_data).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].choices[0].delta.content,
            Some("Hello".to_string())
        );
    }

    #[test]
    fn test_done_message() {
        let transformer = OpenAICompatibleTransformer::new("heroku");
        let mut parser = UnifiedSSEParser::new(transformer);

        let test_data = b"data: [DONE]\n\n";
        let result = parser.process_bytes(test_data).unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_heroku_stream() {
        use futures::stream;

        let test_data = vec![
            Ok(Bytes::from(
                "data: {\"id\":\"test-1\",\"object\":\"chat.completion.chunk\",\"created\":1234567890,\"model\":\"claude-4-5-sonnet\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}]}\n\n",
            )),
            Ok(Bytes::from("data: [DONE]\n\n")),
        ];

        let mock_stream = stream::iter(test_data);
        let mut heroku_stream = create_heroku_stream(mock_stream);

        // First chunk
        let chunk1 = heroku_stream.next().await;
        assert!(chunk1.is_some());
        let chunk1 = chunk1.unwrap().unwrap();
        assert_eq!(chunk1.choices[0].delta.content.as_ref().unwrap(), "Hello");

        // Stream should end after [DONE]
        let end = heroku_stream.next().await;
        assert!(end.is_none());
    }

    #[tokio::test]
    async fn test_heroku_stream_multiple_chunks() {
        use futures::stream;

        let test_data = vec![
            Ok(Bytes::from(
                "data: {\"id\":\"test-1\",\"object\":\"chat.completion.chunk\",\"created\":1234567890,\"model\":\"claude-4-5-sonnet\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}]}\n\n",
            )),
            Ok(Bytes::from(
                "data: {\"id\":\"test-1\",\"object\":\"chat.completion.chunk\",\"created\":1234567890,\"model\":\"claude-4-5-sonnet\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\" from\"},\"finish_reason\":null}]}\n\n",
            )),
            Ok(Bytes::from(
                "data: {\"id\":\"test-1\",\"object\":\"chat.completion.chunk\",\"created\":1234567890,\"model\":\"claude-4-5-sonnet\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\" Heroku!\"},\"finish_reason\":null}]}\n\n",
            )),
            Ok(Bytes::from("data: [DONE]\n\n")),
        ];

        let mock_stream = stream::iter(test_data);
        let mut heroku_stream = create_heroku_stream(mock_stream);

        // First chunk
        let chunk1 = heroku_stream.next().await.unwrap().unwrap();
        assert_eq!(chunk1.choices[0].delta.content.as_ref().unwrap(), "Hello");

        // Second chunk
        let chunk2 = heroku_stream.next().await.unwrap().unwrap();
        assert_eq!(chunk2.choices[0].delta.content.as_ref().unwrap(), " from");

        // Third chunk
        let chunk3 = heroku_stream.next().await.unwrap().unwrap();
        assert_eq!(
            chunk3.choices[0].delta.content.as_ref().unwrap(),
            " Heroku!"
        );

        // Stream should end after [DONE]
        let end = heroku_stream.next().await;
        assert!(end.is_none());
    }

    #[tokio::test]
    async fn test_heroku_stream_claude_model() {
        use futures::stream;

        // Test with Claude model response format
        let test_data = vec![
            Ok(Bytes::from(
                "data: {\"id\":\"msg-heroku-123\",\"object\":\"chat.completion.chunk\",\"created\":1234567890,\"model\":\"claude-4-5-sonnet\",\"choices\":[{\"index\":0,\"delta\":{\"role\":\"assistant\"},\"finish_reason\":null}]}\n\n",
            )),
            Ok(Bytes::from(
                "data: {\"id\":\"msg-heroku-123\",\"object\":\"chat.completion.chunk\",\"created\":1234567890,\"model\":\"claude-4-5-sonnet\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"I'm Claude via Heroku!\"},\"finish_reason\":null}]}\n\n",
            )),
            Ok(Bytes::from(
                "data: {\"id\":\"msg-heroku-123\",\"object\":\"chat.completion.chunk\",\"created\":1234567890,\"model\":\"claude-4-5-sonnet\",\"choices\":[{\"index\":0,\"delta\":{},\"finish_reason\":\"stop\"}]}\n\n",
            )),
            Ok(Bytes::from("data: [DONE]\n\n")),
        ];

        let mock_stream = stream::iter(test_data);
        let mut heroku_stream = create_heroku_stream(mock_stream);

        let mut collected_content = String::new();
        while let Some(chunk_result) = heroku_stream.next().await {
            let chunk = chunk_result.unwrap();
            if let Some(content) = &chunk.choices[0].delta.content {
                collected_content.push_str(content);
            }
        }

        assert_eq!(collected_content, "I'm Claude via Heroku!");
    }

    #[tokio::test]
    async fn test_heroku_stream_amazon_nova_model() {
        use futures::stream;

        // Test with Amazon Nova model response format
        let test_data = vec![
            Ok(Bytes::from(
                "data: {\"id\":\"nova-123\",\"object\":\"chat.completion.chunk\",\"created\":1234567890,\"model\":\"amazon-nova-pro\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"Hello from Nova!\"},\"finish_reason\":null}]}\n\n",
            )),
            Ok(Bytes::from("data: [DONE]\n\n")),
        ];

        let mock_stream = stream::iter(test_data);
        let mut heroku_stream = create_heroku_stream(mock_stream);

        let chunk = heroku_stream.next().await.unwrap().unwrap();
        assert_eq!(chunk.model, "amazon-nova-pro");
        assert_eq!(
            chunk.choices[0].delta.content.as_ref().unwrap(),
            "Hello from Nova!"
        );
    }
}
