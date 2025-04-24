//! Module for selecting a newsgroup via the GROUP command.
//!
//! Functions:
//! - `group()`: sends `GROUP <name>` and returns the server’s response line starting with "211".
use crate::auth::AuthenticatedConnection;
use std::error::Error;
use tokio::io::{split, AsyncWriteExt, BufReader, BufWriter};

/// Sends the GROUP command to select a newsgroup and returns the server's response.
/// The command response is expected to begin with "211".
pub async fn group(
    connection: &mut AuthenticatedConnection,
    group_name: &str,
) -> Result<String, Box<dyn Error>> {
    let (read_half, write_half) = split(&mut connection.tls_stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    let command = format!("GROUP {}\r\n", group_name);
    writer.write_all(command.as_bytes()).await?;
    writer.flush().await?;

    let response = crate::utils::wait_for_response(&mut reader, &["211"], 5, 3).await?;
    Ok(response.trim().to_string())
}
