//! Module for retrieving article statistics via the STAT command.
//!
//! Functions:
//! - `stat()`: sends `STAT <id>` and returns the message ID from the "223" response.
use crate::auth::AuthenticatedConnection;
use std::error::Error;
use tokio::io::{split, AsyncWriteExt, BufReader, BufWriter};

/// Sends the STAT command using the specified article identifier and returns the associated message ID.
/// The STAT command is expected to respond with a line starting with "223", followed by the message ID.
pub async fn stat(
    connection: &mut AuthenticatedConnection,
    identifier: &str,
) -> Result<String, Box<dyn Error>> {
    let (read_half, write_half) = split(&mut connection.tls_stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    // Construct and send the STAT command.
    let command = format!("STAT {}\r\n", identifier);
    writer.write_all(command.as_bytes()).await?;
    writer.flush().await?;

    // Wait for the response; expected code is "223".
    let response = crate::utils::wait_for_response(&mut reader, &["223"], 5, 3).await?;
    if !response.starts_with("223") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "STAT command failed: unexpected response",
        )));
    }

    // Expected format is "223 <message-id>".
    let parts: Vec<&str> = response.split_whitespace().collect();
    if parts.len() >= 2 {
        Ok(parts[1].to_string())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "STAT command failed: invalid response format",
        )))
    }
}
