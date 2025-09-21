# My Async Logger

Async logging library with batch processing and graceful shutdown.

## Features

- Async logging with tokio
- Batch processing for efficiency
- Graceful shutdown
- Timezone support
- Error handling

## Usage

```rust
use my_async_logger::{LoggerHandle, LoggerConfig};

#[tokio::main]
async fn main() {
    let config = LoggerConfig::default();
    let logger = LoggerHandle::new(config).await.unwrap();
    logger.log("Hello world!".to_string()).await.unwrap();
}
