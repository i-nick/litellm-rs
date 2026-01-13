//! Featherless Streaming Support

use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;

use crate::core::providers::base::sse::{OpenAICompatibleTransformer, UnifiedSSEStream};

pub type FeatherlessStream = UnifiedSSEStream<
    Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    OpenAICompatibleTransformer,
>;

pub fn create_featherless_stream(
    stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
) -> FeatherlessStream {
    let transformer = OpenAICompatibleTransformer::new("featherless");
    UnifiedSSEStream::new(Box::pin(stream), transformer)
}
