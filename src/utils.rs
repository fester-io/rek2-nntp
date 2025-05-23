//! Utility functions for NNTP I/O handling.
//!
//! Functions:
//! - `read_line_with_timeout()`: reads one line with a timeout.
//! - `wait_for_response()`: loops reading until a line matches expected prefixes.
use std::error::Error;
use tokio::io::AsyncBufReadExt;
use tokio::time::{timeout, Duration};

pub async fn read_line_with_timeout<R>(
    reader: &mut R,
    timeout_secs: u64,
) -> Result<String, Box<dyn Error>>
where
    R: AsyncBufReadExt + Unpin,
{
    // Buffer for the raw bytes
    let mut buf = Vec::<u8>::new();

    // Read until we hit '\n' (inclusive) or the timeout fires
    timeout(
        Duration::from_secs(timeout_secs),
        reader.read_until(b'\n', &mut buf),
    )
    .await??;

    // Convert to String, replacing any invalid UTF-8 with the Unicode replacement char ()
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

pub async fn wait_for_response<R>(
    reader: &mut R,
    expected_prefixes: &[&str],
    timeout_secs: u64,
    max_attempts: u8,
) -> Result<String, Box<dyn Error>>
where
    R: AsyncBufReadExt + Unpin,
{
    for _attempt in 0..max_attempts {
        let response = read_line_with_timeout(reader, timeout_secs).await?;
        if expected_prefixes
            .iter()
            .any(|&prefix| response.starts_with(prefix))
        {
            return Ok(response);
        }
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::TimedOut,
        "Did not receive an expected response within the timeout period",
    )))
}