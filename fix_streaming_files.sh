#!/bin/bash

for PROVIDER in datarobot docker_model_runner empower exa_ai featherless firecrawl; do
    UPPER=$(echo $PROVIDER | sed 's/_/ /g' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) tolower(substr($i,2))}1' | sed 's/ //g')
    
    cat > "src/core/providers/${PROVIDER}/streaming.rs" << EOF
//! ${UPPER} Streaming Support

use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;

use crate::core::providers::base::sse::{OpenAICompatibleTransformer, UnifiedSSEStream};

pub type ${UPPER}Stream = UnifiedSSEStream<
    Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    OpenAICompatibleTransformer,
>;

pub fn create_${PROVIDER}_stream(
    stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
) -> ${UPPER}Stream {
    let transformer = OpenAICompatibleTransformer::new("${PROVIDER}");
    UnifiedSSEStream::new(Box::pin(stream), transformer)
}
EOF
    
    echo "Fixed streaming for ${PROVIDER}"
done

echo "All streaming files fixed!"
