//! Empower Streaming Support

use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;

use crate::core::providers::base::sse::{OpenAICompatibleTransformer, UnifiedSSEStream};

pub type EmpowerStream = UnifiedSSEStream<
    Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    OpenAICompatibleTransformer,
>;

pub fn create_empower_stream(
    stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
) -> EmpowerStream {
    let transformer = OpenAICompatibleTransformer::new("empower");
    UnifiedSSEStream::new(Box::pin(stream), transformer)
}
