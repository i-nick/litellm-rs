//! Firecrawl Streaming Support

use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;

use crate::core::providers::base::sse::{OpenAICompatibleTransformer, UnifiedSSEStream};

pub type FirecrawlStream = UnifiedSSEStream<
    Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    OpenAICompatibleTransformer,
>;

pub fn create_firecrawl_stream(
    stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
) -> FirecrawlStream {
    let transformer = OpenAICompatibleTransformer::new("firecrawl");
    UnifiedSSEStream::new(Box::pin(stream), transformer)
}
