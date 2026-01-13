//! DockerModelRunner Streaming Support

use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;

use crate::core::providers::base::sse::{OpenAICompatibleTransformer, UnifiedSSEStream};

pub type DockerModelRunnerStream = UnifiedSSEStream<
    Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    OpenAICompatibleTransformer,
>;

pub fn create_docker_model_runner_stream(
    stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
) -> DockerModelRunnerStream {
    let transformer = OpenAICompatibleTransformer::new("docker_model_runner");
    UnifiedSSEStream::new(Box::pin(stream), transformer)
}
